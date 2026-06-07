//! The "Game" layer of Wordle represents the top-level of interaction with library consumers.
//! Library consumers only understand strings and colors, not internal types, so none of them
//! escape the API boundary.
//!
//! The primary type here is [`Wordle`], which holds the global word list, and can be used to spawn
//! [`Round`]s, the primary method of gameplay.
#[cfg(test)]
mod tests;

mod error;
mod letter;
pub mod round;
mod space;
mod word;

use std::collections::HashSet;

use crate::client::Client;
pub(crate) use crate::game::error::Error;
use crate::game::error::{
    Result,
    State,
    WordExt,
};
use crate::game::round::Round;
use crate::game::round::score::ScoreMode;
use crate::game::space::WordSpace;
use crate::game::word::Word;

/// [`Wordle`] holds the list of all possible solutions, and is used to spawn [`Round`]s, which are
/// the entrypoint for gameplay.
///
/// The global word list for the official Wordle game is expected to update _at most_ once per day,
/// so consumers will likely want to hold on to only one of these at a time throughout the duration
/// of their application's lifetime.
///
/// # Performance
///
/// The words in the word list are stored in an internal fixed-width representation, which allows
/// the data to be stored in a contiguous block of memory.
///
/// Creating a [`Round`] by any method will make a full copy of the word list of which the [`Round`]
/// will take ownership.
///
/// The most expensive component of [`Wordle`] is initializing it. It accepts an [`IntoIterator`] of
/// any type that can be viewed as a [`&str`]. The strings are then processed as follows:
///
///   1. Whitespace trimmed
///   2. Case normalized
///   3. Length ([`WORD_LENGTH`]) and ASCII coherence verified
///
/// Processed words are then de-duplicated, and the values from the iterator are discarded.
///
/// # Usage
///
/// Users can create custom games with local resources using [`Wordle::from_list`],
/// [`Wordle::round`], and [`Wordle::play`]. They can also use a client to fetch the global word
/// list plus daily answers from an upstream resource by implementing [`Client`].
///
/// # Examples
///
/// ## Make a game from local resources
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use wordle::{
///     Color,
///     WORD_LENGTH,
///     Wordle,
/// };
///
/// let word_list = vec!["Hello", "VALID", "crunk"];
/// let answer = "valid";
///
/// let game = Wordle::from_list(word_list)?;
///
/// let mut round = game.round(answer)?;
///
/// let colors = round.guess(answer)?;
///
/// assert_eq!(colors, [Color::Green; WORD_LENGTH]);
///
/// // You can also have the game play itself all the way through
/// let guesses = game.play(answer)?.into_iter().collect::<Vec<_>>();
/// println!("{guesses:#?}");
///
/// # Ok(())
/// # }
/// ```
///
/// ## Make a round from online resources for interactive gameplay
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::error::Error;
/// use std::fmt::{
///     Display,
///     Formatter,
/// };
///
/// use wordle::{
///     Client,
///     Color,
///     WORD_LENGTH,
///     Wordle,
/// };
///
/// // This is just to make the example work. It's likely you won't need to define a custom error
/// // type just to implement Client.
/// #[derive(Debug)]
/// enum MyNetworkError {
///     InvalidDate { date: usize },
/// }
///
/// impl Display for MyNetworkError {
///     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
///         write!(f, "{self:?}")
///     }
/// }
///
/// impl Error for MyNetworkError {}
///
/// // This crate provides no implementation of its network Client
/// struct MyWordleClient {
///     word_list: Vec<String>,
/// }
///
/// impl Client for MyWordleClient {
///     type Date = usize;
///     type Error = MyNetworkError;
///
///     fn fetch_answer(&self, date: Self::Date) -> Result<String, Self::Error> {
///         if date >= self.word_list.len() {
///             return Err(Self::Error::InvalidDate { date });
///         };
///
///         Ok(self.word_list[date].clone())
///     }
///
///     fn fetch_words(&self) -> Result<impl Iterator<Item = String> + 'static, Self::Error> {
///         Ok(self.word_list.clone().into_iter())
///     }
/// }
///
/// let client = MyWordleClient {
///     word_list: ["Hello", "VALID", "crunk"].map(String::from).to_vec(),
/// };
///
/// let game = Wordle::from_client(&client)?;
///
/// let mut round = game.round_date(&client, 1)?;
///
/// let colors = round.guess("VALID")?;
///
/// assert_eq!(colors, [Color::Green; WORD_LENGTH]);
///
/// // You can also have the game play itself all the way through
///
/// let guesses = game.play_date(&client, 1)?.into_iter().collect::<Vec<_>>();
///
/// println!("{guesses:#?}");
///
/// # Ok(())
/// # }
/// ```
///
/// [`WORD_LENGTH`]: crate::WORD_LENGTH
#[derive(Debug)]
pub struct Wordle {
    words: WordSpace,
}

impl Wordle {
    /// Creates a new [`Wordle`] based on a word list in memory.
    ///
    /// See [`Wordle#performance`] for more on how the strings passed to this function are
    /// processed.
    ///
    /// # Errors
    ///
    /// This function will immediately return a [`wordle::Error::Fatal`] if any words are invalid
    /// (length, non-ASCII).
    ///
    /// [`wordle::Error::Fatal`]: crate::Error::Fatal
    pub fn from_list<I, S>(word_list: I) -> crate::Result<Self>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>, {
        Ok(Self {
            words: word_list
                .into_iter()
                .map(|s| Word::from_internal(s.as_ref()))
                .collect::<Result<HashSet<Word>>>()?
                .into_iter()
                .collect(),
        })
    }

    /// Uses a provided [`Client`] to resolve a word list,
    /// then creates a new [`Wordle`] from that list.
    ///
    /// For more on how the words are processed once fetched via the passed [`Client`], see
    /// [`Wordle#performance`].
    ///
    /// # Errors
    ///
    /// This method will return an error under the following conditions:
    ///
    ///   - The passed client has an error fetching the word list -> [`wordle::Error::Safe`]
    ///   - Any of the words from the list are not valid (length, non-ASCII)
    ///     -> [`wordle::Error::Fatal`]
    ///
    /// [`wordle::Error::Safe`]: crate::Error::Safe
    /// [`wordle::Error::Fatal`]: crate::Error::Fatal
    pub fn from_client(client: &impl Client) -> crate::Result<Self> {
        let word_list = client.fetch_words().map_err(Error::network)?;
        Self::from_list(word_list)
    }

    /// Plays a round of Wordle from beginning to end. This is the non-interactive gameplay method
    /// for a locally-derived answer. The caller will only receive back the list of guesses made
    /// by the crate. To play using the equivalent interactive method, see [`Wordle::round`].
    ///
    /// # Errors
    ///
    /// This method will return an error under the following conditions:
    ///
    ///   - The passed answer is not a valid word (length, non-ASCII) -> [`wordle::Error::Fatal`]
    ///   - The [`Wordle`] encounters an unexpected invalid state -> [`wordle::Error::Fatal`]
    ///
    /// # Return
    ///
    /// If the round is played successfully, all of the guesses made by the program will be
    /// returned. This will occur (and be [`Ok`]) even if the crate did not guess the correct
    /// answer.
    ///
    /// [`wordle::Error::Safe`]: crate::Error::Safe
    /// [`wordle::Error::Fatal`]: crate::Error::Fatal
    pub fn play(&self, answer: impl AsRef<str>) -> crate::Result<impl IntoIterator<Item = String>> {
        let mut round = Round::new(self.words.clone(), Word::from_internal(answer.as_ref())?);
        let mut score_mode = ScoreMode::UniqueOnly { n: 1 };

        while !round.is_over() {
            let next_guess =
                round
                    .best(score_mode)
                    .into_iter()
                    .next()
                    .ok_or(Error::InvalidState {
                        state: State::OutOfWords,
                    })?;
            let word = next_guess.word_data();
            let colors = round.evaluate(word);
            round.commit(word, colors);
            score_mode = ScoreMode::Default { n: 1 };
        }

        Ok(round.into_guesses())
    }

    /// Fetches the Wordle answer for a given date, and then calls [`Wordle::play`] with that
    /// answer. This is the non-interactive gameplay method for a network-derived answer. The caller
    /// will only receive back the list of guesses made by the crate. To play using the equivalent
    /// interactive method, see [`Wordle::round_date`].
    ///
    /// # Errors
    ///
    /// This method will return an error under the following conditions:
    ///
    ///   - The client encounters an error when resolving the answer -> [`wordle::Error::Safe`]
    ///   - The passed answer is not a valid word (length, non-ASCII) -> [`wordle::Error::Fatal`]
    ///   - The [`Wordle`] encounters an unexpected invalid state -> [`wordle::Error::Fatal`]
    ///
    /// # Return
    ///
    /// If the round is played successfully, all of the guesses made by the program will be
    /// returned. This will occur (and be [`Ok`]) even if the crate did not guess the correct
    /// answer.
    ///
    /// [`wordle::Error::Safe`]: crate::Error::Safe
    /// [`wordle::Error::Fatal`]: crate::Error::Fatal
    pub fn play_date<D>(
        &self,
        resource_client: &impl Client<Date = D>,
        date: D,
    ) -> crate::Result<impl IntoIterator<Item = String>> {
        let answer = resource_client.fetch_answer(date).map_err(Error::network)?;
        self.play(answer)
    }

    /// Creates a new [`Round`] using this [`Wordle`]'s word list. This is the interactive
    /// gameplay method for a locally-derived answer. The caller will be able to play the game by
    /// calling [`Round::guess`] in a loop and evaluating the [`Colors`] returned by each call.
    /// To play using the equivalent non-interactive method, see [`Wordle::play`].
    ///
    /// # Errors
    ///
    /// This method returns an error if the passed answer is not a valid word (length, non-ASCII)
    /// -> [`wordle::Error::Fatal`].
    ///
    /// [`Colors`]: crate::Colors
    /// [`wordle::Error::Fatal`]: crate::Error::Fatal
    pub fn round(&self, answer: impl AsRef<str>) -> crate::Result<Round> {
        Ok(Round::new(
            self.words.clone(),
            Word::from_internal(answer.as_ref())?,
        ))
    }

    /// Fetches the Wordle answer for a given date, and then calls [`Wordle::round`] with that
    /// answer. This is the interactive gameplay method for a network-derived answer. The caller
    /// will be able to play the game by calling [`Round::guess`] in a loop and evaluating the
    /// [`Colors`] returned by each call. To play using the equivalent non-interactive method,
    /// see [`Wordle::play_date`].
    ///
    /// # Errors
    ///
    /// This method will return an error under the following conditions:
    ///
    ///   - The client encounters an error when resolving the answer -> [`wordle::Error::Safe`]
    ///   - The [`Wordle`] encounters an unexpected invalid state -> [`wordle::Error::Fatal`]
    ///
    /// [`Colors`]: crate::Colors
    /// [`wordle::Error::Fatal`]: crate::Error::Fatal
    /// [`wordle::Error::Safe`]: crate::Error::Safe
    pub fn round_date<D>(
        &self,
        resource_client: &impl Client<Date = D>,
        date: D,
    ) -> crate::Result<Round> {
        let answer = resource_client.fetch_answer(date).map_err(Error::network)?;
        self.round(&answer)
    }
}

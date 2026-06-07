#[cfg(test)]
mod tests;

pub mod score;

use std::cmp::Ordering;

use crate::collections::array_view::ArrayView;
use crate::game::WordExt;
use crate::game::error::Error;
use crate::game::round::score::{
    Score,
    ScoreMode,
    ScoreType,
    Scorer,
};
use crate::game::space::WordSpace;
use crate::game::word::Word;
use crate::{
    Colors,
    N_GUESSES,
};

/// Represents a single round of Wordle. Can only be played once,
/// and is interactive.
///
/// A [`Round`] can only be instantiated by a [`Wordle`]. The round retains a copy of the
/// [`Wordle`]'s word list, but generates constraints around the supplied answer. The answer can
/// be supplied manually via [`Wordle::round`] or resolved by date via [`Wordle::round_date`].
///
/// # Performance
///
/// [`Round`]s maintain one internal allocation: A copy of the word list from the [`Wordle`] which
/// generated the [`Round`]. Although words are eliminated after each [`guess`], this allocation
/// _does not_ shrink, and is never re-allocated. Like [`Wordle`], the [`Round`]'s word list is
/// represented in a fixed-size internal format which is packed into a single block of contiguous
/// memory.
///
/// The most expensive methods of [`Round`] are [`Round::best`] and [`Round::guess`], and those
/// document their own performance.
///
/// # Examples
///
/// These [`Round`]s are intended to be used either interactively, or in such a way that the
/// intermediate guesses can be evaluated in some way by the consumer.
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use wordle::Wordle;
///
/// let word_list = vec!["HELLO", "VALID", "CRUNK"];
///
/// let game = Wordle::from_list(word_list)?;
///
/// let mut round = game.round("VALID")?;
///
/// let mut guesses = vec!["AROSE", "INLAY", "VALID", "HELLO"].into_iter();
///
/// while !round.is_over() {
///     let colors = round.guess(guesses.next().unwrap())?;
///     println!("{colors:#?}");
/// }
///
/// let remainder = guesses.collect::<Vec<_>>();
/// assert!(!remainder.is_empty());
/// # Ok(())
/// # }
/// ```
///
/// If consumers intend to play through a [`Round`] fully-automatically and simply want a list of
/// the solving algorithm's guesses, then [`Wordle::play`] or [`Wordle::play_date`] can be used
/// instead. It uses a [`Round`] to back its solution, and will simply return the list of guesses.
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use wordle::Wordle;
///
/// let word_list = vec!["HELLO", "VALID", "CRUNK"];
///
/// let game = Wordle::from_list(word_list)?;
///
/// let guesses = game.play("VALID")?.into_iter().collect::<Vec<_>>();
///
/// println!("{:#?}", &guesses);
///
/// assert_eq!(guesses.last().unwrap(), "VALID");
/// # Ok(())
/// # }
/// ```
///
/// [`Wordle`]: crate::Wordle
/// [`Wordle::round`]: crate::Wordle::round
/// [`Wordle::round_date`]: crate::Wordle::round_date
/// [`Wordle::play`]: crate::Wordle::play
/// [`Wordle::play_date`]: crate::Wordle::play_date
/// [`guess`]: Round::guess
#[derive(Debug)]
pub struct Round {
    guesses: ArrayView<Word, { N_GUESSES }>,
    answer:  Word,
    words:   WordSpace,
}

impl Round {
    /// Creates new state for a round of the game by parsing the game's
    /// constraints into data which [`Round`] is able to track.
    pub(super) fn new(mut words: WordSpace, answer: Word) -> Self {
        // Handle old puzzles which might have had different word lists
        if !words.contains(answer) {
            words.include_answer(answer);
        };

        Self {
            guesses: ArrayView::default(),
            answer,
            words,
        }
    }

    /// Reports if the current guess is the answer or all guesses have been exhausted.
    pub fn is_over(&self) -> bool {
        self.guesses.len() >= N_GUESSES
            || self.guesses.peek().is_some_and(|word| word == &self.answer)
    }

    /// Determines the results of a guess based on the actual answer.
    /// The results is [`Colors`], which describes the colors
    /// that should be assigned to each letter in the provided guess.
    pub(super) fn evaluate(&self, guess: Word) -> Colors {
        self.answer.colors(guess)
    }

    /// Updates internal state vectors with the outcome of a prior guess.
    pub(super) fn commit(&mut self, guess: Word, colors: Colors) {
        self.words.update(guess, colors);
        self.guesses.push(guess);
    }

    /// Commits a guess to the active [`Round`], and returns the tile [`Colors`] derived by
    /// comparing the guess to the answer.
    ///
    /// # Performance
    ///
    /// This method will use the input data to perform a computation over every remaining word in
    /// the list to determine their validity based on the constraints defined by the input
    /// [`Colors`] and those of all previous guesses. Any words which are invalid under those
    /// constraints as dictated by the rules of [`Wordle`] are eliminated as possible valid answers
    /// to be suggested by [`Round::best`].
    ///
    /// Internal counters and state buffers modified by this method are updated in-place and are
    /// packed into contiguous, fixed-size blocks of memory. This method makes zero allocations and
    /// does not modify the memory footprint of the [`Round`].
    ///
    /// # Errors
    ///
    /// [`crate::Error::Player`] will be returned if:
    ///   * The [`Round`] [`is_over`]
    ///   * The guess is not valid within the constraints of the Wordle rules.
    ///
    /// [`is_over`]: Round::is_over
    /// [`Wordle`]: crate::Wordle
    pub fn guess(&mut self, guess: impl AsRef<str>) -> crate::Result<Colors> {
        if self.is_over() {
            return Err(Error::TooManyGuesses.into());
        };

        let guess = Word::from_guess(guess.as_ref())?;
        let colors = self.evaluate(guess);
        self.commit(guess, colors);

        Ok(colors)
    }

    /// Consumes the [`Round`] and returns an iterator over all the guesses made.
    /// The guesses are meant to be collected only after the round [`is_over`].
    ///
    /// [`is_over`]: Round::is_over
    pub fn into_guesses(self) -> impl IntoIterator<Item = String> {
        self.guesses.into_iter().map(String::from)
    }

    /// Finds the current best words in the word list by counting the most
    /// common characters and then scoring all the words with unique
    /// characters based on those counts.
    ///
    /// These scores are computed based on the constraints understood by the [`Round`] during the
    /// current turn. This means that previous [`guess`]es are taken into account. The words
    /// returned by this method reflect the ones determined to be most likely to produce value if
    /// they were guessed _right now_.
    ///
    /// This method accepts a [`ScoreMode`], which allows the caller to decide:
    ///   * If all words in the list will be considered
    ///   * If only words that have a unique set of characters will be considered
    ///   * How many of the top words will be returned
    ///     * If this number is greater than, or equal to, the total number of words in the list,
    ///       then all words will be returned with their [`Score`s].
    ///
    /// # Scoring Algorithm
    ///
    /// - For all words:
    ///   - If [`ScoreMode::UniqueOnly`] and the word does not have a unique set of letters, 0
    ///   - Sum the frequency that all unique letters in the word occur anywhere in all possible
    ///     answers at the current turn of the [`Round`]. This _roughly_ represents the probability
    ///     of getting a [`Yellow`] or [`Green`] letter by guessing the word.
    ///   - For any words that have equal scores, break the tie by summing the frequency that all
    ///     letters in the word appear in the exact same positions in all possible answers at the
    ///     current turn of the [`Round`]. This represents the probability of getting a [`Green`]
    ///     letter by guessing the word.
    ///     - If this results in yet another tie, this tie is broken by summing the frequency of all
    ///       unique letters in the word against the global word list rather than the one that has
    ///       been reduced by previous guesses.
    ///
    /// The returned [`Score`s] are sorted by these criteria, from highest to lowest.
    ///
    /// # Performance
    ///
    /// For each call to [`Round::best`], [`Score`s] are only computed one time for each possible
    /// answer to the [`Round`]. The frequencies used by the algorithm are pre-computed and stored
    /// eagerly when the [`Round`] is initialized and updated each time a [`guess`] is made, so the
    /// score computation performed for each word involves only summing these values together for
    /// each letter in the word. The same goes for the tiebreaker calculation, which is only
    /// performed when when any two words have the same initial score. The tiebreaker will also only
    /// be performed one time for each combination of words with the same score.
    ///
    /// Because the number of returned scores is dynamic and must be sorted, there is a single
    /// allocation to store any computed [`Score`s]. That buffer is returned directly to the caller.
    /// If the configured amount of scores is low and the space of possible answers to this
    /// [`Round`] is large, the returned [`IntoIterator`] will contain significant excess memory, as
    /// it had to collect [`Score`s] for all possible answers before being truncated to the caller's
    /// specification. Because of this, the [`IntoIterator`] should be consumed quickly if the
    /// caller does not want to waste this excess memory.
    ///
    /// # Examples
    ///
    /// ## Getting the best starting word
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use wordle::{
    ///     ScoreMode,
    ///     Wordle,
    /// };
    ///
    /// let word_list = vec!["HELLO", "VALID", "CRUNK"];
    ///
    /// let game = Wordle::from_list(word_list)?;
    ///
    /// let mut round = game.round("VALID")?;
    ///
    /// let best = round
    ///     .best(ScoreMode::UniqueOnly { n: 1 })
    ///     .into_iter()
    ///     .next();
    ///
    /// if let Some(word) = best {
    ///     println!("Best word: {word}");
    /// } else {
    ///     panic!("Could not find best word.");
    /// };
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ## Getting the top 10 best words
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use wordle::{
    ///     ScoreMode,
    ///     Wordle,
    /// };
    ///
    /// let mut word_list = vec![
    ///     "HELLO", "VALID", "CRUNK", "DRUNK", "SPUNK", "CLUNK", "TRUNK", "CHUNK", "FLUNK",
    /// ];
    /// word_list.extend(vec!["PLUNK", "SKUNK", "SLUNK", "STUNK"]);
    ///
    /// let game = Wordle::from_list(word_list)?;
    ///
    /// let mut round = game.round("CRUNK")?;
    ///
    /// let best = round.best(ScoreMode::Default { n: 10 });
    ///
    /// let mut count = 0usize;
    /// println!("Top 10 words:");
    ///
    /// for score in best {
    ///     println!("  {score}");
    ///     count += 1;
    /// }
    ///
    /// assert_eq!(count, 10);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Score`s]: Score
    /// [`guess`]: Round::guess
    /// [`Yellow`]: crate::Color::Yellow
    /// [`Green`]: crate::Color::Green
    pub fn best(&self, score_mode: ScoreMode) -> impl IntoIterator<Item = Score> {
        if self.words.is_empty() || score_mode.n() == 0 {
            return Vec::new();
        };

        // We don't check `score_mode.n()` here because, in that case, the caller wants us
        // to compute the actual best word.
        if self.words.len() == 1 {
            return vec![Score::new(self.words[0], 0)];
        };

        let mut scores = self
            .words
            .iter()
            .map(|word| score::compute(&self.words, word, score_mode))
            .collect::<Vec<_>>();

        scores.sort_by(|s1, s2| {
            // Sort in reverse, so the highest scores are first.
            s1.score()
                .cmp(&s2.score())
                .then_with(|| {
                    // Handle tiebreakers when both words have the same score
                    score::break_tie(&self.words, s1.word_data(), s2.word_data())
                })
                .reverse()
        });

        scores.truncate(score_mode.n());

        scores
    }
}

impl Scorer for WordSpace {
    fn score(&self, word: Word) -> ScoreType {
        self.letter_frequency(word)
    }

    fn break_tie(&self, word1: Word, word2: Word) -> Ordering {
        self.exact_letter_frequency(word1)
            .cmp(&self.exact_letter_frequency(word2))
            .then_with(|| {
                self.global_letter_frequency(word1)
                    .cmp(&self.global_letter_frequency(word2))
            })
    }
}

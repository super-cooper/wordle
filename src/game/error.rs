use std::fmt::{
    Display,
    Formatter,
};
use std::str::FromStr;

use crate::N_GUESSES;
use crate::game::word::{
    self,
    Word,
};

pub(crate) type Result<T> = std::result::Result<T, Error>;

/// Describes a game state.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum State {
    OutOfWords,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::OutOfWords => write!(f, "ran out of words"),
        }
    }
}

/// Errors for the game layer.
#[derive(Debug)]
pub(crate) enum Error {
    /// An invalid word which invalidates the game.
    InvalidWord { word: String, e: word::Error },
    /// An invalid word from a player guess.
    InvalidGuess { word: String, e: word::Error },
    /// An error resulting from use of a [`crate::net::ResourceClient`].
    Network { e: Box<dyn std::error::Error> },
    /// The player attempted to guess more than the allowed number of times.
    TooManyGuesses,
    /// Unexpected invalid game state.
    InvalidState { state: State },
}

impl Error {
    pub const fn invalid_word(word: String, e: word::Error) -> Self {
        Self::InvalidWord { word, e }
    }

    pub const fn invalid_guess(word: String, e: word::Error) -> Self {
        Self::InvalidGuess { word, e }
    }

    pub fn network(e: impl std::error::Error + 'static) -> Self {
        Self::Network { e: Box::new(e) }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidWord { e, word } => write!(f, "invalid word \"{word}\": {e}"),
            Self::InvalidGuess { e, word } => write!(f, "invalid guess \"{word}\": {e}"),
            Self::Network { e } => write!(f, "network error: {e}"),
            Self::TooManyGuesses => {
                write!(f, "attempted to guess too many times (max {N_GUESSES})")
            },
            Self::InvalidState { state } => write!(f, "invalid state: {state}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidWord { e, .. } | Self::InvalidGuess { e, .. } => Some(e),
            Self::Network { e } => Some(e.as_ref()),
            Self::InvalidState {
                state: State::OutOfWords,
            }
            | Self::TooManyGuesses => None,
        }
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::InvalidWord {
                    word: l_word,
                    e: l_e,
                },
                Self::InvalidWord {
                    word: r_word,
                    e: r_e,
                },
            )
            | (
                Self::InvalidGuess {
                    word: l_word,
                    e: l_e,
                },
                Self::InvalidGuess {
                    word: r_word,
                    e: r_e,
                },
            ) => l_word == r_word && l_e == r_e,
            (Self::InvalidState { state: l_state }, Self::InvalidState { state: r_state }) => {
                l_state == r_state
            },
            (Self::TooManyGuesses, Self::TooManyGuesses) => true,
            _ => false,
        }
    }
}

/// Extension trait for [`Word`] which provides contextual error conversion for the game layer.
pub(super) trait WordExt: Sized {
    /// Parses a string into a [`Word`], converting word errors into [`Error::InvalidGuess`].
    fn from_guess(guess: &str) -> Result<Self>;
    /// Parses a string into a [`Word`], converting word errors into [`Error::InvalidWord`].
    fn from_internal(word: &str) -> Result<Self>;
}

impl WordExt for Word {
    fn from_guess(guess: &str) -> Result<Self> {
        Self::from_str(guess).map_err(|e| Error::invalid_guess(guess.to_string(), e))
    }

    fn from_internal(word: &str) -> Result<Self> {
        Self::from_str(word).map_err(|e| Error::invalid_word(word.to_string(), e))
    }
}

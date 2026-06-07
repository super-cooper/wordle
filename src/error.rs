use std::fmt::{
    Display,
    Formatter,
};

use crate::game;

/// This error type provides a boundary between the internal API and crate consumers. It holds an
/// internal error which can be inspected via [`Error::source`].
///
/// [`Error::source`]: std::error::Error::source
#[derive(Debug, PartialEq)]
pub struct InternalError {
    pub(crate) e: game::Error,
}

impl Display for InternalError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.e.fmt(f)
    }
}

impl std::error::Error for InternalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.e)
    }
}

/// Errors that may come up while playing Wordle.
///
/// The wrapped [`InternalError`] obfuscates internal structures, but there are three variants that
/// describe the state of the game.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// An error that renders the Wordle game unplayable.
    /// A [`Wordle`] that was used when one of these is returned should be discarded.
    ///
    /// [`Wordle`]: crate::game::Wordle
    Fatal {
        /// The error that resulted in the fatal failure.
        e: InternalError,
    },
    /// An error in the system that has not made the game's state invalid.
    /// These errors are considered clean, and the operation that caused them can be safely retried.
    Safe {
        /// The error that was encountered internally, but is considered safe.
        e: InternalError,
    },
    /// An error caused by the player, such as an invalid guess.
    Player {
        /// The error tripped internally by the player's mistake.
        e: InternalError,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Fatal { e } => write!(f, "fatal: {e}"),
            Self::Safe { e } => write!(f, "everything's ok, but: {e}"),
            Self::Player { e } => write!(f, "player caused: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Fatal { e } | Self::Safe { e } | Self::Player { e } => Some(e),
        }
    }
}

impl From<game::Error> for Error {
    fn from(e: game::Error) -> Self {
        match e {
            game::Error::Network { .. } => Self::Safe {
                e: InternalError { e },
            },
            game::Error::InvalidWord { .. } | game::Error::InvalidState { .. } => Self::Fatal {
                e: InternalError { e },
            },
            game::Error::InvalidGuess { .. } | game::Error::TooManyGuesses => Self::Player {
                e: InternalError { e },
            },
        }
    }
}

use std::fmt::{
    Display,
    Formatter,
};

use crate::WORD_LENGTH;
use crate::game::letter;

pub(super) type Result<U> = std::result::Result<U, Error>;

/// Errors for the Word layer.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Error {
    InvalidLetter { e: letter::Error },
    InvalidLength { len: usize },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidLetter { e } => {
                write!(f, "invalid letter: {e}")
            },
            Self::InvalidLength { len } => {
                write!(f, "invalid length (must be {WORD_LENGTH}): {len}")
            },
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidLetter { e } => Some(e),
            Self::InvalidLength { .. } => None,
        }
    }
}

impl From<letter::Error> for Error {
    fn from(e: letter::Error) -> Self {
        Self::InvalidLetter { e }
    }
}

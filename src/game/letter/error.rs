use std::fmt::{
    Display,
    Formatter,
};

use crate::WORD_LENGTH;

pub(crate) type Result<U> = std::result::Result<U, Error>;

/// Errors for the letter layer.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Error {
    NotAsciiAlphabetic { c: char },
    InvalidPosition { i: usize },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::NotAsciiAlphabetic { c } => {
                write!(f, "'{c}' is not ASCII alphabetic (A-Z)")
            },
            Self::InvalidPosition { i } => {
                write!(f, "invalid position {i} (must be 0-{WORD_LENGTH})")
            },
        }
    }
}

impl std::error::Error for Error {}

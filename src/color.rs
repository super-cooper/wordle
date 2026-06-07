use std::fmt::{
    Display,
    Formatter,
};

use crate::WORD_LENGTH;

/// Represents the highlight color of the box where a letter would be rendered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    /// Indicates a letter that _is not_ in the answer at any other position.
    Gray,
    /// Indicates a letter that _is_ in the answer at at least one other position.
    Yellow,
    /// Indicates a letter that _is_ in the answer at the exact same position.
    Green,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// The standard collection of colors that map directly to the letters in a word by position.
pub type Colors = [Color; WORD_LENGTH];

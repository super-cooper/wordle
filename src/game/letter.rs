//! This is the "Letter" layer of Wordle. It represents the layer of abstraction pertaining to
//! letters within a word. Naturally, it provides types to represent a [`Letter`] and [`Color`].
//! There is also a convenient alias for an array of [`Color`]s that is [`WORD_LENGTH`] long. This
//! should be used to pass around [`Color`]s that are meant to map to each [`Letter`] in a word.
//!
//! This layer manages global game constraints for the game's total letter space. It has no
//! knowledge of the game's word space.

#[cfg(test)]
pub(crate) mod tests;

mod error;

use std::fmt::{
    Debug,
    Display,
    Formatter,
};
use std::hash::Hash;

pub(crate) use error::Error;
pub(super) use error::Result;

use crate::WORD_LENGTH;

/// Used to mask off the ordinal and position encoded into a single byte.
/// The ordinal is the rightmost 5 bits, and the position is the leftmost 3.
const MASK: u8 = 0b00011111;
/// The number of bits to shift the position for encode and decode.
const SHIFT: u8 = MASK.trailing_ones() as u8;

/// Represents a letter in a word in terms of an ordinal, i.e. the index 0 through 25
/// of the character between 'A' and 'Z' and the position 0 through 4 of that character in the word.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub(super) struct Letter {
    data: u8,
}

impl Letter {
    /// Returns the ordinal (A-Z -> 0-25) of the letter.
    pub const fn ordinal(self) -> usize {
        (self.data & MASK) as usize
    }

    /// Returns the position (0-4) of the letter.
    pub const fn position(self) -> usize {
        (self.data >> SHIFT) as usize
    }

    /// Creates a [`char`] representation of the letter.
    pub fn to_char(self) -> char {
        char::from(b'A' + (self.ordinal() as u8))
    }
}

impl TryFrom<(usize, char)> for Letter {
    type Error = Error;

    fn try_from((i, c): (usize, char)) -> Result<Self> {
        if !c.is_ascii_alphabetic() {
            return Err(Self::Error::NotAsciiAlphabetic { c });
        };

        if i >= WORD_LENGTH {
            return Err(Self::Error::InvalidPosition { i });
        };

        let position = i as u8;
        let ordinal = (c.to_ascii_uppercase() as u8) - b'A';

        Ok(Self {
            data: (position << SHIFT) | ordinal,
        })
    }
}

impl Display for Letter {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}({})", self.to_char(), self.position())
    }
}

impl Debug for Letter {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        (self as &dyn Display).fmt(f)
    }
}

// A size-bound sequence of letters that makes up the internals of a word.
pub(super) type Letters = [Letter; WORD_LENGTH];

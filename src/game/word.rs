//! This is the "Word" layer of Wordle. It represents the layer of abstraction pertaining to the
//! words themselves.
//!
//! The core type here is [`Word`]. It represents a valid word within the constraints of the Wordle
//! rules. It should always be used in place of strings, since it provides guarantees about the
//! validity of the data it contains.
//!
//! At the crate boundary, every input string meant to represent a word is either converted to a
//! [`Word`] or [`map`]ped to an existing one in a word list. The latter is necessary if the word
//! list already exists, as mapping to an existing word ensures that the input is valid within the
//! constraints of the word list, not just the Wordle rules.

#[cfg(test)]
pub(crate) mod tests;

mod error;

use std::fmt::{
    Debug,
    Display,
    Formatter,
};
use std::hash::Hash;
use std::str::FromStr;

use crate::collections::array_view::ArrayView;
use crate::game::letter::{
    self,
    Letter,
    Letters,
};
pub(crate) use crate::game::word::error::Error;
use crate::game::word::error::Result;
use crate::{
    ALPHABET_LENGTH,
    Color,
    Colors,
    WORD_LENGTH,
};

/// Represents a Wordle word with desired validations and behavior.
/// All strings at the crate boundary should be parsed or [`map`]ped into [`Word`]s, as a [`Word`]
/// that is invalid under the constraints of the Wordle rules cannot be constructed. This means that
/// using a [`Word`] instead of a string provides a guarantee of valid data.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct Word {
    letters: Letters,
}

impl Word {
    /// View this [`Word`] as a slice of [`Letters`].
    ///
    /// This is used instead of implementing [`IntoIterator<Item = Letter>`] to prevent [`Letter`]'s
    /// visibility from leaking outside the word layer, as trait implementations cannot be scoped
    /// to a module.
    pub const fn letters(self) -> Letters {
        self.letters
    }

    /// Compares this [`Word`] against another and derives the resulting
    /// [`Colors`] that would be returned for this word based on the rules of
    /// Wordle.
    ///
    /// In the context of this method, [`self`] is the answer to the puzzle.
    pub fn colors(self, guess: Self) -> Colors {
        // Track final color results to return
        let mut colors = [Color::Gray; WORD_LENGTH];
        // Count the amount of each letter available to be marked yellow
        let mut available_yellows = [0u8; ALPHABET_LENGTH];

        // First pass: count all the letters in self
        for letter in self.letters {
            available_yellows[letter.ordinal()] += 1;
        }

        // Second pass: find greens
        for letter in guess.letters {
            let ordinal = letter.ordinal();
            let position = letter.position();
            if self.letters[position] == letter {
                colors[position] = Color::Green;
                available_yellows[ordinal] = available_yellows[ordinal].saturating_sub(1);
            };
        }

        // Third pass: now that all greens are known,
        // we can accurately determine which other letters are yellow
        for letter in guess.letters {
            let ordinal = letter.ordinal();
            let position = letter.position();
            // If the letter isn't already marked green and there are yellows available,
            // then the letter is yellow
            if !matches!(colors[position], Color::Green) && available_yellows[ordinal] > 0 {
                colors[position] = Color::Yellow;
                available_yellows[ordinal] = available_yellows[ordinal].saturating_sub(1);
            };
        }

        // All remaining letters are invalid, and left gray
        colors
    }

    /// Determines if the [`Word`] contains all unique letters.
    pub fn is_unique(self) -> bool {
        for (i, l_1) in self.letters.into_iter().enumerate() {
            for (j, l_2) in self.letters.into_iter().enumerate() {
                if i != j && l_1.ordinal() == l_2.ordinal() {
                    return false;
                }
            }
        }
        true
    }
}

impl FromStr for Word {
    type Err = Error;

    /// Creates a new [`Word`] from a [`&str`] input.
    ///
    /// This function makes a best effort to pre-process dirty data:
    ///   - Whitespace trimmed
    ///   - Case normalized
    ///
    /// The input data is validated against the following constraints after pre-processing:
    ///   - Length is equal to [`WORD_LENGTH`]
    ///   - All characters are valid ASCII alphabetic characters (A-Z)
    ///
    /// Any deviation from the above constraints will result in an [`Error`].
    fn from_str(word: &str) -> Result<Self> {
        let word = word.trim();

        // This is safe because letters know their position, and will short circuit before
        // attempting to write beyond the bounds of the ArrayView
        word.chars()
            .enumerate()
            .map(Letter::try_from)
            .collect::<letter::Result<ArrayView<Letter, WORD_LENGTH>>>()?
            .try_into()
    }
}

impl TryFrom<ArrayView<Letter, WORD_LENGTH>> for Word {
    type Error = Error;

    fn try_from(value: ArrayView<Letter, WORD_LENGTH>) -> Result<Self> {
        let len = value.len();
        if len != WORD_LENGTH {
            return Err(Error::InvalidLength { len });
        }
        Ok(Self {
            letters: value.into(),
        })
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let text = self
            .letters
            .iter()
            .copied()
            .map(Letter::to_char)
            .collect::<String>();
        write!(f, "{text}")
    }
}

impl Debug for Word {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_list().entries(self.letters).finish()
    }
}

impl From<Word> for String {
    fn from(word: Word) -> Self {
        word.to_string()
    }
}

#[cfg(test)]
pub(crate) mod tests;

mod letter_space;

use std::ops::Index;

use crate::game::space::letter_space::LetterSpace;
use crate::game::word::Word;
use crate::{
    ALPHABET_LENGTH,
    Colors,
    WORD_LENGTH,
};

/// Understands the space of all possible winning words.
///
/// It is initialized with its own copy of all the valid [`Word`]s in the game.
/// This is because the [`WordSpace`]'s word list will mutate as it learns which words
/// are invalid.
///
/// As this game is played, the [`WordSpace`] will need to be updated via [`WordSpace::update`]
/// with the results each guess to narrow down the space of valid guesses.
///
/// [`WordSpace`] also tracks positional and frequency data of all the letters in the valid answer
/// space. Therefore, it should be used to calculate which guesses have the highest probability of
/// resulting in the most Yellow and Green letters.
#[derive(Debug, Clone)]
pub(super) struct WordSpace {
    words:   Vec<Word>,
    letters: LetterSpace,
}

impl WordSpace {
    /// Tells if this [`WordSpace`] does not contain any [`Word`]s.
    pub const fn is_empty(&self) -> bool {
        self.words.is_empty()
    }

    /// This is not expected to be used throughout the course of the game. It is only here in case
    /// an old game is being played, and the answer is not in the current published version of the
    /// word list. Including the answer into the space ensures that the puzzle will remain solvable.
    pub fn include_answer(&mut self, answer: Word) {
        for letter in answer.letters() {
            self.letters.add(letter);
            self.letters.add_global(letter);
        }
        self.words.push(answer);
    }

    /// Returns true if the given word is still considered a possible winning word.
    pub fn contains(&self, word: Word) -> bool {
        self.words.contains(&word)
    }

    /// Returns the number of possible winning words.
    pub const fn len(&self) -> usize {
        self.words.len()
    }

    /// Updates the constraints based on the passed guess and the resulting colors.
    /// The new information passed to this method allows us to eliminate a number of words from the
    /// pool of possible winners.
    pub fn update(&mut self, guess: Word, colors: Colors) {
        self.letters.update(guess.letters(), colors);
        self.words.retain(|word| {
            let is_valid = self.letters.validate(word.letters());
            if !is_valid {
                for letter in word.letters().iter().copied() {
                    self.letters.remove(letter);
                }
            };
            is_valid
        });
    }

    /// Get an iterator over all possible winning words.
    pub fn iter(&self) -> impl Iterator<Item = Word> {
        self.words.iter().copied()
    }

    /// Calculates the frequency at which all the unique letters in the given [`Word`] appear at any
    /// position across all words in the space.
    pub fn letter_frequency(&self, word: Word) -> u32 {
        const SENTINEL: usize = ALPHABET_LENGTH + 1;
        // Tracks unique letter scores as (ordinal, shared_count).
        // For the ordinal, 26 acts as a sentinel value because it is invalid, so if we encounter
        // that, we know the letter being scanned has not been counted before.
        let mut counts = [(SENTINEL, 0u32); WORD_LENGTH];
        for letter in word.letters() {
            let ordinal = letter.ordinal();
            for (ord_i, count) in &mut counts {
                if *ord_i != ordinal && *ord_i != SENTINEL {
                    continue;
                };

                *ord_i = ordinal;
                *count = self.letters.shared_count(letter);
                break;
            }
        }

        counts.into_iter().map(|(_, value)| value).sum()
    }

    /// Calculates the frequency at which all the letters in the given [`Word`] appear at the exact
    /// same positions across all words in the space.
    pub fn exact_letter_frequency(&self, word: Word) -> u32 {
        word.letters()
            .iter()
            .map(|letter| self.letters.exact_count(*letter))
            .sum()
    }

    /// Calculates the frequency at which all the letters in the given [`Word`] appear at any
    /// position across all words in the global word list.
    pub fn global_letter_frequency(&self, word: Word) -> u32 {
        word.letters()
            .iter()
            .map(|letter| self.letters.global_count(*letter))
            .sum()
    }
}

impl FromIterator<Word> for WordSpace {
    fn from_iter<T: IntoIterator<Item = Word>>(iter: T) -> Self {
        let words = iter.into_iter().collect::<Vec<_>>();
        let letters = words
            .iter()
            .copied()
            .flat_map(Word::letters)
            .collect::<LetterSpace>();

        Self { words, letters }
    }
}

impl Index<usize> for WordSpace {
    type Output = Word;

    fn index(&self, index: usize) -> &Self::Output {
        &self.words[index]
    }
}

#[cfg(test)]
mod tests;

use std::cmp::Ordering;
use std::fmt::{
    Display,
    Formatter,
};

use crate::game::word::Word;

/// The standard numeric type used to represent a word score.
#[allow(
    clippy::module_name_repetitions,
    reason = "Publicly exported via wordle namespace"
)]
pub type ScoreType = u32;

/// Provides an implementation to derive a score from a word.
pub(super) trait Scorer {
    /// Defines a heuristic by which a [`Word`] may be scored.
    fn score(&self, word: Word) -> ScoreType;

    /// Defines a different calculation which breaks ties when multiple scores are equal.
    fn break_tie(&self, word1: Word, word2: Word) -> Ordering;
}

/// The desired approach for computation of word scores.
#[derive(Clone, Copy, Debug)]
#[allow(
    clippy::module_name_repetitions,
    reason = "Publicly exported via wordle namespace"
)]
pub enum ScoreMode {
    /// Score the words such that they are all considered under equal standing.
    Default {
        /// The number of words output by the scoring algorithm.
        n: usize,
    },
    /// Compute scores only for words that don't have any repeat characters.
    UniqueOnly {
        /// The number of words output by the scoring algorithm.
        n: usize,
    },
}

impl ScoreMode {
    /// Returns the number configured for the amount of words output by the scoring algorithm.
    pub const fn n(self) -> usize {
        match self {
            Self::Default { n } | Self::UniqueOnly { n } => n,
        }
    }
}

/// Associates a word with its pre-computed score.
#[derive(Debug, Eq)]
pub struct Score {
    word:  Word,
    score: ScoreType,
}

impl Score {
    /// Create a new [`Score`].
    pub(super) const fn new(word: Word, score: ScoreType) -> Self {
        Self { word, score }
    }

    /// Borrows a reference to the held value.
    pub(in crate::game) const fn word_data(&self) -> Word {
        self.word
    }

    /// Returns the text of the held word.
    pub fn word(&self) -> String {
        self.word.to_string()
    }

    /// Returns the pre-computed score for the held value.
    pub const fn score(&self) -> ScoreType {
        self.score
    }
}

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        self.score.eq(&other.score)
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.word, self.score)
    }
}

/// Score a [`Word`] using the heuristic defined by the passed [`Scorer`].
///
/// If the mode is [`ScoreMode::UniqueOnly`], then all non-unique values receive a score
/// of 0.
pub(super) fn compute(scorer: &impl Scorer, word: Word, mode: ScoreMode) -> Score {
    if matches!(mode, ScoreMode::UniqueOnly { .. }) && !word.is_unique() {
        return Score::new(word, 0);
    };

    Score::new(word, scorer.score(word))
}

/// Break a tie among [`Scorable`] items when their scores returned by [`compute`] are the same.
///
/// If the mode is [`ScoreMode::UniqueOnly`], then all non-unique values receive a score
/// of 0.
pub(super) fn break_tie(scorer: &impl Scorer, word1: Word, word2: Word) -> Ordering {
    scorer.break_tie(word1, word2)
}

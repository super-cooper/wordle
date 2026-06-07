#[cfg(test)]
mod tests;

use std::num::Saturating;

use crate::color::{
    Color,
    Colors,
};
use crate::game::letter::{
    Letter,
    Letters,
};
use crate::{
    ALPHABET_LENGTH,
    WORD_LENGTH,
};

// Used to track which positions are forbidden for a single letter
type PositionMask = u8;

// Each letter gets a mask where a 1 in any position 0-4 indicates that the given letter
// is forbidden from being used in that position
type ForbiddenBitmap = [PositionMask; ALPHABET_LENGTH];

// Tracks limit constraints for each letter. Either the minimum required or maximum allowed amount
// of the given letter for any given word.
type LimitTracker = [u8; ALPHABET_LENGTH];

// Counts the number of times a letter appears in any position in any word
type SharedCounter = [Saturating<u32>; ALPHABET_LENGTH];

// Counts the number of times a letter appears in a specific position in any word
type ExactCounter = [[Saturating<u32>; ALPHABET_LENGTH]; WORD_LENGTH];

/// Tracks data about the global letter space across all currently valid words.
///
/// The data tracked is as follows:
///   - The number of times each letter appears overall in the global word list. This counter never
///     shrinks.
///   - The number of each letter that appears in any position in any word
///   - The number of each letter that appears in a specific position in any word
///   - The known minimum required amount of each letter for a word to be a potential winner.
///   - The known maximum allowed amount of each letter for a word to be a potential winner
///   - The positions where each letter is known to be invalid (already picked gray)
#[derive(Debug, Clone)]
pub(super) struct LetterSpace {
    global_counter:      SharedCounter,
    shared_counter:      SharedCounter,
    exact_counter:       ExactCounter,
    min_required:        LimitTracker,
    max_allowed:         LimitTracker,
    forbidden_positions: ForbiddenBitmap,
}

impl LetterSpace {
    /// This is the "read" method of [`LetterSpace`]. It determines if a set of
    /// [`Letter`]s is valid under the constraints understood by this type.
    ///
    /// The constraints validated are as follows:
    ///
    ///   - The [`Letter`] must not be known as invalid at its given position, i.e. derived
    ///     [`Color::Gray`] at that position, or a different [`Letter`] with [`Color::Green`] at
    ///     the same position.
    ///   - There must be _at least_ the currently-known minimum required amount of each letter in
    ///     the sequence. This value is raised by encountering the same letter in different
    ///     positions with [`Color::Yellow`].
    ///   - There must be _at most_ the currently-known maximum allowed amount of each letter in the
    ///     sequence. This value is clamped by encountering the same [`Letter`] at different
    ///     positions with [`Color::Gray`].
    pub fn validate(&self, letters: Letters) -> bool {
        // This counter tracks the limit bounds implied by the passed letters. Once all of the
        // unique ordinals in the letters are counted (with all others defaulting to zero),
        // we can compare those implied constraints to the ones we know to be true, as enforced by
        // Self::min_required and Self::max_allowed.
        let mut counter = LimitTracker::default();

        // Accumulate counts for the passed letters, but short-circuit if any letters are at an
        // invalid position
        for letter in letters {
            let ordinal = letter.ordinal();
            let position = letter.position();

            // If any letter is at an invalid position, the word is invalid.
            let forbidden_bitmap = self.forbidden_positions[ordinal];
            let position_mask = (PositionMask::default() + 1) << position;
            if forbidden_bitmap & position_mask != PositionMask::default() {
                return false;
            };

            counter[ordinal] += 1;
        }

        // At this point, we have constraints for the passed letters accumulated in the counter.
        // Now, we verify those constraints against the constraints of the letter space.
        counter.iter().copied().enumerate().all(|(ordinal, count)| {
            count >= self.min_required[ordinal] && count <= self.max_allowed[ordinal]
        })
    }

    /// This is the "write" method of [`LetterSpace`]. The passed [`Letter`]s and
    /// [`Colors`] provide new information to update the existing constraints.
    ///
    /// The rules tested by this function are as followed:
    ///
    ///   - A [Green] [`Letter`] confirms that the answer MUST have that [ordinal] in the exact
    ///     position.
    ///   - A [Green] [`Letter`] confirms that no other [ordinal] may appear at the given position.
    ///   - Each [`Letter`] that appears as [Yellow] or [Green] must also appear in the answer _at
    ///     least_ the same amount of times as it appears [Yellow] or [Green] in the guess.
    ///   - Each [`Letter`] that appears as [Gray] must only appear in the answer _at most_ the
    ///     same amount of times as it appears [Yellow] or [Green] in the guess.
    ///
    /// [Green]: `Color::Green`
    /// [Yellow]: `Color::Yellow`
    /// [Gray]: `Color::Gray`
    /// [ordinal]: [`Letter::ordinal`]
    pub fn update(&mut self, letters: Letters, colors: Colors) {
        // Track the counts of all letters confirmed to be in the answer based on the input.
        // Succinctly: the counts of all yellows and greens for each unique ordinal in the input.
        // The values accumulated here will be used to tighten the min/max constraints if applicable
        let mut counter = LimitTracker::default();
        // Track any gray letters that come up by ordinal. When a letter is gray, that means it
        // cannot show up anywhere else in the word, so its max_allowed constraint can be clamped to
        // the amount of that letter found in the coordinates passed to this function.
        let mut gray = [false; ALPHABET_LENGTH];

        for (letter, color) in letters.iter().zip(colors) {
            let forbidden_mask = (PositionMask::default() + 1) << letter.position();
            let ordinal = letter.ordinal();

            match color {
                Color::Green => {
                    counter[ordinal] += 1;
                    // Since green confirms the letter at the given position,
                    // it effectively forbids all other letters from being at that position.
                    for other in (0..(ordinal)).chain((ordinal + 1)..ALPHABET_LENGTH) {
                        self.forbidden_positions[other] |= forbidden_mask;
                    }
                },
                Color::Yellow => {
                    counter[ordinal] += 1;
                    self.forbidden_positions[ordinal] |= forbidden_mask;
                },
                Color::Gray => {
                    gray[ordinal] = true;
                    self.forbidden_positions[ordinal] |= forbidden_mask;
                },
            };
        }

        // Re-calculate global limits for each letter
        for (ordinal, count) in counter.iter().copied().enumerate() {
            let min_required = &mut self.min_required[ordinal];
            let max_allowed = &mut self.max_allowed[ordinal];

            if count > *min_required {
                *min_required = count;
            };

            // We found a gray letter, which means we can clamp it to the number of times
            // it appeared in the current guess, as it's not going to be anywhere else.
            if gray[ordinal] && count < *max_allowed {
                *max_allowed = count;
            };
        }
    }

    /// Add a [`Letter`] to the counter.
    pub fn add(&mut self, letter: Letter) {
        self.shared_counter[letter.ordinal()] += 1;
        self.exact_counter[letter.position()][letter.ordinal()] += 1;
    }

    /// Add a [`Letter`] to the global counter. This should only be done at initialization time of
    /// an owning struct.
    pub fn add_global(&mut self, letter: Letter) {
        self.global_counter[letter.ordinal()] += 1;
    }

    /// Remove a [`Letter`] from the counter.
    pub fn remove(&mut self, letter: Letter) {
        self.shared_counter[letter.ordinal()] -= 1;
        self.exact_counter[letter.position()][letter.ordinal()] -= 1;
    }

    /// Get the amount of times a letter appears at any position in the letter space.
    pub const fn shared_count(&self, letter: Letter) -> u32 {
        self.shared_counter[letter.ordinal()].0
    }

    /// Get the amount of times a letter appears in an exact position in the letter space.
    pub const fn exact_count(&self, letter: Letter) -> u32 {
        self.exact_counter[letter.position()][letter.ordinal()].0
    }

    /// Get the amount of times a letter appears in the overall global word list.
    pub const fn global_count(&self, letter: Letter) -> u32 {
        self.global_counter[letter.ordinal()].0
    }
}

impl Default for LetterSpace {
    fn default() -> Self {
        Self {
            global_counter:      SharedCounter::default(),
            shared_counter:      SharedCounter::default(),
            exact_counter:       ExactCounter::default(),
            min_required:        LimitTracker::default(),
            max_allowed:         [WORD_LENGTH as u8; ALPHABET_LENGTH],
            forbidden_positions: ForbiddenBitmap::default(),
        }
    }
}

impl FromIterator<Letter> for LetterSpace {
    fn from_iter<T: IntoIterator<Item = Letter>>(iter: T) -> Self {
        let mut space = Self::default();
        for letter in iter {
            space.add(letter);
        }
        space.global_counter = space.shared_counter;

        space
    }
}

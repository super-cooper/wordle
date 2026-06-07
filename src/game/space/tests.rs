use assertables::{
    assert_contains,
    assert_in,
    assert_is_empty,
    assert_iter_eq,
    assert_len_eq_x,
    assert_not_contains,
    assert_not_empty,
};
use fixtures::*;
use rstest::{
    fixture,
    rstest,
};

use super::*;
use crate::game::word::tests::fixtures::*;
use crate::{
    Color,
    WORD_LENGTH,
};

impl WordSpace {
    pub fn test(words: Vec<Word>) -> Self {
        let mut letters = LetterSpace::default();
        for word in &words {
            for letter in word.letters() {
                letters.add(letter);
            }
        }
        Self { words, letters }
    }
}

pub mod fixtures {
    use super::*;

    #[fixture]
    pub fn test_word_space(valid_data: Vec<Word>) -> WordSpace {
        WordSpace::test(valid_data)
    }

    #[fixture]
    pub fn empty_word_space() -> WordSpace {
        WordSpace::test(Vec::new())
    }
}

mod from_iter {
    use super::*;

    #[rstest]
    fn full(valid_data: Vec<Word>) {
        let cp = valid_data.clone();

        let space = WordSpace::from_iter(valid_data);

        assert_eq!(&space.words, &cp);
        assert_eq!(space.letters.total_letters(), 3 * WORD_LENGTH as u32);
    }

    #[rstest]
    fn empty() {
        let space = WordSpace::from_iter(Vec::new());

        assert_is_empty!(space.words);
        assert_eq!(space.letters.total_letters(), 0);
    }
}

mod is_empty {
    use super::*;

    #[rstest]
    fn full(test_word_space: WordSpace) {
        assert_not_empty!(test_word_space);
    }

    #[rstest]
    fn empty(empty_word_space: WordSpace) {
        assert_is_empty!(empty_word_space);
    }
}

#[rstest]
fn include_answer(mut empty_word_space: WordSpace, grunt: Word) {
    empty_word_space.include_answer(grunt.clone());

    assert_in!(&grunt, empty_word_space.words);
    assert_eq!(empty_word_space.letters.total_letters(), WORD_LENGTH as u32);
}

mod contains {
    use super::*;

    #[rstest]
    fn does(test_word_space: WordSpace, valid: Word) {
        assert_contains!(test_word_space, valid);
    }

    #[rstest]
    fn doesnt(test_word_space: WordSpace, grunt: Word) {
        assert_not_contains!(test_word_space, grunt);
    }
}

#[rstest]
fn len(test_word_space: WordSpace) {
    assert_len_eq_x!(test_word_space, 3);
}

#[rstest]
fn update(mut test_word_space: WordSpace, grunt: Word, crunk: Word) {
    let colors = [
        Color::Gray,
        Color::Green,
        Color::Green,
        Color::Green,
        Color::Gray,
    ];

    test_word_space.update(grunt, colors);

    assert_len_eq_x!(&test_word_space.words, 1);
    assert_contains!(&test_word_space.words, &crunk);
    assert_eq!(test_word_space.letters.total_letters(), WORD_LENGTH as u32);
}

#[rstest]
fn iter(test_word_space: WordSpace) {
    let data = test_word_space.words.clone();
    assert_iter_eq!(
        test_word_space.iter().collect::<Vec<_>>(),
        data.into_iter().collect::<Vec<_>>()
    );
}

#[rstest]
fn global_letter_frequency(mut empty_word_space: WordSpace, valid: Word, hello: Word) {
    for letter in valid.letters().into_iter().chain(hello.letters()) {
        empty_word_space.letters.add_global(letter);
    }

    // v(1) + a(1) + l(3) + i(1) + d(1)
    assert_eq!(empty_word_space.global_letter_frequency(valid), 7);
}

mod letter_frequency {
    use super::*;

    #[rstest]
    fn unique(mut empty_word_space: WordSpace, valid: Word, hello: Word) {
        for letter in valid.letters().into_iter().chain(hello.letters()) {
            empty_word_space.letters.add(letter);
        }

        // v(1) + a(1) + l(3) + i(1) + d(1)
        assert_eq!(empty_word_space.letter_frequency(valid), 7);
    }

    #[rstest]
    fn duplicate(mut empty_word_space: WordSpace, valid: Word, hello: Word) {
        for letter in valid.letters().into_iter().chain(hello.letters()) {
            empty_word_space.letters.add(letter);
        }

        // h(1) + e(1) + l(3) + l(0) + o(1)
        assert_eq!(empty_word_space.letter_frequency(hello), 6);
    }
}

#[rstest]
fn exact_letter_frequency(mut empty_word_space: WordSpace, valid: Word, hello: Word) {
    for letter in valid.letters().into_iter().chain(hello.letters()) {
        empty_word_space.letters.add(letter);
    }

    // v(1) + a(1) + l(2) + i(1) + d(1)
    assert_eq!(empty_word_space.exact_letter_frequency(valid), 6);
}

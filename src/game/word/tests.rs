use std::assert_matches;

use assertables::{
    assert_all,
    assert_err_eq_x,
    assert_ok,
};
use fixtures::*;
use proptest::proptest;
use rstest::{
    fixture,
    rstest,
};

use super::*;
use crate::game::letter::tests::fixtures::*;

pub mod fixtures {
    use super::*;

    #[fixture]
    pub fn valid_strings() -> Vec<String> {
        ["HELLO", "VALID", "CRUNK"]
            .into_iter()
            .map(String::from)
            .collect()
    }

    #[fixture]
    pub fn valid_data(hello: Word, valid: Word, crunk: Word) -> Vec<Word> {
        vec![hello, valid, crunk]
    }

    #[fixture]
    pub fn valid_answer() -> String {
        String::from("VALID")
    }

    #[fixture]
    pub fn crunk(c0: Letter, r1: Letter, u2: Letter, n3: Letter, k4: Letter) -> Word {
        Word {
            letters: [c0, r1, u2, n3, k4],
        }
    }

    #[fixture]
    pub fn divla(d0: Letter, i1: Letter, v2: Letter, l3: Letter, a4: Letter) -> Word {
        Word {
            letters: [d0, i1, v2, l3, a4],
        }
    }

    #[fixture]
    pub fn gaunt(g0: Letter, a1: Letter, u2: Letter, n3: Letter, t4: Letter) -> Word {
        Word {
            letters: [g0, a1, u2, n3, t4],
        }
    }

    #[fixture]
    pub fn glunk(g0: Letter, l1: Letter, u2: Letter, n3: Letter, k4: Letter) -> Word {
        Word {
            letters: [g0, l1, u2, n3, k4],
        }
    }

    #[fixture]
    pub fn grunt(g0: Letter, r1: Letter, u2: Letter, n3: Letter, t4: Letter) -> Word {
        Word {
            letters: [g0, r1, u2, n3, t4],
        }
    }

    #[fixture]
    pub fn halid(h0: Letter, a1: Letter, l2: Letter, i3: Letter, d4: Letter) -> Word {
        Word {
            letters: [h0, a1, l2, i3, d4],
        }
    }

    #[fixture]
    pub fn hello(h0: Letter, e1: Letter, l2: Letter, l3: Letter, o4: Letter) -> Word {
        Word {
            letters: [h0, e1, l2, l3, o4],
        }
    }

    #[fixture]
    pub fn llama(l0: Letter, l1: Letter, a2: Letter, m3: Letter, a4: Letter) -> Word {
        Word {
            letters: [l0, l1, a2, m3, a4],
        }
    }

    #[fixture]
    pub fn munch(m0: Letter, u1: Letter, n2: Letter, c3: Letter, h4: Letter) -> Word {
        Word {
            letters: [m0, u1, n2, c3, h4],
        }
    }

    #[fixture]
    pub fn small(s0: Letter, m1: Letter, a2: Letter, l3: Letter, l4: Letter) -> Word {
        Word {
            letters: [s0, m1, a2, l3, l4],
        }
    }

    #[fixture]
    pub fn valid(v0: Letter, a1: Letter, l2: Letter, i3: Letter, d4: Letter) -> Word {
        Word {
            letters: [v0, a1, l2, i3, d4],
        }
    }

    #[fixture]
    pub fn grunt_str() -> &'static str {
        const GRUNT: &str = "GRUNT";
        GRUNT
    }
}

mod from_str {
    use super::*;

    // 5 ASCII alphabetic characters surrounded by arbitrary whitespace
    const VALID_WORDSPACE: &str = "\\s*([a-zA-Z]{5})\\s*";

    proptest! {
        #[test]
        fn valid(s in VALID_WORDSPACE) {
            let w = Word::from_str(&s);

            assert_ok!(w, "{s}");
        }

        #[test]
        fn invalid_length(s in "\\s*([a-zA-Z]{0,4}|[a-zA-Z]{6,})\\s*") {
            let w = Word::from_str(&s);

            assert_matches!(
                w,
                Err(
                    Error::InvalidLetter { e: letter::Error::InvalidPosition { .. }}
                    | Error::InvalidLength { .. }
                ),
                "{s}"
            );
        }
    }

    #[rstest]
    #[case("abcde", 0)]
    #[case("abcde", 1)]
    #[case("abcde", 2)]
    #[case("abcde", 3)]
    #[case("abcde", 4)]
    #[trace]
    fn invalid_chars(#[case] s: &str, #[case] replace: usize) {
        let s = s
            .chars()
            .enumerate()
            .map(|(i, c)| if i == replace { '🥴' } else { c })
            .collect::<String>();

        let w = Word::from_str(&s);

        assert_matches!(
            w,
            Err(Error::InvalidLetter {
                e: letter::Error::NotAsciiAlphabetic { .. },
            }),
        )
    }
}

mod try_from_array_view {
    use super::*;

    #[rstest]
    fn ok(h0: Letter, e1: Letter, l2: Letter, l3: Letter, o4: Letter) {
        let mut av = ArrayView::<Letter, WORD_LENGTH>::default();
        av.push(h0);
        av.push(e1);
        av.push(l2);
        av.push(l3);
        av.push(o4);

        let w = assert_ok!(Word::try_from(av));

        assert_eq!(w.letters[0], h0);
        assert_eq!(w.letters[1], e1);
        assert_eq!(w.letters[2], l2);
        assert_eq!(w.letters[3], l3);
        assert_eq!(w.letters[4], o4);
    }

    #[rstest]
    #[case(vec![v0(), a1(), l2(), i3()])]
    #[case(vec![v0(), a1(), l2()])]
    #[case(vec![v0(), a1()])]
    #[case(vec![v0()])]
    #[case(vec![])]
    fn too_short(#[case] letters: Vec<Letter>) {
        let len = letters.len();
        let mut av = ArrayView::<Letter, WORD_LENGTH>::default();

        for letter in letters {
            av.push(letter);
        }

        let res = Word::try_from(av);

        assert_err_eq_x!(res, Error::InvalidLength { len });
    }
}

proptest! {
    #[test]
    fn display(s in "[A-Z]{5}") {
        let w = assert_ok!(Word::from_str(&s));

        assert_eq!(format!("{w}"), s);
    }
}

#[rstest]
fn as_letters(hello: Word) {
    assert_eq!(hello.letters(), hello.letters);
}

mod colors {
    use super::*;

    #[rstest]
    fn all_green(hello: Word) {
        let colors = hello.colors(hello);

        assert_all!(colors.into_iter(), |c| c == Color::Green);
    }

    #[rstest]
    fn all_yellow(valid: Word, divla: Word) {
        let colors = valid.colors(divla);

        assert_all!(colors.into_iter(), |c| c == Color::Yellow);
    }

    #[rstest]
    fn all_gray(hello: Word, grunt: Word) {
        let colors = hello.colors(grunt);

        assert_all!(colors.into_iter(), |c| c == Color::Gray);
    }

    mod duplicate {
        use super::*;

        #[rstest]
        fn one_green(hello: Word, valid: Word) {
            let colors = valid.colors(hello);

            assert_eq!(
                colors,
                [
                    Color::Gray,
                    Color::Gray,
                    Color::Green,
                    Color::Gray,
                    Color::Gray
                ]
            );
        }

        #[rstest]
        fn one_yellow(hello: Word, glunk: Word) {
            let colors = glunk.colors(hello);

            assert_eq!(
                colors,
                [
                    Color::Gray,
                    Color::Gray,
                    Color::Yellow,
                    Color::Gray,
                    Color::Gray
                ]
            );
        }

        #[rstest]
        fn two_yellow(hello: Word, llama: Word) {
            let colors = llama.colors(hello);

            assert_eq!(
                colors,
                [
                    Color::Gray,
                    Color::Gray,
                    Color::Yellow,
                    Color::Yellow,
                    Color::Gray
                ]
            );
        }

        #[rstest]
        fn one_green_one_yellow(hello: Word, small: Word) {
            let colors = small.colors(hello);

            assert_eq!(
                colors,
                [
                    Color::Gray,
                    Color::Gray,
                    Color::Yellow,
                    Color::Green,
                    Color::Gray
                ]
            );
        }
    }

    #[rstest]
    fn mixed(llama: Word, small: Word) {
        let colors = llama.colors(small);

        assert_eq!(
            colors,
            [
                Color::Gray,
                Color::Yellow,
                Color::Green,
                Color::Yellow,
                Color::Yellow
            ]
        );
    }
}

mod is_unique {
    use super::*;

    #[rstest]
    fn is(valid: Word) {
        assert!(valid.is_unique());
    }

    #[rstest]
    fn isnt(hello: Word) {
        assert!(!hello.is_unique());
    }
}

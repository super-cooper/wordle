use assertables::{
    assert_err_eq_x,
    assert_ok,
};
use proptest::proptest;
use rstest::{
    fixture,
    rstest,
};

use super::*;

pub mod fixtures {
    use super::*;

    #[fixture]
    pub fn a0() -> Letter {
        Letter { data: 0b000_00000 }
    }

    #[fixture]
    pub fn b0() -> Letter {
        Letter { data: 0b000_00001 }
    }

    #[fixture]
    pub fn c0() -> Letter {
        Letter { data: 0b000_00010 }
    }

    #[fixture]
    pub fn d0() -> Letter {
        Letter { data: 0b000_00011 }
    }

    #[fixture]
    pub fn g0() -> Letter {
        Letter { data: 0b000_00110 }
    }

    #[fixture]
    pub fn h0() -> Letter {
        Letter { data: 0b000_00111 }
    }

    #[fixture]
    pub fn l0() -> Letter {
        Letter { data: 0b000_01011 }
    }

    #[fixture]
    pub fn m0() -> Letter {
        Letter { data: 0b000_01100 }
    }

    #[fixture]
    pub fn s0() -> Letter {
        Letter { data: 0b000_10010 }
    }

    #[fixture]
    pub fn v0() -> Letter {
        Letter { data: 0b000_10101 }
    }

    #[fixture]
    pub fn a1() -> Letter {
        Letter { data: 0b001_00000 }
    }

    #[fixture]
    pub fn e1() -> Letter {
        Letter { data: 0b001_00100 }
    }

    #[fixture]
    pub fn i1() -> Letter {
        Letter { data: 0b001_01000 }
    }

    #[fixture]
    pub fn l1() -> Letter {
        Letter { data: 0b001_01011 }
    }

    #[fixture]
    pub fn m1() -> Letter {
        Letter { data: 0b001_01100 }
    }

    #[fixture]
    pub fn r1() -> Letter {
        Letter { data: 0b001_10001 }
    }

    #[fixture]
    pub fn u1() -> Letter {
        Letter { data: 0b001_10100 }
    }

    #[fixture]
    pub fn x1() -> Letter {
        Letter { data: 0b001_10111 }
    }

    #[fixture]
    pub fn a2() -> Letter {
        Letter { data: 0b010_00000 }
    }

    #[fixture]
    pub fn g2() -> Letter {
        Letter { data: 0b010_00110 }
    }

    #[fixture]
    pub fn l2() -> Letter {
        Letter { data: 0b010_01011 }
    }

    #[fixture]
    pub fn n2() -> Letter {
        Letter { data: 0b010_01101 }
    }

    #[fixture]
    pub fn u2() -> Letter {
        Letter { data: 0b010_10100 }
    }

    #[fixture]
    pub fn v2() -> Letter {
        Letter { data: 0b010_10101 }
    }

    #[fixture]
    pub fn a3() -> Letter {
        Letter { data: 0b011_00000 }
    }

    #[fixture]
    pub fn c3() -> Letter {
        Letter { data: 0b011_00010 }
    }

    #[fixture]
    pub fn i3() -> Letter {
        Letter { data: 0b011_01000 }
    }

    #[fixture]
    pub fn l3() -> Letter {
        Letter { data: 0b011_01011 }
    }

    #[fixture]
    pub fn m3() -> Letter {
        Letter { data: 0b011_01100 }
    }

    #[fixture]
    pub fn n3() -> Letter {
        Letter { data: 0b011_01101 }
    }

    #[fixture]
    pub fn t3() -> Letter {
        Letter { data: 0b011_10011 }
    }

    #[fixture]
    pub fn a4() -> Letter {
        Letter { data: 0b100_00000 }
    }

    #[fixture]
    pub fn d4() -> Letter {
        Letter { data: 0b100_00011 }
    }

    #[fixture]
    pub fn h4() -> Letter {
        Letter { data: 0b100_00111 }
    }

    #[fixture]
    pub fn k4() -> Letter {
        Letter { data: 0b100_01010 }
    }

    #[fixture]
    pub fn l4() -> Letter {
        Letter { data: 0b100_01011 }
    }
    #[fixture]
    pub fn o4() -> Letter {
        Letter { data: 0b100_01110 }
    }

    #[fixture]
    pub fn t4() -> Letter {
        Letter { data: 0b100_10011 }
    }

    #[fixture]
    pub fn z4() -> Letter {
        Letter { data: 0b100_11001 }
    }
}

mod try_from {
    use super::*;

    mod usize_char {
        use super::*;

        #[rstest]
        #[case((0, 'A'), Letter{data: 0b000_00000})]
        #[case((0, 'B'), Letter{data: 0b000_00001})]
        #[case((0, 'M'), Letter{data: 0b000_01100})]
        #[case((3, 'N'), Letter{data: 0b011_01101})]
        #[case((4, 'A'), Letter{data: 0b100_00000})]
        #[case((4, 'Z'), Letter{data: 0b100_11001})]
        #[case((0, 'a'), Letter{data: 0b000_00000})]
        #[case((0, 'b'), Letter{data: 0b000_00001})]
        #[case((0, 'm'), Letter{data: 0b000_01100})]
        #[case((3, 'n'), Letter{data: 0b011_01101})]
        #[case((4, 'a'), Letter{data: 0b100_00000})]
        #[case((4, 'z'), Letter{data: 0b100_11001})]
        #[trace]
        fn ok(#[case] input: (usize, char), #[case] output: Letter) {
            let l = assert_ok!(Letter::try_from(input));

            assert_eq!(l, output);
        }

        #[rstest]
        #[case((0, '!'), Error::NotAsciiAlphabetic { c: '!' })]
        #[case((0, 'Á'), Error::NotAsciiAlphabetic { c: 'Á' })]
        #[case((0, '🥴'), Error::NotAsciiAlphabetic { c: '🥴' })]
        #[case((5, 'A'), Error::InvalidPosition { i: 5 })]
        #[case((6, 'A'), Error::InvalidPosition { i: 6 })]
        #[case((255, 'A'), Error::InvalidPosition { i: 255 })]
        #[case((5, '!'), Error::NotAsciiAlphabetic { c: '!' })]
        #[case((6, 'Á'), Error::NotAsciiAlphabetic { c: 'Á' })]
        #[case((255, '🥴'), Error::NotAsciiAlphabetic { c: '🥴' })]
        #[trace]
        fn err(#[case] input: (usize, char), #[case] e: Error) {
            assert_err_eq_x!(Letter::try_from(input), e);
        }

        mod fuzz {
            use super::*;

            proptest! {
                #[test]
                fn lower(i in 0..WORD_LENGTH, c in b'a'..=b'z') {
                    assert_ok!(Letter::try_from((i, c as char)));
                }

                #[test]
                fn upper(i in 0..WORD_LENGTH, c in b'A'..=b'Z') {
                    assert_ok!(Letter::try_from((i, c as char)));
                }
            }
        }
    }
}

#[rstest]
#[case(Letter{data: 0b000_00000}, 0)]
#[case(Letter{data: 0b000_00001}, 1)]
#[case(Letter{data: 0b001_00101}, 5)]
#[case(Letter{data: 0b011_01111}, 15)]
#[case(Letter{data: 0b100_00000}, 0)]
#[trace]
fn ordinal(#[case] input: Letter, #[case] ord: usize) {
    assert_eq!(input.ordinal(), ord);
}

#[rstest]
#[case(Letter{data: 0b00000000}, 0)]
#[case(Letter{data: 0b00000001}, 0)]
#[case(Letter{data: 0b00100101}, 1)]
#[case(Letter{data: 0b01101111}, 3)]
#[case(Letter{data: 0b10000000}, 4)]
#[trace]
fn position(#[case] input: Letter, #[case] pos: usize) {
    assert_eq!(input.position(), pos);
}

#[rstest]
#[case(Letter{data: 0b00000000}, 'A')]
#[case(Letter{data: 0b00000001}, 'B')]
#[case(Letter{data: 0b00001100}, 'M')]
#[case(Letter{data: 0b00011001}, 'Z')]
#[case(Letter{data: 0b00100000}, 'A')]
#[case(Letter{data: 0b01000001}, 'B')]
#[case(Letter{data: 0b10001100}, 'M')]
#[case(Letter{data: 0b01111001}, 'Z')]
#[trace]
fn to_char(#[case] input: Letter, #[case] c: char) {
    assert_eq!(input.to_char(), c);
}

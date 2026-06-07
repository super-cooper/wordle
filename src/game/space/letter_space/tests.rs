use fixtures::*;
use rstest::{
    fixture,
    rstest,
};

use super::*;
use crate::game::letter::tests::fixtures::*;

const FULL_SHARED: SharedCounter = [Saturating(u32::MAX); ALPHABET_LENGTH];
const FULL_EXACT: ExactCounter = [[Saturating(u32::MAX); ALPHABET_LENGTH]; WORD_LENGTH];

impl LetterSpace {
    pub fn mock_one() -> Self {
        let mut space = Self::default();
        space.shared_counter[0] = Saturating(1);
        space.exact_counter[0][0] = Saturating(1);
        space.global_counter = space.shared_counter;

        space
    }

    pub const fn mock_full() -> Self {
        Self {
            global_counter:      FULL_SHARED,
            shared_counter:      FULL_SHARED,
            exact_counter:       FULL_EXACT,
            min_required:        [u8::MAX; ALPHABET_LENGTH],
            max_allowed:         [u8::MAX; ALPHABET_LENGTH],
            forbidden_positions: [u8::MAX; ALPHABET_LENGTH],
        }
    }

    pub fn total_letters(&self) -> u32 {
        self.shared_counter
            .iter()
            .copied()
            .sum::<Saturating<u32>>()
            .0
    }
}

fn make_shared(mut init: SharedCounter, f: impl Fn(&mut SharedCounter)) -> SharedCounter {
    f(&mut init);
    init
}

fn make_exact(mut init: ExactCounter, f: impl Fn(&mut ExactCounter)) -> ExactCounter {
    f(&mut init);
    init
}

mod fixtures {

    use super::*;

    #[fixture]
    pub fn unique_letter_sequence(
        a0: Letter,
        x1: Letter,
        g2: Letter,
        n3: Letter,
        z4: Letter,
    ) -> [Letter; WORD_LENGTH] {
        [a0, x1, g2, n3, z4]
    }
}

#[rstest]
#[case(vec![], SharedCounter::default(), ExactCounter::default())]
#[case(
    vec![a0()],
    make_shared(SharedCounter::default(), |s| s[0].0 = 1),
    make_exact(ExactCounter::default(), |e| e[0][0].0 = 1),
)]
#[case(
    vec![a0(), b0()],
    make_shared(SharedCounter::default(), |s| {s[0].0 = 1; s[1].0 = 1;}),
    make_exact(ExactCounter::default(), |e| {e[0][0].0 = 1; e[0][1].0 = 1;})
)]
#[case(
    vec![a0(), b0(), b0(), n3(), a4(), z4()],
    make_shared(
        SharedCounter::default(),
        |s| {
            s[0].0 = 2;
            s[1].0 = 2;
            s[13].0 = 1;
            s[25].0 = 1;
        },
    ),
    make_exact(
        ExactCounter::default(),
        |e| {
            e[0][0].0 = 1;
            e[0][1].0 = 2;
            e[3][13].0 = 1;
            e[4][0].0 = 1;
            e[4][25].0 = 1;
        },
    )
)]
#[trace]
fn from_iter(
    #[case] input: Vec<Letter>,
    #[case] expected_shared: SharedCounter,
    #[case] expected_exact: ExactCounter,
) {
    let space = LetterSpace::from_iter(input);

    assert_eq!(space.shared_counter, expected_shared);
    assert_eq!(space.global_counter, expected_shared);
    assert_eq!(space.exact_counter, expected_exact);
}

#[rstest]
#[case(LetterSpace::default(), vec![], SharedCounter::default(), ExactCounter::default())]
#[case(
    LetterSpace::default(),
    vec![a0()],
    make_shared(SharedCounter::default(), |s| s[0].0 = 1),
    make_exact(ExactCounter::default(), |e| e[0][0].0 = 1)
)]
#[case(
    LetterSpace::default(),
    vec![a0(), b0()],
    make_shared(SharedCounter::default(), |s| {s[0].0 = 1; s[1].0 = 1;}),
    make_exact(ExactCounter::default(), |e| {e[0][0].0 = 1; e[0][1].0 = 1;})
)]
#[case(
    LetterSpace::default(),
    vec![a0(), b0(), m0(), n3(), a4(), z4()],
    make_shared(
        SharedCounter::default(),
        |s| {
            s[0].0 = 2;
            s[1].0 = 1;
            s[12].0 = 1;
            s[13].0 = 1;
            s[25].0 = 1;
        },
    ),
    make_exact(
        ExactCounter::default(),
        |e| {
            e[0][0].0 = 1;
            e[0][1].0 = 1;
            e[0][12].0 = 1;
            e[3][13].0 = 1;
            e[4][0].0 = 1;
            e[4][25].0 = 1;
        },
    ),
)]
#[case(
    LetterSpace::default(),
    vec![a0(), b0(), a0(), m0(), m0(), z4(), n3(), a4(), a0(), z4()],
    make_shared(
        SharedCounter::default(),
        |s| {
            s[0].0 = 4;
            s[1].0 = 1;
            s[12].0 = 2;
            s[13].0 = 1;
            s[25].0 = 2;
        },
    ),
    make_exact(
        ExactCounter::default(),
        |e| {
            e[0][0].0 = 3;
            e[0][1].0 = 1;
            e[0][12].0 = 2;
            e[3][13].0 = 1;
            e[4][0].0 = 1;
            e[4][25].0 = 2;
        },
    ),
)]
#[case(
    LetterSpace::mock_one(),
    vec![a0(), b0(), a0(), m0(), m0(), z4(), n3(), a4(), a0(), z4()],
    make_shared(
        SharedCounter::default(),
        |s| {
            s[0].0 = 5;
            s[1].0 = 1;
            s[12].0 = 2;
            s[13].0 = 1;
            s[25].0 = 2;
        },
    ),
    make_exact(
        ExactCounter::default(),
        |e| {
            e[0][0].0 = 4;
            e[0][1].0 = 1;
            e[0][12].0 = 2;
            e[3][13].0 = 1;
            e[4][0].0 = 1;
            e[4][25].0 = 2;
        },
    ),
)]
#[case(LetterSpace::mock_full(), vec![], FULL_SHARED, FULL_EXACT)]
#[case(LetterSpace::mock_full(), vec![a0(), b0()], FULL_SHARED, FULL_EXACT)]
#[trace]
fn add(
    #[notrace]
    #[case]
    mut space: LetterSpace,
    #[case] input: Vec<Letter>,
    #[case] expected_shared: SharedCounter,
    #[case] expected_exact: ExactCounter,
) {
    let expected_min = space.min_required;
    let expected_max = space.max_allowed;
    let expected_forbidden = space.forbidden_positions;
    let expected_global = space.global_counter;

    for letter in input {
        space.add(letter);
    }

    // Assert counters have been correctly incremented
    assert_eq!(space.shared_counter, expected_shared);
    assert_eq!(space.exact_counter, expected_exact);

    // Assert nothing unexpected was modified
    assert_eq!(space.min_required, expected_min);
    assert_eq!(space.max_allowed, expected_max);
    assert_eq!(space.forbidden_positions, expected_forbidden);
    assert_eq!(space.global_counter, expected_global);
}

#[rstest]
#[case(LetterSpace::default(), vec![], SharedCounter::default())]
#[case(
    LetterSpace::default(),
    vec![a0()],
    make_shared(SharedCounter::default(), |s| s[0].0 = 1),
)]
#[case(
    LetterSpace::default(),
    vec![a0(), b0()],
    make_shared(SharedCounter::default(), |s| {s[0].0 = 1; s[1].0 = 1;}),
)]
#[case(
    LetterSpace::default(),
    vec![a0(), b0(), m0(), n3(), a4(), z4()],
    make_shared(
        SharedCounter::default(),
        |s| {
            s[0].0 = 2;
            s[1].0 = 1;
            s[12].0 = 1;
            s[13].0 = 1;
            s[25].0 = 1;
        },
    ),
)]
#[case(
    LetterSpace::default(),
    vec![a0(), b0(), a0(), m0(), m0(), z4(), n3(), a4(), a0(), z4()],
    make_shared(
        SharedCounter::default(),
        |s| {
            s[0].0 = 4;
            s[1].0 = 1;
            s[12].0 = 2;
            s[13].0 = 1;
            s[25].0 = 2;
        },
    ),
)]
#[case(
    LetterSpace::mock_one(),
    vec![a0(), b0(), a0(), m0(), m0(), z4(), n3(), a4(), a0(), z4()],
    make_shared(
        SharedCounter::default(),
        |s| {
            s[0].0 = 5;
            s[1].0 = 1;
            s[12].0 = 2;
            s[13].0 = 1;
            s[25].0 = 2;
        },
    ),
)]
#[case(LetterSpace::mock_full(), vec![], FULL_SHARED)]
#[case(LetterSpace::mock_full(), vec![a0(), b0()], FULL_SHARED)]
#[trace]
fn add_global(
    #[notrace]
    #[case]
    mut space: LetterSpace,
    #[case] input: Vec<Letter>,
    #[case] expected_global: SharedCounter,
) {
    let expected_min = space.min_required;
    let expected_max = space.max_allowed;
    let expected_forbidden = space.forbidden_positions;
    let expected_shared = space.shared_counter;
    let expected_exact = space.exact_counter;

    for letter in input {
        space.add_global(letter);
    }

    // Assert counter has been correctly incremented
    assert_eq!(space.global_counter, expected_global);

    // Assert nothing unexpected was modified
    assert_eq!(space.min_required, expected_min);
    assert_eq!(space.max_allowed, expected_max);
    assert_eq!(space.forbidden_positions, expected_forbidden);
    assert_eq!(space.shared_counter, expected_shared);
    assert_eq!(space.exact_counter, expected_exact);
}

#[rstest]
#[case(LetterSpace::default(), vec![], SharedCounter::default(), ExactCounter::default())]
#[case(LetterSpace::mock_one(), vec![a0()], SharedCounter::default(), ExactCounter::default())]
#[case(
    LetterSpace::mock_full(),
    vec![a0(), b0()],
    make_shared(FULL_SHARED, |s| {s[0].0 = u32::MAX - 1; s[1].0 = u32::MAX - 1;}),
    make_exact(FULL_EXACT, |e| {e[0][0].0 = u32::MAX - 1; e[0][1].0 = u32::MAX - 1;})
)]
#[case(
    LetterSpace::mock_full(),
    vec![a0(), b0(), m0(), n3(), a4(), z4()],
    make_shared(
        FULL_SHARED,
        |s| {
            s[0].0 = u32::MAX - 2;
            s[1] .0 = u32::MAX - 1;
            s[12].0 = u32::MAX - 1;
            s[13].0 = u32::MAX - 1;
            s[25].0 = u32::MAX - 1;
        },
    ),
    make_exact(
        FULL_EXACT,
        |e| {
            e[0][0].0 = u32::MAX - 1;
            e[0][1].0 = u32::MAX - 1;
            e[0][12].0 = u32::MAX - 1;
            e[3][13].0 = u32::MAX - 1;
            e[4][0].0 = u32::MAX - 1;
            e[4][25].0 = u32::MAX - 1;
        },
    ),
)]
#[case(
    LetterSpace::mock_full(),
    vec![a0(), b0(), a0(), m0(), m0(), z4(), n3(), a4(), a0(), z4()],
    make_shared(
        FULL_SHARED,
        |s| {
            s[0].0 = u32::MAX - 4;
            s[1].0 = u32::MAX - 1;
            s[12].0 = u32::MAX - 2;
            s[13].0 = u32::MAX - 1;
            s[25].0 = u32::MAX - 2;
        },
    ),
    make_exact(
        FULL_EXACT,
        |e| {
            e[0][0].0 = u32::MAX - 3;
            e[0][1].0 = u32::MAX - 1;
            e[0][12].0 = u32::MAX - 2;
            e[3][13].0 = u32::MAX - 1;
            e[4][0].0 = u32::MAX - 1;
            e[4][25].0 = u32::MAX -  2;
        },
    ),
)]
#[case(LetterSpace::default(), vec![a0(), b0()], SharedCounter::default(), ExactCounter::default())]
#[trace]
fn remove(
    #[notrace]
    #[case]
    mut space: LetterSpace,
    #[case] input: Vec<Letter>,
    #[case] expected_shared: SharedCounter,
    #[case] expected_exact: ExactCounter,
) {
    let expected_min = space.min_required;
    let expected_max = space.max_allowed;
    let expected_forbidden = space.forbidden_positions;
    let expected_global = space.global_counter;

    for letter in input {
        space.remove(letter);
    }

    // Assert counters have been correctly decremented
    assert_eq!(space.shared_counter, expected_shared);
    assert_eq!(space.exact_counter, expected_exact);

    // Assert nothing unexpected was modified
    assert_eq!(space.min_required, expected_min);
    assert_eq!(space.max_allowed, expected_max);
    assert_eq!(space.forbidden_positions, expected_forbidden);
    assert_eq!(space.global_counter, expected_global);
}

#[rstest]
#[case(LetterSpace::default(), a0(), 0)]
#[case(LetterSpace::default(), b0(), 0)]
#[case(LetterSpace::default(), n3(), 0)]
#[case(LetterSpace::mock_one(), a0(), 1)]
#[case(LetterSpace::mock_one(), a4(), 1)]
#[case(LetterSpace::mock_one(), b0(), 0)]
#[case(LetterSpace::mock_one(), n3(), 0)]
#[case(LetterSpace::mock_full(), a0(), u32::MAX)]
#[case(LetterSpace::mock_full(), b0(), u32::MAX)]
#[case(LetterSpace::mock_full(), n3(), u32::MAX)]
#[trace]
fn global_count(#[case] space: LetterSpace, #[case] letter: Letter, #[case] expected: u32) {
    assert_eq!(space.global_count(letter), expected);
}

#[rstest]
#[case(LetterSpace::default(), a0(), 0)]
#[case(LetterSpace::default(), b0(), 0)]
#[case(LetterSpace::default(), n3(), 0)]
#[case(LetterSpace::mock_one(), a0(), 1)]
#[case(LetterSpace::mock_one(), a4(), 1)]
#[case(LetterSpace::mock_one(), b0(), 0)]
#[case(LetterSpace::mock_one(), n3(), 0)]
#[case(LetterSpace::mock_full(), a0(), u32::MAX)]
#[case(LetterSpace::mock_full(), b0(), u32::MAX)]
#[case(LetterSpace::mock_full(), n3(), u32::MAX)]
#[trace]
fn shared_count(#[case] space: LetterSpace, #[case] letter: Letter, #[case] expected: u32) {
    assert_eq!(space.shared_count(letter), expected);
}

#[rstest]
#[case(LetterSpace::default(), a0(), 0)]
#[case(LetterSpace::default(), b0(), 0)]
#[case(LetterSpace::default(), n3(), 0)]
#[case(LetterSpace::mock_one(), a0(), 1)]
#[case(LetterSpace::mock_one(), a4(), 0)]
#[case(LetterSpace::mock_one(), b0(), 0)]
#[case(LetterSpace::mock_one(), n3(), 0)]
#[case(LetterSpace::mock_full(), a0(), u32::MAX)]
#[case(LetterSpace::mock_full(), b0(), u32::MAX)]
#[case(LetterSpace::mock_full(), n3(), u32::MAX)]
#[trace]
fn exact_count(#[case] space: LetterSpace, #[case] letter: Letter, #[case] expected: u32) {
    assert_eq!(space.exact_count(letter), expected);
}

mod validate {
    use super::*;

    #[rstest]
    fn forbidden(
        a0: Letter,
        b0: Letter,
        x1: Letter,
        g2: Letter,
        n3: Letter,
        t3: Letter,
        a4: Letter,
    ) {
        let mut space = LetterSpace::default();
        // A0 is forbidden
        space.forbidden_positions[0] = 0b00000001;
        // N3 is forbidden
        space.forbidden_positions[13] = 0b00001000;

        let has_a0 = [a0, x1, g2, t3, a4];
        let has_n3 = [b0, x1, g2, n3, a4];
        let has_both = [a0, x1, g2, n3, a4];

        assert!(
            !space.validate(has_a0),
            "A0 was forbidden but considered valid"
        );
        assert!(
            !space.validate(has_n3),
            "N3 was forbidden but considered valid"
        );
        assert!(
            !space.validate(has_both),
            "A0 and N3 were forbidden but considered valid"
        );
    }

    mod min_max {
        use super::*;

        #[rstest]
        fn min(a0: Letter, x1: Letter, g2: Letter, n3: Letter, a4: Letter) {
            let mut space = LetterSpace::default();
            // Min required 'N's is 3
            space.min_required[13] = 3;

            assert!(
                !space.validate([a0, x1, g2, n3, a4]),
                "Not enough 'N's was valid"
            );
        }

        #[rstest]
        fn max(a0: Letter, x1: Letter, g2: Letter, n3: Letter, z4: Letter) {
            let mut space = LetterSpace::default();
            // Max allowed 'A's is 0
            space.max_allowed[0] = 0;

            assert!(
                !space.validate([a0, x1, g2, n3, z4]),
                "Too many 'A's was valid"
            );
        }
    }

    #[rstest]
    fn forbidden_and_min_max(
        a0: Letter,
        b0: Letter,
        x1: Letter,
        g2: Letter,
        n3: Letter,
        t3: Letter,
        a4: Letter,
    ) {
        let mut space = LetterSpace::default();
        // A0 is forbidden
        space.forbidden_positions[0] = 0b00000001;
        // N3 is forbidden
        space.forbidden_positions[13] = 0b00001000;
        // Max allowed 'A's is 0
        space.max_allowed[0] = 0;
        // Min required 'N's is 3
        space.min_required[13] = 3;

        let has_a0 = [a0, x1, g2, t3, a4];
        let has_n3 = [b0, x1, g2, n3, a4];
        let has_both = [a0, x1, g2, n3, a4];

        assert!(
            !space.validate(has_a0),
            "A0 was forbidden but considered valid"
        );
        assert!(
            !space.validate(has_n3),
            "N3 was forbidden but considered valid"
        );
        assert!(
            !space.validate(has_both),
            "A0 and N3 were forbidden but considered valid"
        );
    }

    #[rstest]
    fn no_constraints(
        a0: Letter,
        b0: Letter,
        x1: Letter,
        g2: Letter,
        n3: Letter,
        t3: Letter,
        a4: Letter,
    ) {
        let space = LetterSpace::default();

        let has_a0 = [a0, x1, g2, t3, a4];
        let has_n3 = [b0, x1, g2, n3, a4];
        let has_both = [a0, x1, g2, n3, a4];

        assert!(
            space.validate(has_a0),
            "A0 was not valid despite no constraints"
        );
        assert!(
            space.validate(has_n3),
            "N3 was not valid despite no constraints"
        );
        assert!(
            space.validate(has_both),
            "A0 and N3 were not valid despite no constraints"
        );
    }
}

mod update {
    use super::*;

    mod single {
        use super::*;

        #[rstest]
        fn green(unique_letter_sequence: [Letter; WORD_LENGTH]) {
            let mut space = LetterSpace::default();
            let colors = [Color::Green; WORD_LENGTH];

            space.update(unique_letter_sequence, colors);

            for letter in unique_letter_sequence {
                let ordinal = letter.ordinal();
                assert_eq!(
                    space.min_required[ordinal], 1,
                    "min_required was not raised for green {letter}",
                );

                for other in 0..WORD_LENGTH {
                    let position = letter.position() as u8;
                    let mask = 1u8 << position;
                    let forbidden = space.forbidden_positions[other] & mask;
                    let mut expected = mask;

                    if other == ordinal {
                        expected = 0;
                    };

                    assert_eq!(
                        forbidden, expected,
                        "ordinal {other}'s forbidden status was unexpected on green {letter}"
                    );
                }
            }
        }

        #[rstest]
        fn yellow(unique_letter_sequence: [Letter; WORD_LENGTH]) {
            let mut space = LetterSpace::default();
            let colors = [Color::Yellow; WORD_LENGTH];

            space.update(unique_letter_sequence, colors);

            for letter in unique_letter_sequence {
                let ordinal = letter.ordinal();
                assert_eq!(
                    space.min_required[ordinal], 1,
                    "min_required was not raised for yellow {letter}"
                );

                for other in 0..WORD_LENGTH {
                    let position = letter.position() as u8;
                    let mask = 1u8 << position;
                    let forbidden = space.forbidden_positions[other] & mask;
                    let mut expected = 0;

                    if other == ordinal {
                        expected = mask;
                    };

                    assert_eq!(
                        forbidden, expected,
                        "ordinal {other}'s forbidden status was unexpected on yellow {letter}"
                    );
                }
            }
        }

        #[rstest]
        fn gray(unique_letter_sequence: [Letter; WORD_LENGTH]) {
            let mut space = LetterSpace::default();
            let colors = [Color::Gray; WORD_LENGTH];

            space.update(unique_letter_sequence, colors);

            for letter in unique_letter_sequence {
                let ordinal = letter.ordinal();
                assert_eq!(
                    space.max_allowed[ordinal], 0,
                    "max_allowed was not clamped to 0 for gray {letter}"
                );

                for other in 0..WORD_LENGTH {
                    let position = letter.position() as u8;
                    let mask = 1u8 << position;
                    let forbidden = space.forbidden_positions[other] & mask;
                    let mut expected = 0;

                    if other == ordinal {
                        expected = mask;
                    };

                    assert_eq!(
                        forbidden, expected,
                        "ordinal {other}'s forbidden status was unexpected on gray {letter}"
                    );
                }
            }
        }
    }

    mod min_required {
        use super::*;

        #[rstest]
        fn not_lowered(
            unique_letter_sequence: [Letter; WORD_LENGTH],
            b0: Letter,
            t3: Letter,
            z4: Letter,
        ) {
            let mut space = LetterSpace::default();
            let colors = [
                Color::Yellow,
                Color::Green,
                Color::Green,
                Color::Yellow,
                Color::Gray,
            ];

            space.update(unique_letter_sequence, colors);

            let letters_2 = [
                b0,
                unique_letter_sequence[1],
                unique_letter_sequence[2],
                t3,
                z4,
            ];
            let colors_2 = [
                Color::Gray,
                Color::Green,
                Color::Green,
                Color::Gray,
                Color::Gray,
            ];

            space.update(letters_2, colors_2);

            assert_eq!(
                space.min_required[0], 1,
                "min_required changed for 'A' when we had a Yellow 'A' on prior guess"
            );
        }

        #[rstest]
        fn stable(
            unique_letter_sequence: [Letter; WORD_LENGTH],
            b0: Letter,
            a3: Letter,
            a4: Letter,
        ) {
            let mut space = LetterSpace::default();
            let colors = [
                Color::Yellow,
                Color::Green,
                Color::Green,
                Color::Yellow,
                Color::Gray,
            ];

            space.update(unique_letter_sequence, colors);

            let letters_3 = [
                b0,
                unique_letter_sequence[1],
                unique_letter_sequence[2],
                a3,
                a4,
            ];
            let colors_3 = [
                Color::Gray,
                Color::Green,
                Color::Green,
                Color::Gray,
                Color::Green,
            ];

            space.update(letters_3, colors_3);

            assert_eq!(
                space.min_required[0], 1,
                "min_required was changed even though we didn't get more Yellow or Green 'A's"
            );
        }
    }

    mod max_allowed {
        use super::*;

        #[rstest]
        fn clamp_one(a0: Letter, x1: Letter, g2: Letter, n3: Letter, a4: Letter) {
            let mut space = LetterSpace::default();
            let letters = [a0, x1, g2, n3, a4];
            let colors = [
                Color::Yellow,
                Color::Green,
                Color::Green,
                Color::Green,
                Color::Gray,
            ];

            space.update(letters, colors);

            assert_eq!(
                space.max_allowed[0], 1,
                "'A' was not clamped to 1 with a Yellow and Gray in the same sequence"
            );
        }

        #[rstest]
        fn clamp_two(a0: Letter, x1: Letter, g2: Letter, a3: Letter, a4: Letter) {
            let mut space = LetterSpace::default();
            let letters = [a0, x1, g2, a3, a4];
            let colors = [
                Color::Yellow,
                Color::Gray,
                Color::Gray,
                Color::Yellow,
                Color::Gray,
            ];

            space.update(letters, colors);

            assert_eq!(
                space.max_allowed[0], 2,
                "'A' was not clamped to 2 with two Yellows and a Gray in the same sequence"
            );
        }

        #[rstest]
        fn not_modified(unique_letter_sequence: [Letter; WORD_LENGTH]) {
            let mut space = LetterSpace::default();
            let colors = [
                Color::Yellow,
                Color::Green,
                Color::Yellow,
                Color::Green,
                Color::Green,
            ];

            space.update(unique_letter_sequence, colors);

            assert_eq!(
                space.max_allowed, [WORD_LENGTH as u8; ALPHABET_LENGTH],
                "max_allowed was modified without any gray letters"
            );
        }
    }
}

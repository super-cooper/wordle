use mockall::mock;
use mocks::*;
use rstest::rstest;

use super::*;
use crate::game::word::tests::fixtures::*;

mod mocks {
    use super::*;

    mock! {
        pub Scorer {}
        impl Scorer for Scorer {
            fn score(&self, word: Word) -> ScoreType;
            fn break_tie(&self, word1: Word, word2: Word) -> Ordering;
        }
    }
}

mod score_mode {
    use super::*;

    mod n {
        use super::*;

        #[rstest]
        fn same() {
            let default = ScoreMode::Default { n: 0 };
            let unique = ScoreMode::UniqueOnly { n: 0 };

            assert_eq!(default.n(), unique.n());
        }

        #[rstest]
        fn different() {
            let default = ScoreMode::Default { n: 0 };
            let unique = ScoreMode::UniqueOnly { n: 1 };

            assert_ne!(default.n(), unique.n());
        }
    }
}

mod score {
    use super::*;

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(9001)]
    #[trace]
    fn new(valid: Word, #[case] s: ScoreType) {
        let score = Score::new(valid, s);

        assert_eq!(score.word, valid);
        assert_eq!(score.score, s);
    }

    #[rstest]
    fn word(valid: Word, grunt: Word) {
        let score = Score {
            word:  valid,
            score: 0,
        };

        assert_eq!(score.word(), "VALID");

        let score = Score::new(grunt, 100);

        assert_eq!(score.word(), "GRUNT");
    }

    #[rstest]
    #[case(0, 0, true)]
    #[case(0, 1, false)]
    #[case(1, 0, false)]
    #[case(35, 35, true)]
    #[case(35, 31, false)]
    #[case(32, 37, false)]
    #[trace]
    fn eq(
        valid: Word,
        grunt: Word,
        #[case] score_1: ScoreType,
        #[case] score_2: ScoreType,
        #[case] expected: bool,
    ) {
        let s1 = Score {
            word:  valid,
            score: score_1,
        };
        let s2 = Score {
            word:  grunt,
            score: score_2,
        };

        assert_eq!(s1.eq(&s2), expected);
    }

    #[rstest]
    #[case(0, 0, Ordering::Equal)]
    #[case(0, 1, Ordering::Less)]
    #[case(1, 0, Ordering::Greater)]
    #[case(35, 35, Ordering::Equal)]
    #[case(35, 31, Ordering::Greater)]
    #[case(32, 37, Ordering::Less)]
    #[trace]
    fn ord(
        valid: Word,
        grunt: Word,
        #[case] score_1: ScoreType,
        #[case] score_2: ScoreType,
        #[case] expected: Ordering,
    ) {
        let s1 = Score {
            word:  valid,
            score: score_1,
        };
        let s2 = Score {
            word:  grunt,
            score: score_2,
        };

        assert_eq!(s1.cmp(&s2), expected);
    }
}

mod compute {
    use super::*;

    #[rstest]
    #[case(0, ScoreMode::Default{ n: 0 }, 1, 0)]
    #[case(0, ScoreMode::Default{ n: 1 }, 1, 0)]
    #[case(10, ScoreMode::Default{ n: 0 }, 1, 10)]
    #[case(10, ScoreMode::Default{ n: 1 }, 1, 10)]
    #[case(0, ScoreMode::UniqueOnly{ n: 0 }, 0, 0)]
    #[case(0, ScoreMode::UniqueOnly{ n: 1 }, 0, 0)]
    #[case(10, ScoreMode::UniqueOnly{ n: 0 }, 0, 0)]
    #[case(10, ScoreMode::UniqueOnly{ n: 1 }, 0, 0)]
    #[trace]
    fn not_unique(
        hello: Word,
        #[case] base: ScoreType,
        #[case] mode: ScoreMode,
        #[case] scorer_calls: usize,
        #[case] expected: ScoreType,
    ) {
        let mut scorer = MockScorer::new();
        scorer
            .expect_score()
            .times(scorer_calls)
            .returning(move |_| base);

        let score = super::compute(&scorer, hello, mode);

        assert_eq!(score.word, hello);
        assert_eq!(score.score, expected);
    }

    #[rstest]
    #[case(0, ScoreMode::Default{ n: 0 }, 0)]
    #[case(0, ScoreMode::Default{ n: 1 }, 0)]
    #[case(10, ScoreMode::Default{ n: 0 }, 10)]
    #[case(10, ScoreMode::Default{ n: 1 }, 10)]
    #[case(0, ScoreMode::UniqueOnly{ n: 0 }, 0)]
    #[case(0, ScoreMode::UniqueOnly{ n: 1 }, 0)]
    #[case(10, ScoreMode::UniqueOnly{ n: 0 }, 10)]
    #[case(10, ScoreMode::UniqueOnly{ n: 1 }, 10)]
    #[trace]
    fn unique(
        valid: Word,
        #[case] base: ScoreType,
        #[case] mode: ScoreMode,
        #[case] expected: ScoreType,
    ) {
        let mut scorer = MockScorer::new();
        scorer.expect_score().times(1).returning(move |_| base);

        let score = super::compute(&scorer, valid, mode);

        assert_eq!(score.word, valid);
        assert_eq!(score.score, expected);
    }
}

#[rstest]
#[case(Ordering::Greater)]
#[case(Ordering::Equal)]
#[case(Ordering::Less)]
fn break_tie(hello: Word, valid: Word, #[case] expected: Ordering) {
    let mut scorer = MockScorer::new();
    scorer
        .expect_break_tie()
        .times(1)
        .returning(move |_, _| expected);

    let ordering = super::break_tie(&scorer, hello, valid);

    assert_eq!(ordering, expected);
}

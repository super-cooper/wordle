use assertables::{
    assert_contains,
    assert_err_eq_x,
    assert_is_empty,
    assert_len_eq_x,
    assert_ok,
};
use fixtures::*;
use rstest::{
    fixture,
    rstest,
};

use super::*;
use crate::game::space::tests::fixtures::*;
use crate::game::word::tests::fixtures::*;

pub mod fixtures {
    use super::*;

    #[fixture]
    pub fn test_round(test_word_space: WordSpace, valid: Word) -> Round {
        Round {
            guesses: ArrayView::default(),
            answer:  valid,
            words:   test_word_space,
        }
    }
}

mod new {
    use super::*;

    #[rstest]
    fn base(test_word_space: WordSpace, valid: Word) {
        let round = Round::new(test_word_space, valid);

        assert_eq!(&round.answer, &valid);
        assert_len_eq_x!(round.guesses, 0);
        assert_len_eq_x!(&round.words, 3);
        assert_contains!(&round.words, valid);
    }

    #[rstest]
    fn include_answer(test_word_space: WordSpace, grunt: Word) {
        let round = Round::new(test_word_space, grunt);

        assert_eq!(&round.answer, &grunt);
        assert_len_eq_x!(round.guesses, 0);
        assert_len_eq_x!(&round.words, 4);
        assert_contains!(&round.words, grunt);
    }
}

mod is_over {
    use super::*;

    #[rstest]
    fn n_guesses(mut test_round: Round, hello: Word) {
        for _ in 0..N_GUESSES {
            assert!(!test_round.is_over());
            test_round.guesses.push(hello);
        }

        assert!(test_round.is_over());
    }

    #[rstest]
    fn right_answer(mut test_round: Round, grunt: Word) {
        test_round.answer = grunt;
        test_round.guesses.push(grunt);

        assert!(test_round.is_over());
    }
}

mod guess {
    use super::*;

    #[rstest]
    fn after_answer(mut test_round: Round, valid_answer: String) {
        assert_ok!(test_round.guess(&valid_answer));

        let res = test_round.guess("GREMO");

        assert_err_eq_x!(
            res,
            crate::Error::Player {
                e: crate::InternalError {
                    e: Error::TooManyGuesses,
                },
            }
        );
    }

    #[rstest]
    fn incorrect(mut test_round: Round, grunt_str: &str) {
        let res = test_round.guess(grunt_str);

        assert_ok!(res);
    }

    #[rstest]
    fn incorrect_amount(mut test_round: Round, grunt_str: &str) {
        for i in 0..crate::N_GUESSES {
            let res = test_round.guess(grunt_str);

            assert_ok!(res, "Incorrect guess {i} was an error");
        }

        let res = test_round.guess(grunt_str);

        assert_err_eq_x!(
            res,
            crate::Error::Player {
                e: crate::InternalError {
                    e: Error::TooManyGuesses,
                },
            }
        );
    }

    #[rstest]
    fn correct(mut test_round: Round, valid_answer: String) {
        let res = test_round.guess(&valid_answer);

        assert_ok!(res);
    }

    #[rstest]
    fn correct_after_incorrect(mut test_round: Round, valid_answer: String) {
        let res = test_round.guess("BRUMT");

        assert_ok!(res);

        let res = test_round.guess(&valid_answer);

        assert_ok!(res);
    }
}

mod best {

    use super::*;

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(2)]
    #[case(4)]
    #[case(6)]
    #[trace]
    fn base(
        valid: Word,
        crunk: Word,
        grunt: Word,
        munch: Word,
        gaunt: Word,
        halid: Word,
        #[case] n: usize,
    ) {
        let rd = Round {
            guesses: ArrayView::default(),
            answer:  grunt,
            words:   WordSpace::test(vec![valid, crunk, munch, grunt, gaunt, halid]),
        };

        let best = rd
            .best(ScoreMode::Default { n })
            .into_iter()
            .collect::<Vec<_>>();

        // Ensure we got back `n` scores
        assert_len_eq_x!(&best, n);

        // Totals:
        // v(1) + a(3) + l(2) + i(2) + d(2) = 10
        // c(2) + r(2) + u(4) + n(4) + k(1) = 13
        // g(2) + r(2) + u(4) + n(4) + t(2) = 14
        // m(1) + u(4) + n(4) + c(2) + h(2) = 13
        // g(2) + a(3) + u(4) + n(4) + t(2) = 15
        // h(2) + a(3) + l(2) + i(2) + d(2) = 11
        //
        // Tiebreakers:
        // v(1) + a(3) + l(2) + i(2) + d(2) = 10
        // c(1) + r(2) + u(3) + n(3) + k(1) = 10
        // g(2) + r(2) + u(3) + n(3) + t(2) = 12
        // m(1) + u(1) + n(1) + c(1) + h(1) =  5
        // g(2) + a(3) + u(3) + n(3) + t(2) = 13
        // h(1) + a(3) + l(2) + i(2) + d(2) = 10
        let expected_scores = [15, 14, 13, 13, 11, 10];
        let expected_words = [gaunt, grunt, crunk, munch, halid, valid];

        // Ensure scores are sorted
        for i in 0..n {
            assert_eq!(best[i].score(), expected_scores[i], "{best:?}");
            assert_eq!(best[i].word_data(), expected_words[i], "{best:?}");
        }
    }

    #[rstest]
    fn zero_words(hello: Word) {
        let rd = Round {
            guesses: ArrayView::default(),
            answer:  hello,
            words:   WordSpace::test(Vec::new()),
        };

        let best = rd
            .best(ScoreMode::Default { n: 1000 })
            .into_iter()
            .collect::<Vec<_>>();

        assert_is_empty!(&best);
    }

    #[rstest]
    fn zero_n(hello: Word, crunk: Word) {
        let rd = Round {
            guesses: ArrayView::default(),
            answer:  hello,
            words:   WordSpace::test(vec![hello, crunk]),
        };

        let best = rd
            .best(ScoreMode::Default { n: 0 })
            .into_iter()
            .collect::<Vec<_>>();

        assert_is_empty!(&best);
    }

    #[rstest]
    fn one_word(hello: Word) {
        let rd = Round {
            guesses: ArrayView::default(),
            answer:  hello,
            words:   WordSpace::test(vec![hello]),
        };

        let best = rd
            .best(ScoreMode::Default { n: 1000 })
            .into_iter()
            .collect::<Vec<_>>();

        assert_len_eq_x!(&best, 1);
        assert_eq!(best[0].score(), 0);
        assert_eq!(best[0].word_data(), hello);
    }

    #[rstest]
    fn one_n(grunt: Word, glunk: Word, gaunt: Word, llama: Word) {
        let rd = Round {
            guesses: ArrayView::default(),
            answer:  grunt,
            words:   WordSpace::test(vec![grunt, glunk, gaunt, llama]),
        };

        let best = rd
            .best(ScoreMode::Default { n: 1 })
            .into_iter()
            .collect::<Vec<_>>();

        eprintln!("{best:?}");

        assert_len_eq_x!(&best, 1);
        // g(3) + r(1) + u(3) + n(3) + t(2) = 12
        // g(3) + l(3) + u(3) + n(3) + k(1) = 13
        // g(3) + a(3) + u(3) + n(3) + t(2) = 14
        // l(3) + l(0) + a(3) + m(1) + a(0) =  7
        assert_eq!(best[0].word_data(), gaunt);
        assert_eq!(best[0].score(), 14);
    }
}

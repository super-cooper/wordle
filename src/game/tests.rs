use std::collections::HashSet;
use std::fmt::{
    Display,
    Formatter,
};
use std::{
    assert_matches,
    vec,
};

use assertables::{
    assert_contains,
    assert_len_eq,
    assert_len_eq_x,
    assert_not_empty,
    assert_ok,
};
use fixtures::*;
use mocks::*;
use rstest::{
    fixture,
    rstest,
};

use super::*;
use crate::InternalError;
use crate::game::space::tests::fixtures::*;
use crate::game::word::tests::fixtures::*;

mod mocks {

    use super::*;

    #[derive(Clone, Copy, Debug)]
    pub struct MockDate;

    #[derive(Clone, Debug)]
    pub struct MockErr;

    impl Display for MockErr {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "")
        }
    }

    impl std::error::Error for MockErr {}

    pub struct MockClient {
        pub answer:    std::result::Result<String, MockErr>,
        pub word_list: std::result::Result<Vec<String>, MockErr>,
    }

    impl Client for MockClient {
        type Date = MockDate;
        type Error = MockErr;

        fn fetch_answer(&self, _: Self::Date) -> std::result::Result<String, Self::Error> {
            self.answer.clone()
        }

        fn fetch_words(
            &self,
        ) -> std::result::Result<impl Iterator<Item = String> + 'static, Self::Error> {
            self.word_list.clone().map(Vec::into_iter)
        }
    }
}

pub mod fixtures {

    use super::*;

    #[fixture]
    pub fn valid_wordle(test_word_space: WordSpace) -> Wordle {
        Wordle {
            words: test_word_space,
        }
    }
}

mod from_local {
    use super::*;

    #[rstest]
    #[case(valid_strings())]
    #[case(
        ["AAAAA", "BBBBB", "ABCDE", "fooba", "wAbbA", "grorl ", "   smorg"]
            .map(String::from)
            .to_vec()
    )]
    #[case(vec![])]
    #[trace]
    fn valid_words(#[case] words: Vec<String>) {
        let game = assert_ok!(Wordle::from_list(words.clone()));

        assert_len_eq!(game.words, words);
    }

    #[rstest]
    #[case(vec![""])]
    #[case(vec!["omgwtfbbq"])]
    #[case(vec!["foo"])]
    #[case(vec!["foo!!"])]
    #[case(vec!["oh my"])]
    #[case(vec!["VALID", "INVALID", "WOPAH"])]
    #[trace]
    fn invalid_words(#[case] words: Vec<&str>) {
        let game = Wordle::from_list(words.into_iter());

        assert_matches!(
            game,
            Err(crate::Error::Fatal {
                e: InternalError {
                    e: Error::InvalidWord { .. },
                },
            }),
        );
    }
}

mod from_online {
    use super::*;

    #[rstest]
    fn ok(valid_strings: Vec<String>, valid_answer: String) {
        let client = MockClient {
            answer:    Ok(valid_answer),
            word_list: Ok(valid_strings.to_vec()),
        };

        let game = assert_ok!(Wordle::from_client(&client));

        assert_len_eq!(game.words, valid_strings);
    }

    #[rstest]
    fn error(valid_answer: String) {
        let client = MockClient {
            answer:    Ok(valid_answer),
            word_list: Err(MockErr),
        };
        let game = Wordle::from_client(&client);

        assert_matches!(
            game,
            Err(crate::Error::Safe {
                e: InternalError {
                    e: Error::Network { .. },
                },
            }),
        );
    }

    #[rstest]
    fn invalid_words(valid_answer: String) {
        let client = MockClient {
            answer:    Ok(valid_answer),
            word_list: Ok(["HELLO", "INVALID"].map(String::from).to_vec()),
        };

        let res = Wordle::from_client(&client);

        assert_matches!(
            res,
            Err(crate::Error::Fatal {
                e: InternalError {
                    e: Error::InvalidWord { .. },
                },
            }),
        );
    }
}

mod play {
    use super::*;

    #[rstest]
    fn normal(valid_strings: Vec<String>, valid_answer: String, valid_wordle: Wordle) {
        let results = valid_wordle
            .play(valid_answer.clone())
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();

        assert_not_empty!(&results);

        assert_eq!(
            results.last().unwrap().clone(),
            valid_answer,
            "Last result was not answer"
        );

        for result in results.iter() {
            assert_contains!(valid_strings, result);
        }

        let expected_len = results.len();
        let unique_results = HashSet::<_>::from_iter(results);

        assert_len_eq_x!(unique_results, expected_len, "Got duplicate results");
    }

    #[rstest]
    fn bad_answer(valid_wordle: Wordle) {
        let bad_answer = "NYUHUH";
        // The map is just to change the type into something Debug
        let result = valid_wordle.play(bad_answer).map(|_| Vec::<String>::new());

        assert_matches!(
            result,
            Err(crate::Error::Fatal {
                e: InternalError {
                    e: Error::InvalidWord { .. },
                },
            }),
        );
    }
}

mod play_date {
    use super::*;

    #[rstest]
    fn normal(valid_answer: String, valid_wordle: Wordle) {
        let client = MockClient {
            answer:    Ok(valid_answer.clone()),
            // An invalid word list is set here to make sure it doesn't get used
            word_list: Ok(vec![String::from("invalid")]),
        };
        let answers = valid_wordle
            .play_date(&client, MockDate)
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();

        assert_not_empty!(&answers);

        assert_eq!(answers.last().unwrap(), &valid_answer);
    }
}

mod round {
    use super::*;

    #[rstest]
    fn ok(valid_answer: String, valid_wordle: Wordle) {
        assert_ok!(valid_wordle.round(valid_answer));
    }

    #[rstest]
    fn bad_answer(valid_wordle: Wordle) {
        let res = valid_wordle.round("invalid");

        assert_matches!(
            res,
            Err(crate::Error::Fatal {
                e: InternalError {
                    e: Error::InvalidWord { .. },
                },
            })
        );
    }
}

mod round_date {
    use super::*;

    #[rstest]
    fn ok(valid_answer: String, valid_wordle: Wordle) {
        let client = MockClient {
            answer:    Ok(valid_answer),
            // An invalid word list is set here to make sure it doesn't get used
            word_list: Ok(vec![String::from("invalid")]),
        };

        let res = valid_wordle.round_date(&client, MockDate);

        assert_ok!(res);
    }

    #[rstest]
    fn bad_answer(valid_strings: Vec<String>, valid_wordle: Wordle) {
        let client = MockClient {
            answer:    Ok("invalid".to_string()),
            word_list: Ok(valid_strings),
        };

        let res = valid_wordle.round_date(&client, MockDate {});

        assert_matches!(
            res,
            Err(crate::Error::Fatal {
                e: InternalError {
                    e: Error::InvalidWord { .. },
                },
            }),
        );
    }

    #[rstest]
    fn network_error(valid_strings: Vec<String>, valid_wordle: Wordle) {
        let client = MockClient {
            answer:    Err(MockErr),
            word_list: Ok(valid_strings),
        };

        let res = valid_wordle.round_date(&client, MockDate {});

        assert_matches!(
            res,
            Err(crate::Error::Safe {
                e: InternalError {
                    e: Error::Network { .. },
                },
            }),
        );
    }
}

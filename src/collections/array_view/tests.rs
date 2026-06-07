use assertables::{
    assert_len_eq,
    assert_len_eq_x,
    assert_none,
    assert_some,
};
use fixtures::*;
use rstest::{
    fixture,
    rstest,
};

use super::*;

type AV = ArrayView<u8, 3>;

mod fixtures {
    use super::*;

    #[fixture]
    pub fn full() -> AV {
        ArrayView {
            data:  [Some(3), Some(2), Some(1)],
            count: 3,
        }
    }

    #[fixture]
    pub fn partial() -> AV {
        ArrayView {
            data:  [Some(3), Some(2), None],
            count: 2,
        }
    }

    #[fixture]
    pub fn empty() -> AV {
        ArrayView {
            data:  [None; 3],
            count: 0,
        }
    }
}

#[rstest]
#[case(full(), 3)]
#[case(partial(), 2)]
#[case(empty(), 0)]
#[trace]
fn len(#[case] av: AV, #[case] expected: usize) {
    assert_len_eq_x!(av, expected);
}

mod peek {
    use super::*;

    #[rstest]
    #[case(full(), 1)]
    #[case(partial(), 2)]
    #[trace]
    fn some(#[case] av: AV, #[case] expected: u8) {
        let val = assert_some!(av.peek());

        assert_eq!(*val, expected);
    }

    #[rstest]
    fn none(empty: AV) {
        assert_none!(empty.peek());
    }
}

mod push {
    use super::*;

    #[rstest]
    #[case(partial(), 3)]
    #[case(empty(), 1)]
    #[trace]
    fn ok(
        #[case]
        #[notrace]
        mut av: AV,
        #[case] expected_len: usize,
    ) {
        av.push(255);

        assert_len_eq_x!(&av, expected_len);

        assert_eq!(av.data[expected_len - 1], Some(255));
    }

    #[rstest]
    #[should_panic(expected = "Pushed past end of ArrayView")]
    fn panic(mut full: AV) {
        full.push(255);
    }
}

mod from_iterator {
    use super::*;

    #[rstest]
    #[case(vec![100, 200, 255])]
    #[case(vec![100, 200])]
    #[case(vec![])]
    #[trace]
    fn ok(#[case] from: Vec<u8>) {
        let av = from.iter().copied().collect::<AV>();

        assert_len_eq!(&av, &from);

        for (actual, expected) in av.data.into_iter().zip(from.iter().copied()) {
            assert_eq!(
                actual,
                Some(expected),
                "Didn't get expected value from iterator"
            );
        }
    }

    #[rstest]
    #[should_panic(expected = "Pushed past end of ArrayView")]
    fn panic() {
        let too_big = [0u8; 4];
        let _ = AV::from_iter(too_big);
    }
}

#[rstest]
#[case(full(), vec![3, 2, 1])]
#[case(partial(), vec![3, 2])]
#[case(empty(), vec![])]
#[trace]
fn into_iter(#[case] av: AV, #[case] expected: Vec<u8>) {
    let it = av.into_iter();
    let expected_count = expected.len();
    let mut count = 0usize;

    for (actual, expected) in it.zip(expected.iter().copied()) {
        count += 1;
        assert_eq!(actual, expected, "Didn't get expected value from iterator");
    }

    assert_eq!(count, expected_count, "Iterator was unexpected length");
}

use testing::{sploosh, splish};

#[test]
fn it_should_test_sploosh_and_splish() {
    assert!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)) == 4);
}
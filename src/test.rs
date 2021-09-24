use crate::calc::{dvt};

#[test]
fn my_test() {
    assert_eq!(dvt(5.0, 100.0), 0.05);
}
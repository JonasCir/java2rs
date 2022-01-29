mod common;
use crate::common::assert_code_equality;

#[test]
fn test_assert_code_equality() {
    assert_code_equality(&"resources/common/empty");
}

#[test]
#[should_panic]
fn test_assert_code_equality_should_panic() {
    assert_code_equality(&"resources/common/panic");
}

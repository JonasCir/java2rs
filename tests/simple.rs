mod common;

use crate::common::assert_code_equality;

#[test]
fn test_simple_main_class() {
    assert_code_equality(&"resources/simple/main_class");
}

#[test]
fn test_simple_noargs_constructor() {
    assert_code_equality(&"resources/simple/noargs_constructor");
}

#[test]
fn test_simple_noargs_constructor_with_main_first() {
    assert_code_equality(&"resources/simple/noargs_constructor_with_main_first");
}

#[test]
fn test_simple_noargs_constructor_with_main_last() {
    assert_code_equality(&"resources/simple/noargs_constructor_with_main_last");
}

use pretty_assertions::assert_eq;

pub fn assert_code_equality(expected: &str, actual: &str) {
    let expected = prettyplease::unparse(&syn::parse_file(expected).unwrap());
    assert_eq!(expected, actual);
}

use java2rs::process;
use pretty_assertions::assert_eq;
use std::fs;
use std::path::Path;

pub fn assert_code_equality(base_path: &str) {
    let rust_file = "tests/".to_owned() + base_path + ".rs";
    let java_file = "tests/".to_owned() + base_path + ".java";

    let rust_code = fs::read_to_string(Path::new(&rust_file)).unwrap();
    let java_code = fs::read_to_string(Path::new(&java_file)).unwrap();

    let actual = process(&java_code);
    let expected = prettyplease::unparse(&syn::parse_file(&rust_code).unwrap());

    assert_eq!(actual, expected);
}

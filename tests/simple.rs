mod common;

use crate::common::assert_code_equality;
use java2rs::process;

#[test]
fn test_simple_main_class() {
    let input = r#"
    public class MainClass {
        public static void main(String[] args){
            System.out.println("Hello, world!");
        }
    }
    "#;
    let expected = r#"
        struct MainClass {}
        fn main() {
            println!("Hello, world!")
        }
    "#;

    assert_code_equality(expected, &process(input));
}

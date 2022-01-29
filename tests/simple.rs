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

#[test]
fn test_simple_noargs_constructor() {
    let input = r#"
    public class MainClass {
        public MainClass(){
        }
    }
    "#;
    let expected = r#"
        struct MainClass {}
        impl MainClass {
            pub fn new() -> Self{
                Self{}
            }
        }
    "#;

    assert_code_equality(expected, &process(input));
}

#[test]
fn test_simple_noargs_constructor_with_main_first() {
    let input = r#"
    public class MainClass {
        public static void main(String[] args){
            System.out.println("Hello, world!");
        }
        public MainClass(){
        }
    }
    "#;
    let expected = r#"
        struct MainClass {}
        impl MainClass {
            pub fn new() -> Self{
                Self{}
            }
        }
        fn main() {
            println!("Hello, world!")
        }
    "#;

    assert_code_equality(expected, &process(input));
}

#[test]
fn test_simple_noargs_constructor_with_main_last() {
    let input = r#"
    public class MainClass {
        public MainClass(){
        }
        public static void main(String[] args){
            System.out.println("Hello, world!");
        }
    }
    "#;
    let expected = r#"
        struct MainClass {}
        impl MainClass {
            pub fn new() -> Self{
                Self{}
            }
        }
        fn main() {
            println!("Hello, world!")
        }
    "#;

    assert_code_equality(expected, &process(input));
}

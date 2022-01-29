#![feature(assert_matches)]
#![feature(iter_intersperse)]

#[macro_use]
extern crate contracts;
#[macro_use]
extern crate strum;

mod codegen;
mod ir;
mod java;

use crate::codegen::RustCodegen;
use tree_sitter::{Parser, Tree, TreeCursor};

pub fn process(java_code: &str) -> String {
    let tree = parse_java(java_code);
    let cursor = &mut tree.walk();

    generate_rust_code(java_code, cursor)
}

fn generate_rust_code(java_code: &str, cursor: &mut TreeCursor) -> String {
    let res = java::handle_program(cursor, java_code);
    let rust_code = res.to_rust();
    println!("{rust_code}");
    let syntax_tree = syn::parse_file(&rust_code.to_string()).unwrap();
    prettyplease::unparse(&syntax_tree)
}

fn parse_java(java_code: &str) -> Tree {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_java::language())
        .expect("Error loading Java grammar");
    parser.parse(java_code, None).unwrap()
}

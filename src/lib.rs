#![feature(assert_matches)]
#![feature(iter_intersperse)]
#![feature(assert_matches)]

mod java;

use crate::java::programs::handle_program;
use std::assert_matches::assert_matches;
use std::fs;
use tree_sitter::{Parser, TreeCursor};

pub fn process(path: String) {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_java::language())
        .expect("Error loading Java grammar");
    let tree = parser.parse(crate::java::CODE.clone(), None).unwrap();
    let mut cursor = tree.walk();
    let res = handle_program(&mut cursor);

    fs::write("out.rs", &res).expect("Unable to write file");
}

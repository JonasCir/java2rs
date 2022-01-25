pub mod blocks;
pub mod classes;
pub mod expressions;
pub mod fields;
pub mod identifiers;
pub mod literals;
pub mod methods;
pub mod modifiers;
pub mod programs;
pub mod types;

use lazy_static::lazy_static;
use std::fs;

lazy_static! {
    pub static ref CODE: String =
        fs::read_to_string("resources/Main.java").expect("Something went wrong reading the file");
}

pub mod utils {
    use tree_sitter::{Node, TreeCursor};

    pub fn text_print(node: &Node) {
        println!(
            "{}: {}",
            node.utf8_text(&super::CODE.as_bytes()).unwrap(),
            node.kind()
        );
    }

    pub fn text(node: &Node) -> String {
        node.utf8_text(&super::CODE.as_bytes()).unwrap().to_string()
    }

    pub fn print_children(cursor: &mut TreeCursor) {
        for child in cursor.node().children(cursor) {
            text_print(&child);
        }
        cursor.goto_parent();
    }

    pub fn print_siblings(node: Node) {
        let mut current = node;
        while let Some(new) = current.next_sibling() {
            text_print(&new);
            current = new.clone();
        }
    }
}

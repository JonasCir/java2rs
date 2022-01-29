mod blocks;
mod classes;
mod expressions;
mod fields;
mod identifiers;
mod literals;
mod methods;
mod modifiers;
mod programs;
mod types;

pub use programs::handle_program;

pub mod utils {
    use tree_sitter::{Node, TreeCursor};

    /// Used for debugging to print the text and kind of a node to stdout.
    ///
    /// # Arguments
    ///
    /// * `node`: The node to be printed.
    /// * `code`: The source file content.
    ///
    /// returns: ()
    #[allow(dead_code)]
    pub fn text_print(node: &Node, code: &str) {
        println!(
            "{}: {}",
            node.utf8_text(code.as_bytes()).unwrap(),
            node.kind()
        );
    }

    pub fn text(node: &Node, code: &str) -> String {
        node.utf8_text(code.as_bytes()).unwrap().to_string()
    }

    /// Used for debugging to print all children of the cursor's current node.
    ///
    /// # Arguments
    ///
    /// * `cursor`:
    /// * `code`:
    ///
    /// returns: ()
    #[allow(dead_code)]
    pub fn print_children(cursor: &mut TreeCursor, code: &str) {
        for child in cursor.node().children(cursor) {
            text_print(&child, code);
        }
        cursor.goto_parent();
        panic!("Panic to prevent the iterator from being used again!")
    }

    /// Used for debugging to print all siblings of the cursor's current node.
    ///
    /// # Arguments
    ///
    /// * `node`:
    /// * `code`:
    ///
    /// returns: ()
    #[allow(dead_code)]
    pub fn print_siblings(node: Node, code: &str) {
        let mut current = node;
        while let Some(new) = current.next_sibling() {
            text_print(&new, code);
            current = new;
        }
        panic!("Panic to prevent the iterator from being used again!")
    }
}

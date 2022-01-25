use crate::java::identifiers::handle_type_identifier;
use crate::java::utils::print_children;
use tree_sitter::{Node, TreeCursor};

pub fn handle_void_type(void_type: &Node) -> String {
    assert_eq!(void_type.kind(), "void_type");
    assert_eq!(void_type.next_sibling().unwrap().kind(), "identifier");
    assert_eq!(void_type.child_count(), 0);
    "".to_string()
}

pub fn handle_dimensions(cursor: &mut TreeCursor) -> String {
    let dimensions = cursor.node();
    assert_eq!(dimensions.kind(), "dimensions");
    assert!(dimensions.next_sibling().is_none());
    assert_eq!(dimensions.named_child_count(), 0);
    assert!(cursor.goto_parent());
    "[]".to_string()
}

#[must_use]
pub fn handle_array_type(cursor: &mut TreeCursor) -> String {
    let array_type = cursor.node();
    assert_eq!(array_type.kind(), "array_type");
    assert_eq!(
        array_type.next_named_sibling().unwrap().kind(),
        "identifier"
    );
    assert_eq!(array_type.child_count(), 2);
    assert!(cursor.goto_first_child());
    let type_identifier = handle_type_identifier(cursor);

    assert!(cursor.goto_next_sibling());
    let dimensions = handle_dimensions(cursor);

    assert_eq!(cursor.node().kind(), "array_type");
    type_identifier + " " + &*dimensions
}

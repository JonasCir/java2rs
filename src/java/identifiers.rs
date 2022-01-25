use crate::java::utils::{text, text_print};
use tree_sitter::{Node, TreeCursor};

#[must_use]
pub fn handle_identifier(identifier: &Node) -> String {
    assert_eq!(identifier.kind(), "identifier");
    assert!(
        identifier.next_sibling().is_none()
            || matches!(
                identifier.next_sibling().unwrap().kind(),
                "class_body" | "formal_parameters" | "." | "argument_list"
            )
    );
    assert_eq!(identifier.child_count(), 0);
    text(&identifier)
}

#[must_use]
pub fn handle_type_identifier(cursor: &mut TreeCursor) -> String {
    let type_identifier = cursor.node();
    assert_eq!(type_identifier.kind(), "type_identifier");
    assert_eq!(type_identifier.next_sibling().unwrap().kind(), "dimensions");
    assert_eq!(type_identifier.child_count(), 0);
    text(&type_identifier)
}

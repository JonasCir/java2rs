use crate::java::identifiers::handle_identifier;
use tree_sitter::TreeCursor;

#[must_use]
#[invariant(cursor.node().kind() == "field_access")]
pub fn handle_field_access(cursor: &mut TreeCursor, code: &str) -> String {
    let field_access = cursor.node();
    assert_eq!(field_access.next_sibling().unwrap().kind(), ".");
    assert_eq!(field_access.child_count(), 3);

    // todo the iteration in this method somehow breaks the cursor, so we need to create a new one
    //  before calling this function for now
    let access_str = field_access
        .named_children(cursor)
        .collect::<Vec<_>>()
        .iter()
        .map(|c| -> String { handle_identifier(c, code) })
        .intersperse('.'.to_string())
        .collect();

    assert!(cursor.goto_parent());
    access_str
}

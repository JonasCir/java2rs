use crate::java::identifiers::handle_identifier;
use crate::java::utils::print_children;
use tree_sitter::TreeCursor;

pub fn handle_field_access(cursor: &mut TreeCursor) -> String {
    let field_access = cursor.node();
    assert_eq!(field_access.kind(), "field_access");
    assert_eq!(field_access.next_sibling().unwrap().kind(), ".");
    assert_eq!(field_access.child_count(), 3);

    field_access
        .named_children(cursor)
        .collect::<Vec<_>>()
        .iter()
        .map(handle_identifier)
        .intersperse('.'.to_string())
        .collect()
}

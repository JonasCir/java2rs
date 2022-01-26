use crate::ir::r#type::{ArrayType, Dimensions, ScalarType};
use crate::java::identifiers::handle_type_identifier;
use crate::java::utils::text;
use tree_sitter::{Node, TreeCursor};

pub fn handle_void_type(void_type: &Node) -> ScalarType {
    assert_eq!(void_type.kind(), "void_type");
    assert_eq!(void_type.next_sibling().unwrap().kind(), "identifier");
    assert_eq!(void_type.child_count(), 0);
    ScalarType::Void
}

pub fn handle_dimensions(dimensions: &Node, code: &str) -> Dimensions {
    assert_eq!(dimensions.kind(), "dimensions");
    assert!(dimensions.next_sibling().is_none());
    assert_eq!(dimensions.named_child_count(), 0);
    let dim = text(dimensions, code);
    assert_eq!(dim, "[]");
    let res = Dimensions::default();
    assert!(res.start == 0 && res.end == 0);
    res
}

#[must_use]
pub fn handle_array_type(cursor: &mut TreeCursor, code: &str) -> ArrayType {
    let array_type = cursor.node();
    assert_eq!(array_type.kind(), "array_type");
    assert_eq!(
        array_type.next_named_sibling().unwrap().kind(),
        "identifier"
    );
    assert_eq!(array_type.child_count(), 2);

    assert!(cursor.goto_first_child());
    let type_identifier = handle_type_identifier(&cursor.node(), code);

    assert!(cursor.goto_next_sibling());
    let dimensions = handle_dimensions(&cursor.node(), code);

    assert!(cursor.goto_parent());
    assert_eq!(cursor.node().kind(), "array_type");

    ArrayType::new(type_identifier, dimensions)
}

use crate::ir::r#type::{ArrayType, Dimensions, ScalarType};
use crate::java::identifiers::handle_type_identifier;
use crate::java::utils::text;
use tree_sitter::{Node, TreeCursor};

#[must_use]
#[requires(void_type.kind() == "void_type")]
#[ensures(ret == ScalarType::Void)]
pub fn handle_void_type(void_type: &Node) -> ScalarType {
    assert_eq!(void_type.next_sibling().unwrap().kind(), "identifier");
    ScalarType::Void
}

#[must_use]
#[requires(dimensions.kind() == "dimensions")]
#[ensures(ret.start() == 0 && ret.end() == 0)]
pub fn handle_dimensions(dimensions: &Node, code: &str) -> Dimensions {
    assert!(dimensions.next_sibling().is_none());
    assert_eq!(dimensions.child_count(), 2);
    assert_eq!(dimensions.named_child_count(), 0);
    text(dimensions, code).parse().unwrap()
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

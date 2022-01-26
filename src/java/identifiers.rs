use crate::ir::r#type::ScalarType;
use crate::java::utils::text;
use tree_sitter::Node;

#[must_use]
pub fn handle_identifier(identifier: &Node, code: &str) -> String {
    assert_eq!(identifier.kind(), "identifier");
    assert!(
        identifier.next_sibling().is_none()
            || matches!(
                identifier.next_sibling().unwrap().kind(),
                "class_body" | "formal_parameters" | "." | "argument_list"
            )
    );
    assert_eq!(identifier.child_count(), 0);
    text(identifier, code)
}

#[must_use]
pub fn handle_type_identifier(type_identifier: &Node, code: &str) -> ScalarType {
    assert_eq!(type_identifier.kind(), "type_identifier");
    assert_eq!(type_identifier.next_sibling().unwrap().kind(), "dimensions");
    assert_eq!(type_identifier.child_count(), 0);
    let res = text(type_identifier, code);
    assert_eq!(res, "String");
    ScalarType::String
}

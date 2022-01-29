use crate::ir;
use crate::java::utils::text;
use tree_sitter::Node;

#[must_use]
#[requires(identifier.kind() == "identifier" && identifier.child_count() == 0)]
pub fn handle_identifier(identifier: &Node, code: &str) -> String {
    assert!(
        identifier.next_sibling().is_none()
            || matches!(
                identifier.next_sibling().unwrap().kind(),
                "class_body" | "formal_parameters" | "." | "argument_list"
            )
    );
    text(identifier, code)
}

#[must_use]
#[requires(type_identifier.kind() == "type_identifier" && type_identifier.child_count() == 0)]
#[ensures(ret == ir::ScalarType::String)]
pub fn handle_type_identifier(type_identifier: &Node, code: &str) -> ir::ScalarType {
    assert_eq!(type_identifier.next_sibling().unwrap().kind(), "dimensions");
    text(type_identifier, code).parse().unwrap()
}

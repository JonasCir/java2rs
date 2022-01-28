use crate::java::utils::text;
use tree_sitter::Node;

#[must_use]
#[requires(string_literal.kind() == "string_literal" && string_literal.child_count() == 0)]
pub fn handle_string_literal(string_literal: &Node, code: &str) -> String {
    assert_eq!(string_literal.next_sibling().unwrap().kind(), ")");
    text(string_literal, code)
}

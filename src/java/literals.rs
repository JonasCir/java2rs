use crate::java::utils::text;
use tree_sitter::Node;

pub fn handle_string_literal(string_literal: &Node, code: &str) -> String {
    assert_eq!(string_literal.kind(), "string_literal");
    assert_eq!(string_literal.next_sibling().unwrap().kind(), ")");
    assert_eq!(string_literal.child_count(), 0);

    text(string_literal, code)
}

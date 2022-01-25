use crate::java::methods::handle_method_invocation;
use crate::java::utils::print_children;
use tree_sitter::TreeCursor;

pub fn handle_expression_statement(cursor: &mut TreeCursor) -> String {
    let exp_statement = cursor.node();
    assert_eq!(exp_statement.kind(), "expression_statement");
    assert!(
        exp_statement.next_sibling().is_none()
            || exp_statement.next_sibling().unwrap().kind() == "}"
    );

    assert_eq!(exp_statement.child_count(), 2);

    assert!(cursor.goto_first_child());
    let method_invocation = handle_method_invocation(cursor);

    assert_eq!(exp_statement.child(1).unwrap().kind(), ";");
    method_invocation
}

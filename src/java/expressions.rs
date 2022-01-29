use crate::ir;
use crate::java::methods::handle_method_invocation;
use tree_sitter::TreeCursor;

#[must_use]
#[invariant(cursor.node().kind() == "expression_statement")]
pub fn handle_expression_statement(cursor: &mut TreeCursor, code: &str) -> ir::ExpressionStatement {
    let exp_statement = cursor.node();
    assert!(
        exp_statement.next_sibling().is_none()
            || exp_statement.next_sibling().unwrap().kind() == "}"
    );
    assert_eq!(exp_statement.child_count(), 2);

    assert!(cursor.goto_first_child());
    let method_invocation = handle_method_invocation(cursor, code);

    assert!(cursor.goto_next_sibling());
    assert_eq!(cursor.node().kind(), ";");

    assert!(cursor.goto_parent());

    ir::ExpressionStatement::MethodInvocation(method_invocation)
}

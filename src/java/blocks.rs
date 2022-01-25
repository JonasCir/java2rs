use crate::java::expressions::handle_expression_statement;
use crate::java::utils::*;
use tree_sitter::TreeCursor;

#[must_use]
pub fn handle_block(cursor: &mut TreeCursor) -> String {
    let block = cursor.node();
    assert_eq!(block.kind(), "block");
    assert!(block.next_sibling().is_none());
    assert_eq!(block.child_count(), 3);
    assert_eq!(block.named_child_count(), 1);

    assert!(cursor.goto_first_child());
    //eat {
    assert!(cursor.goto_next_sibling());
    let stmt = handle_expression_statement(cursor);
    //assert!(cursor.goto_parent());

    //assert_eq!(cursor.node().kind(), "block");
    stmt
}

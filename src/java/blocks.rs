use crate::ir::block::Block;
use crate::java::expressions::handle_expression_statement;
use tree_sitter::TreeCursor;

#[must_use]
#[invariant(cursor.node().kind() == "block")]
pub fn handle_block(cursor: &mut TreeCursor, code: &str) -> Block {
    let block = cursor.node();
    assert!(block.next_sibling().is_none());
    assert_eq!(block.child_count(), 3);
    assert_eq!(block.named_child_count(), 1);

    assert!(cursor.goto_first_child());
    assert_eq!(cursor.node().kind(), "{");

    assert!(cursor.goto_next_sibling());
    let stmt = handle_expression_statement(cursor, code);

    assert!(cursor.goto_next_sibling());
    assert_eq!(cursor.node().kind(), "}");
    assert!(cursor.goto_parent());

    vec![stmt]
}

use crate::ir::program::Program;
use crate::java::classes::handle_class_declaration;
use tree_sitter::TreeCursor;

#[must_use]
pub fn handle_program(cursor: &mut TreeCursor, code: &str) -> Program {
    let program = cursor.node();
    assert_eq!(program.kind(), "program");
    assert!(program.next_sibling().is_none());
    assert_eq!(program.child_count(), 1);
    assert!(cursor.goto_first_child());
    let class = handle_class_declaration(cursor, code);
    Program::new(class)
}

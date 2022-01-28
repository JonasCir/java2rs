use crate::ir::class::Class;
use crate::ir::method::MethodDeclaration;
use crate::java::identifiers::handle_identifier;
use crate::java::methods::handle_method_declaration;
use crate::java::modifiers::*;
use tree_sitter::{Node, TreeCursor};

#[must_use]
#[invariant(cursor.node().kind() == "class_declaration")]
pub fn handle_class_declaration(cursor: &mut TreeCursor, code: &str) -> Class {
    let class_declaration = cursor.node();
    assert!(class_declaration.next_sibling().is_none());
    assert_eq!(class_declaration.child_count(), 4);

    assert!(cursor.goto_first_child());
    let modifiers = handle_modifiers(cursor);

    assert!(cursor.goto_next_sibling());
    // just eat the class token
    handle_class(&cursor.node());

    assert!(cursor.goto_next_sibling());
    let class_name = handle_identifier(&cursor.node(), code);

    assert!(cursor.goto_next_sibling());
    let (methods, static_methods) = handle_class_body(cursor, code);

    assert!(cursor.goto_parent());
    Class::new(class_name, modifiers, methods, static_methods)
}

pub fn handle_class(class: &Node) {
    assert_eq!(class.kind(), "class");
    assert_eq!(class.next_sibling().unwrap().kind(), "identifier");
    assert_eq!(class.child_count(), 0);
}

#[must_use]
#[invariant(cursor.node().kind() == "class_body")]
pub fn handle_class_body(
    cursor: &mut TreeCursor,
    code: &str,
) -> (Vec<MethodDeclaration>, Vec<MethodDeclaration>) {
    let class_body = cursor.node();
    assert!(class_body.next_sibling().is_none());
    assert_eq!(class_body.child_count(), 3);
    assert_eq!(class_body.named_child_count(), 1);

    // skip the unnamed children "{" and "}" which wrap the class body
    assert!(cursor.goto_first_child());
    assert_eq!(cursor.node().kind(), "{");

    assert!(cursor.goto_next_sibling());
    let method_decl = handle_method_declaration(cursor, code);

    assert!(cursor.goto_next_sibling());
    assert_eq!(cursor.node().kind(), "}");

    assert!(cursor.goto_parent());

    // static methods need to be moved out of the class to dedicated impl block, so create a tuple for
    // static and member functions
    let methods = vec![];
    let static_methods = vec![method_decl];
    (methods, static_methods)
}

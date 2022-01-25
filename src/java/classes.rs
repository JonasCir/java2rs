use crate::java::identifiers::handle_identifier;
use crate::java::methods::handle_method_declaration;
use crate::java::modifiers::*;
use crate::java::utils::{print_children, text_print};
use std::collections::VecDeque;
use tree_sitter::{Node, TreeCursor};

#[must_use]
pub fn handle_class_declaration(cursor: &mut TreeCursor) -> String {
    let class_declaration = cursor.node();
    assert_eq!(class_declaration.kind(), "class_declaration");
    assert!(class_declaration.next_sibling().is_none());
    assert_eq!(class_declaration.child_count(), 4);
    let mut children: VecDeque<Node> = class_declaration.children(cursor).collect();
    assert_eq!(children.len(), 4);
    let modifiers = handle_modifiers(&mut children.pop_front().unwrap().walk());

    handle_class(&mut children.pop_front().unwrap());

    let class_name = handle_identifier(&children.pop_front().unwrap());
    let class_body = handle_class_body(&mut children.pop_front().unwrap().walk());
    let methods_in_body = class_body.1;

    assert_eq!(cursor.node().kind(), "}");

    modifiers
        + " struct "
        + &*class_name
        + "{"
        + "}"
        + "\n\n"
        + "impl "
        + &*class_name
        + "{\n"
        + &*methods_in_body
        + "\n}"
}

#[must_use]
pub fn handle_class_body(cursor: &mut TreeCursor) -> (String, String) {
    let class_body = cursor.node();
    assert_eq!(class_body.kind(), "class_body");
    assert!(class_body.next_sibling().is_none());
    assert_eq!(class_body.named_child_count(), 1);
    // skip the unnamed children "{" and "}" which wrap the class body
    let mut methods: Vec<Node> = class_body.named_children(cursor).collect();
    assert_eq!(methods.len(), 1);
    let method_declaration = methods.pop().unwrap();
    let method_decl = handle_method_declaration(&mut method_declaration.walk());

    // static methods need to be moved out of the class to dedicated impl block, so create a tuple for
    // static and member functions
    ("".to_string(), method_decl)
}

pub fn handle_class(class: &Node) {
    assert_eq!(class.kind(), "class");
    assert_eq!(class.next_sibling().unwrap().kind(), "identifier");
    assert_eq!(class.child_count(), 0);
}

use crate::java::blocks::handle_block;
use crate::java::fields::handle_field_access;
use crate::java::identifiers::*;
use crate::java::literals::handle_string_literal;
use crate::java::modifiers::handle_modifiers;
use crate::java::types::*;
use crate::java::utils::*;
use std::collections::VecDeque;
use tree_sitter::{Node, TreeCursor};

#[must_use]
pub fn handle_method_declaration(cursor: &mut TreeCursor) -> String {
    let method_declaration = cursor.node();
    assert_eq!(method_declaration.kind(), "method_declaration");
    // ignore method brackets
    assert!(method_declaration.next_named_sibling().is_none());
    assert_eq!(method_declaration.child_count(), 5);

    let mut declaration_parts: VecDeque<Node> = method_declaration.children(cursor).collect();
    assert_eq!(declaration_parts.len(), 5);
    let modifiers = handle_modifiers(&mut declaration_parts.pop_front().unwrap().walk());
    let return_type = handle_void_type(&mut declaration_parts.pop_front().unwrap());
    let method_name = handle_identifier(&mut declaration_parts.pop_front().unwrap());
    let parameters = handle_formal_parameters(&mut declaration_parts.pop_front().unwrap().walk());
    let method_body = handle_block(&mut declaration_parts.pop_front().unwrap().walk());

    "\n\t".to_string()
        + &*modifiers
        + " fn "
        + &*method_name
        + " ("
        + &*parameters
        + ")"
        + " -> "
        + &*return_type
        + "{\n"
        + &*method_body
        + "\n}"
}

pub fn handle_formal_parameters(cursor: &mut TreeCursor) -> String {
    let formal_parameters = cursor.node();
    assert_eq!(formal_parameters.kind(), "formal_parameters");
    assert!(
        formal_parameters.next_named_sibling().is_none()
            || formal_parameters.next_named_sibling().unwrap().kind() == "block"
    );
    assert_eq!(formal_parameters.named_child_count(), 1);

    let mut formal_parameters: Vec<Node> = formal_parameters.named_children(cursor).collect();
    assert_eq!(formal_parameters.len(), 1);

    let formal_parameter = &mut formal_parameters.pop().unwrap();
    let parameter = handle_formal_parameter(&mut formal_parameter.walk());
    assert!(cursor.goto_parent());
    assert_eq!(cursor.node().kind(), "formal_parameters");
    parameter
}

pub fn handle_formal_parameter(cursor: &mut TreeCursor) -> String {
    let formal_parameter = cursor.node();
    assert_eq!(formal_parameter.kind(), "formal_parameter");
    assert_eq!(formal_parameter.next_sibling().unwrap().kind(), ")");
    assert_eq!(formal_parameter.child_count(), 2);
    assert_eq!(formal_parameter.named_child_count(), 2);

    let array_type = handle_array_type(&mut formal_parameter.named_child(0).unwrap().walk());

    let parameter_name = handle_identifier(&mut formal_parameter.named_child(1).unwrap());
    assert_eq!(cursor.node().kind(), "formal_parameter");
    parameter_name + ":" + &*array_type
}

pub fn handle_method_invocation(cursor: &mut TreeCursor) -> String {
    let method_invocation = cursor.node();
    assert_eq!(method_invocation.kind(), "method_invocation");
    assert_eq!(method_invocation.next_sibling().unwrap().kind(), ";");
    assert_eq!(method_invocation.child_count(), 4);
    assert_eq!(method_invocation.named_child_count(), 3);
    let mut invocation_parts: VecDeque<Node> = method_invocation.named_children(cursor).collect();

    let field_access = handle_field_access(&mut invocation_parts.pop_front().unwrap().walk());
    let identifier = handle_identifier(&mut invocation_parts.pop_front().unwrap());
    let arguments = handle_argument_list(&mut invocation_parts.pop_front().unwrap().walk());

    field_access + "." + &*identifier + "(" + &*arguments + ")"
}

pub fn handle_argument_list(cursor: &mut TreeCursor) -> String {
    let argument_list = cursor.node();
    assert_eq!(argument_list.kind(), "argument_list");
    assert!(argument_list.next_sibling().is_none());
    assert_eq!(argument_list.child_count(), 3);
    assert_eq!(argument_list.named_child_count(), 1);

    let mut arguments: VecDeque<Node> = argument_list.named_children(cursor).collect();

    let string_literal = handle_string_literal(&arguments.pop_front().unwrap());
    string_literal
}

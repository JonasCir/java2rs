use crate::ir;
use crate::ir::Parameters;
use crate::java::blocks::handle_block;
use crate::java::fields::handle_field_access;
use crate::java::identifiers::*;
use crate::java::literals::handle_string_literal;
use crate::java::modifiers::handle_modifiers;
use crate::java::types::*;
use tree_sitter::TreeCursor;

#[must_use]
#[invariant(cursor.node().kind() == "method_declaration")]
pub fn handle_method_declaration(cursor: &mut TreeCursor, code: &str) -> ir::MethodDeclaration {
    let method_declaration = cursor.node();
    assert_eq!(method_declaration.child_count(), 5);

    assert!(cursor.goto_first_child());
    let modifier = handle_modifiers(cursor);

    assert!(cursor.goto_next_sibling());
    let return_type = handle_void_type(&cursor.node());

    assert!(cursor.goto_next_sibling());
    let method_name = handle_identifier(&cursor.node(), code);

    assert!(cursor.goto_next_sibling());
    let parameters = handle_formal_parameters(cursor, code);

    assert!(cursor.goto_next_sibling());
    let method_body = handle_block(cursor, code);

    assert!(cursor.goto_parent());

    assert!(
        modifier.static_access(),
        "We allow only static method declarations for now"
    );

    ir::MethodDeclaration::new(
        method_name,
        modifier,
        parameters,
        ir::Type::Scalar(return_type),
        method_body,
    )
}

#[must_use]
#[invariant(cursor.node().kind() == "formal_parameters")]
pub fn handle_formal_parameters(cursor: &mut TreeCursor, code: &str) -> ir::Parameters {
    assert!(cursor.goto_first_child());
    assert_eq!(cursor.node().kind(), "(");

    assert!(cursor.goto_next_sibling());

    let mut parameters = Parameters::new();
    while cursor.node().kind() != ")" {
        let parameter = handle_formal_parameter(cursor, code);
        parameters.push(parameter);
        assert!(cursor.goto_next_sibling());
    }

    assert!(cursor.goto_parent());
    parameters
}

#[must_use]
#[invariant(cursor.node().kind() == "formal_parameter")]
pub fn handle_formal_parameter(cursor: &mut TreeCursor, code: &str) -> ir::Parameter {
    let formal_parameter = cursor.node();
    assert_eq!(formal_parameter.next_sibling().unwrap().kind(), ")");
    assert_eq!(formal_parameter.child_count(), 2);
    assert_eq!(formal_parameter.named_child_count(), 2);

    assert!(cursor.goto_first_child());
    let array_type = handle_array_type(cursor, code);
    let ty = ir::Type::Array(array_type);

    assert!(cursor.goto_next_sibling());
    let parameter_name = handle_identifier(&formal_parameter.named_child(1).unwrap(), code);

    assert!(cursor.goto_parent());
    ir::Parameter::new(parameter_name, ty)
}

#[must_use]
#[invariant(cursor.node().kind() == "method_invocation")]
pub fn handle_method_invocation(cursor: &mut TreeCursor, code: &str) -> ir::MethodInvocation {
    let method_invocation = cursor.node();
    assert_eq!(method_invocation.next_sibling().unwrap().kind(), ";");
    assert_eq!(method_invocation.child_count(), 4);
    assert_eq!(method_invocation.named_child_count(), 3);

    assert!(cursor.goto_first_child());
    // todo the iteration in handle_field_access somehow breaks the cursor, so we create a new one
    let field_access = handle_field_access(&mut cursor.node().walk(), code);
    assert_eq!(cursor.node().kind(), "field_access");

    assert!(cursor.goto_next_sibling());
    assert_eq!(cursor.node().kind(), ".");

    assert!(cursor.goto_next_sibling());
    let identifier = handle_identifier(&cursor.node(), code);
    assert_eq!(cursor.node().kind(), "identifier");

    assert!(cursor.goto_next_sibling());
    let arguments = handle_argument_list(cursor, code);

    assert!(cursor.goto_parent());
    ir::MethodInvocation::new(field_access, identifier, arguments)
}

#[must_use]
#[invariant(cursor.node().kind() == "argument_list")]
pub fn handle_argument_list(cursor: &mut TreeCursor, code: &str) -> ir::Arguments {
    let argument_list = cursor.node();
    assert!(argument_list.next_sibling().is_none());
    assert_eq!(argument_list.child_count(), 3);
    assert_eq!(argument_list.named_child_count(), 1);

    assert!(cursor.goto_first_child());
    assert_eq!(cursor.node().kind(), "(");

    assert!(cursor.goto_next_sibling());
    let string_literal = handle_string_literal(&cursor.node(), code);

    assert!(cursor.goto_next_sibling());
    assert_eq!(cursor.node().kind(), ")");

    assert!(cursor.goto_parent());
    vec![string_literal]
}

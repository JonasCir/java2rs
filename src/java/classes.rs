use crate::ir;

use crate::ir::Modifier;
use crate::java::expressions::handle_expression_statement;
use crate::java::identifiers::handle_identifier;
use crate::java::methods::{handle_formal_parameters, handle_method_declaration};
use crate::java::modifiers::*;
use tree_sitter::{Node, TreeCursor};

#[must_use]
#[invariant(cursor.node().kind() == "class_declaration")]
pub fn handle_class_declaration(cursor: &mut TreeCursor, code: &str) -> ir::ClassDeclaration {
    let class_declaration = cursor.node();
    assert!(class_declaration.next_sibling().is_none());

    assert!(cursor.goto_first_child());

    let modifiers = if cursor.node().kind() == "modifiers" {
        let modifiers = handle_modifiers(cursor);
        assert!(cursor.goto_next_sibling());
        modifiers
    } else {
        Modifier::empty()
    };

    // just eat the class token
    handle_class(&cursor.node());

    assert!(cursor.goto_next_sibling());
    let name = handle_identifier(&cursor.node(), code);

    assert!(cursor.goto_next_sibling());
    let class_body = handle_class_body(cursor, code);

    assert!(cursor.goto_parent());
    ir::ClassDeclaration::new(name, modifiers, class_body)
}

pub fn handle_class(class: &Node) {
    assert_eq!(class.kind(), "class");
    assert_eq!(class.next_sibling().unwrap().kind(), "identifier");
    assert_eq!(class.child_count(), 0);
}

#[must_use]
#[invariant(cursor.node().kind() == "class_body")]
pub fn handle_class_body(cursor: &mut TreeCursor, code: &str) -> ir::ClassBody {
    // skip the unnamed children "{" and "}" which wrap the class body
    assert!(cursor.goto_first_child());
    assert_eq!(cursor.node().kind(), "{");

    assert!(cursor.goto_next_sibling());

    let mut method_declarations = ir::MethodDeclarations::new();
    let mut constructor_declaration: Option<ir::ConstructorDeclaration> = None;

    // grammar calls this class_body_declaration
    while cursor.node().kind() != "}" {
        match cursor.node().kind() {
            "method_declaration" => {
                method_declarations.add_method(handle_method_declaration(cursor, code));
            }
            "constructor_declaration" => {
                constructor_declaration = Some(handle_constructor_declaration(cursor, code));
            }
            _ => {
                unimplemented!()
            }
        }
        assert!(cursor.goto_next_sibling());
    }

    assert!(cursor.goto_parent());
    ir::ClassBody::new(constructor_declaration, method_declarations)
}

#[must_use]
#[invariant(cursor.node().kind() == "constructor_declaration")]
pub fn handle_constructor_declaration(
    cursor: &mut TreeCursor,
    code: &str,
) -> ir::ConstructorDeclaration {
    let constructor_declaration = cursor.node();
    assert_eq!(constructor_declaration.child_count(), 4);

    assert!(cursor.goto_first_child());
    let modifier = handle_modifiers(cursor);

    assert!(cursor.goto_next_sibling());
    let _ = handle_identifier(&cursor.node(), code);

    assert!(cursor.goto_next_sibling());
    let parameters = handle_formal_parameters(cursor, code);

    assert!(cursor.goto_next_sibling());
    let constructor_body = handle_constructor_body(cursor, code);

    assert!(cursor.goto_parent());

    ir::ConstructorDeclaration::new(modifier, parameters, constructor_body)
}

#[must_use]
#[invariant(cursor.node().kind() == "constructor_body")]
pub fn handle_constructor_body(cursor: &mut TreeCursor, code: &str) -> ir::ConstructorBody {
    // skip the unnamed children "{" and "}" which wrap the class body
    assert!(cursor.goto_first_child());
    assert_eq!(cursor.node().kind(), "{");

    assert!(cursor.goto_next_sibling());

    let mut statements = ir::Statements::new();
    while cursor.node().kind() != "}" {
        let expression_statement = handle_expression_statement(cursor, code);
        statements.push(expression_statement);
        assert!(cursor.goto_next_sibling());
    }

    assert!(cursor.goto_parent());
    ir::ConstructorBody::new(statements)
}

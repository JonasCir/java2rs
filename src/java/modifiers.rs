use crate::ir::modifier::{Modifier, StaticAccess, Visibility};
use std::assert_matches::assert_matches;
use tree_sitter::{Node, TreeCursor};

#[must_use]
#[invariant(cursor.node().kind() == "modifiers")]
pub fn handle_modifiers(cursor: &mut TreeCursor) -> Modifier {
    let modifiers = cursor.node();
    assert_matches!(
        modifiers.next_sibling().unwrap().kind(),
        "class" | "void_type"
    );
    assert!(0 < modifiers.child_count() && modifiers.child_count() <= 2);

    assert!(cursor.goto_first_child());
    let public = handle_public(&cursor.node());

    // check for static access
    let mut static_access = StaticAccess::NotStatic;
    if cursor.goto_next_sibling() {
        static_access = handle_static(&cursor.node());
    }

    assert!(cursor.goto_parent());
    Modifier::new(public, static_access)
}

#[must_use]
#[requires(r#static.kind() == "static")]
#[ensures(ret == StaticAccess::Static)]
pub fn handle_static(r#static: &Node) -> StaticAccess {
    assert!(r#static.next_sibling().is_none());
    assert_eq!(r#static.child_count(), 0);
    StaticAccess::Static
}

#[must_use]
#[requires(public.kind() == "public")]
#[ensures(ret == Visibility::Public)]
pub fn handle_public(public: &Node) -> Visibility {
    assert!(public.next_sibling().is_none() || public.next_sibling().unwrap().kind() == "static");
    assert_eq!(public.child_count(), 0);
    Visibility::Public
}

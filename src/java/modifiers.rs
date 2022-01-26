use crate::ir::modifier::Visibility::Public;
use crate::ir::modifier::{Modifier, StaticAccess, Visibility};
use std::assert_matches::assert_matches;
use tree_sitter::TreeCursor;

#[must_use]
pub fn handle_modifiers(cursor: &mut TreeCursor) -> Modifier {
    let modifiers = cursor.node();
    assert_eq!(modifiers.kind(), "modifiers");
    assert_matches!(
        modifiers.next_sibling().unwrap().kind(),
        "class" | "void_type"
    );
    assert!(0 < modifiers.child_count() && modifiers.child_count() <= 2);

    assert!(cursor.goto_first_child());
    let public = handle_public(cursor);
    let mut static_access = StaticAccess::NotStatic;
    if cursor.goto_next_sibling() {
        static_access = handle_static(cursor);
        assert_eq!(static_access, StaticAccess::Static);
    }
    assert!(cursor.goto_parent());
    assert_eq!(cursor.node().kind(), "modifiers");
    Modifier::new(public, static_access)
}

pub fn handle_static(cursor: &mut TreeCursor) -> StaticAccess {
    let r#static = cursor.node();
    assert_eq!(r#static.kind(), "static");
    assert!(r#static.next_sibling().is_none());
    assert_eq!(r#static.child_count(), 0);
    StaticAccess::Static
}

#[must_use]
pub fn handle_public(cursor: &mut TreeCursor) -> Visibility {
    let public = cursor.node();
    assert_eq!(public.kind(), "public");
    assert!(public.next_sibling().is_none() || public.next_sibling().unwrap().kind() == "static");
    assert_eq!(public.child_count(), 0);
    Public
}

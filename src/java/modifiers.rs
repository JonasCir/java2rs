use crate::ir;
use tree_sitter::{Node, TreeCursor};

#[must_use]
#[invariant(cursor.node().kind() == "modifiers")]
pub fn handle_modifiers(cursor: &mut TreeCursor) -> ir::Modifier {
    let modifiers = cursor.node();
    assert!(0 < modifiers.child_count() && modifiers.child_count() <= 2);

    assert!(cursor.goto_first_child());
    let public = handle_public(&cursor.node());

    // check for static access
    let mut static_access = ir::StaticAccess::NotStatic;
    if cursor.goto_next_sibling() {
        static_access = handle_static(&cursor.node());
    }

    assert!(cursor.goto_parent());
    ir::Modifier::new(public, static_access)
}

#[must_use]
#[requires(r#static.kind() == "static")]
#[ensures(ret == ir::StaticAccess::Static)]
pub fn handle_static(r#static: &Node) -> ir::StaticAccess {
    assert!(r#static.next_sibling().is_none());
    assert_eq!(r#static.child_count(), 0);
    ir::StaticAccess::Static
}

#[must_use]
#[requires(public.kind() == "public")]
#[ensures(ret == ir::Visibility::Public)]
pub fn handle_public(public: &Node) -> ir::Visibility {
    assert!(public.next_sibling().is_none() || public.next_sibling().unwrap().kind() == "static");
    assert_eq!(public.child_count(), 0);
    ir::Visibility::Public
}

use std::assert_matches::assert_matches;
use tree_sitter::TreeCursor;

#[must_use]
pub fn handle_modifiers(cursor: &mut TreeCursor) -> String {
    let modifiers = cursor.node();
    assert_eq!(modifiers.kind(), "modifiers");
    assert_matches!(
        modifiers.next_sibling().unwrap().kind(),
        "class" | "void_type"
    );
    assert!(0 < modifiers.child_count() && modifiers.child_count() <= 2);
    assert!(cursor.goto_first_child());

    let ret = handle_public(cursor);

    if cursor.goto_next_sibling() {
        ret.to_string().push_str(&handle_static(cursor));
    }
    assert!(cursor.goto_parent());
    assert_eq!(cursor.node().kind(), "modifiers");
    ret
}

pub fn handle_static(cursor: &mut TreeCursor) -> String {
    let r#static = cursor.node();
    assert_eq!(r#static.kind(), "static");
    assert!(r#static.next_sibling().is_none());
    assert_eq!(r#static.child_count(), 0);
    "static".to_string()
}

#[must_use]
pub fn handle_public(cursor: &mut TreeCursor) -> String {
    let public = cursor.node();
    assert_eq!(public.kind(), "public");
    assert!(public.next_sibling().is_none() || public.next_sibling().unwrap().kind() == "static");
    assert_eq!(public.child_count(), 0);
    "pub".to_string()
}

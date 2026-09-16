use super::{UNTITLED_GROUP_NAME, group_suffix_label, pane_index_label};

#[test]
fn grouped_tab_uses_group_name() {
    assert_eq!(group_suffix_label(Some("GitHub")), "GitHub");
}

#[test]
fn unnamed_group_uses_new_group() {
    assert_eq!(group_suffix_label(None), UNTITLED_GROUP_NAME);
    assert_eq!(group_suffix_label(Some("")), UNTITLED_GROUP_NAME);
    assert_eq!(group_suffix_label(Some("   ")), UNTITLED_GROUP_NAME);
}

#[test]
fn pane_suffix_omitted_for_single_pane() {
    assert_eq!(pane_index_label(None), None);
}

#[test]
fn pane_suffix_is_one_based_index() {
    assert_eq!(pane_index_label(Some(2)), Some("2".to_string()));
}

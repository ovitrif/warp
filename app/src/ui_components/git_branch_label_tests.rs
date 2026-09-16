use super::compact_branch_subtitle_display;

#[test]
fn branch_wins_over_working_directory() {
    assert_eq!(
        compact_branch_subtitle_display(Some("main"), Some("~/r/gh/warp")),
        Some(("main".to_string(), true))
    );
}

#[test]
fn empty_branch_falls_back_to_path() {
    assert_eq!(
        compact_branch_subtitle_display(Some("  "), Some("~/vault")),
        Some(("~/vault".to_string(), false))
    );
}

#[test]
fn path_when_no_branch() {
    assert_eq!(
        compact_branch_subtitle_display(None, Some("~/r/sd")),
        Some(("~/r/sd".to_string(), false))
    );
}

#[test]
fn nothing_when_both_empty() {
    assert_eq!(compact_branch_subtitle_display(None, None), None);
    assert_eq!(compact_branch_subtitle_display(Some(""), Some("")), None);
}

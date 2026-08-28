use super::palette_chip_value_text;
use crate::context_chips::{ChipValue, ContextChipKind};

#[test]
fn github_pr_chip_in_sessions_palette_shows_hash_id() {
    let value = ChipValue::Text("https://github.com/synonymdev/bitkit-ios/pull/691".to_string());

    assert_eq!(
        palette_chip_value_text(&ContextChipKind::GithubPullRequest, &value),
        "#691"
    );
}

#[test]
fn github_pr_chip_in_sessions_palette_falls_back_to_raw_url() {
    let value = ChipValue::Text("https://github.com/warp/warp/issues/12".to_string());

    assert_eq!(
        palette_chip_value_text(&ContextChipKind::GithubPullRequest, &value),
        "https://github.com/warp/warp/issues/12"
    );
}

#[test]
fn other_palette_chips_keep_raw_value() {
    let value = ChipValue::Text("feat/raw-post".to_string());

    assert_eq!(
        palette_chip_value_text(&ContextChipKind::ShellGitBranch, &value),
        "feat/raw-post"
    );
}

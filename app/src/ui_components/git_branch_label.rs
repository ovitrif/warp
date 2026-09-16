use warp_core::ui::theme::Fill as WarpThemeFill;
use warpui::Element;
use warpui::elements::{ConstrainedBox, CrossAxisAlignment, Flex, ParentElement, Shrinkable, Text};
use warpui::text_layout::ClipConfig;

use crate::appearance::Appearance;
use crate::ui_components::icons::Icon as UiIcon;

/// Branch name when present, otherwise the working directory. The bool is true when the
/// string is a git branch and should be rendered with a branch icon.
pub(crate) fn compact_branch_subtitle_display(
    git_branch: Option<&str>,
    working_directory: Option<&str>,
) -> Option<(String, bool)> {
    git_branch
        .filter(|branch| !branch.trim().is_empty())
        .map(|branch| (branch.to_string(), true))
        .or_else(|| {
            working_directory
                .filter(|wd| !wd.trim().is_empty())
                .map(|wd| (wd.to_string(), false))
        })
}

pub(crate) fn render_git_branch_text(
    branch: &str,
    text_color: WarpThemeFill,
    font_size: f32,
    appearance: &Appearance,
) -> Box<dyn Element> {
    Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(2.)
        .with_child(
            ConstrainedBox::new(UiIcon::GitBranch.to_warpui_icon(text_color).finish())
                .with_width(font_size - 2.)
                .with_height(font_size - 2.)
                .finish(),
        )
        .with_child(
            Shrinkable::new(
                1.,
                Text::new_inline(branch.to_string(), appearance.ui_font_family(), font_size)
                    .with_clip(ClipConfig::ellipsis())
                    .with_color(text_color.into())
                    .finish(),
            )
            .finish(),
        )
        .finish()
}

#[cfg(test)]
#[path = "git_branch_label_tests.rs"]
mod tests;

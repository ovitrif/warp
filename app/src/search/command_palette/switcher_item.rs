use warp_core::ui::Icon as WarpIcon;
use warpui::elements::{
    ConstrainedBox, Container, CrossAxisAlignment, Empty, Flex, MainAxisAlignment, MainAxisSize,
    ParentElement, Shrinkable, Text,
};
use warpui::fonts::{Properties, Weight};
use warpui::text_layout::ClipConfig;
use warpui::{AppContext, Element, EntityId, WindowId};

use super::super::palette_styles::RESULT_PADDING_HORIZONTAL;
use crate::appearance::Appearance;
use crate::pane_group::{PaneGroup, PaneId};
use crate::search::result_renderer::ItemHighlightState;
use crate::ui_components::agent_icon::terminal_view_agent_icon_variant;
use crate::ui_components::git_branch_label::{
    compact_branch_subtitle_display, render_git_branch_text,
};
use crate::ui_components::icon_with_status::{IconWithStatusVariant, render_icon_with_status};

pub(crate) const UNTITLED_GROUP_NAME: &str = "New Group";
const SWITCHER_ICON_SIZE: f32 = 24.;
const SWITCHER_ICON_GAP: f32 = 8.;
/// Match compact vertical-tabs row inset (8px). The palette already applies
/// [`RESULT_PADDING_HORIZONTAL`].
const SWITCHER_ROW_LEFT_INSET: f32 = 8.;

/// Label after ` · ` for a grouped tab. Untitled groups use [`UNTITLED_GROUP_NAME`].
pub(crate) fn group_suffix_label(name: Option<&str>) -> String {
    name.map(str::trim)
        .filter(|name| !name.is_empty())
        .unwrap_or(UNTITLED_GROUP_NAME)
        .to_string()
}

pub(crate) fn pane_index_label(pane_index: Option<usize>) -> Option<String> {
    pane_index.map(|index| index.to_string())
}

pub(crate) fn empty_palette_icon() -> Box<dyn Element> {
    // Unconstrained `Empty` expands to the flex max width and shoves the row
    // contents to the right. Pin it to zero so the agent icon inside the item
    // is the only left-side chrome.
    ConstrainedBox::new(Empty::new().finish())
        .with_width(0.)
        .with_height(0.)
        .finish()
}

pub(crate) fn render_title_with_suffix(
    title: &str,
    suffix: Option<&str>,
    highlight_state: ItemHighlightState,
    appearance: &Appearance,
    trailing: Option<Box<dyn Element>>,
) -> Box<dyn Element> {
    let font_size = appearance.monospace_font_size();
    let title_color = highlight_state.main_text_fill(appearance).into_solid();
    let suffix_color = highlight_state.sub_text_fill(appearance).into_solid();

    let title_el = Shrinkable::new(
        1.,
        Text::new_inline(title.to_string(), appearance.ui_font_family(), font_size)
            .with_clip(ClipConfig::ellipsis())
            .with_color(title_color)
            .with_style(Properties::default().weight(Weight::Bold))
            .finish(),
    )
    .finish();

    let mut title_and_suffix = Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(title_el);

    if let Some(suffix) = suffix.filter(|suffix| !suffix.is_empty()) {
        title_and_suffix.add_child(
            Text::new_inline(
                format!(" · {suffix}"),
                appearance.ui_font_family(),
                font_size,
            )
            .with_color(suffix_color)
            .finish(),
        );
    }

    let mut title_row = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(Shrinkable::new(1., title_and_suffix.finish()).finish());

    if let Some(trailing) = trailing {
        title_row = title_row.with_main_axis_alignment(MainAxisAlignment::SpaceBetween);
        title_row.add_child(trailing);
    }

    title_row.finish()
}

pub(crate) fn render_branch_or_path_subtitle(
    git_branch: Option<&str>,
    working_directory: Option<&str>,
    highlight_state: ItemHighlightState,
    appearance: &Appearance,
) -> Option<Box<dyn Element>> {
    let sub_fill = highlight_state.sub_text_fill(appearance);
    let font_size = appearance.monospace_font_size() - 2.;
    compact_branch_subtitle_display(git_branch, working_directory).map(
        |(text, show_branch_icon)| {
            if show_branch_icon {
                render_git_branch_text(&text, sub_fill, font_size, appearance)
            } else {
                Text::new_inline(text, appearance.ui_font_family(), font_size)
                    .with_clip(ClipConfig::start())
                    .with_color(sub_fill.into_solid())
                    .finish()
            }
        },
    )
}

pub(crate) fn render_switcher_icon(
    window_id: WindowId,
    pane_group_id: EntityId,
    pane_id: Option<PaneId>,
    appearance: &Appearance,
    app: &AppContext,
) -> Box<dyn Element> {
    let theme = appearance.theme();
    let variant = switcher_icon_variant(window_id, pane_group_id, pane_id, appearance, app);
    render_icon_with_status(variant, SWITCHER_ICON_SIZE, 0., theme, theme.background())
}

pub(crate) fn render_switcher_row(
    icon: Box<dyn Element>,
    text: Box<dyn Element>,
) -> Box<dyn Element> {
    let icon = ConstrainedBox::new(icon)
        .with_width(SWITCHER_ICON_SIZE)
        .with_height(SWITCHER_ICON_SIZE)
        .finish();
    Container::new(
        Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_cross_axis_alignment(CrossAxisAlignment::Start)
            .with_spacing(SWITCHER_ICON_GAP)
            .with_child(icon)
            .with_child(Shrinkable::new(1., text).finish())
            .finish(),
    )
    .with_margin_left(-(RESULT_PADDING_HORIZONTAL - SWITCHER_ROW_LEFT_INSET))
    .finish()
}

fn switcher_icon_variant(
    window_id: WindowId,
    pane_group_id: EntityId,
    pane_id: Option<PaneId>,
    appearance: &Appearance,
    app: &AppContext,
) -> IconWithStatusVariant {
    let Some(pane_group_view) = app
        .views_of_type::<PaneGroup>(window_id)
        .into_iter()
        .flatten()
        .find(|view| view.id() == pane_group_id)
    else {
        return fallback_icon_variant(appearance);
    };
    let pane_group = pane_group_view.as_ref(app);
    let pane_id = pane_id.unwrap_or_else(|| pane_group.focused_pane_id(app));
    pane_group
        .terminal_view_from_pane_id(pane_id, app)
        .and_then(|terminal_view| terminal_view_agent_icon_variant(terminal_view.as_ref(app), app))
        .unwrap_or_else(|| fallback_icon_variant(appearance))
}

fn fallback_icon_variant(appearance: &Appearance) -> IconWithStatusVariant {
    let theme = appearance.theme();
    IconWithStatusVariant::Neutral {
        icon: WarpIcon::Terminal,
        icon_color: theme.main_text_color(theme.background()),
    }
}

#[cfg(test)]
#[path = "switcher_item_tests.rs"]
mod tests;

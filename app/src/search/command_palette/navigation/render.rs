use pathfinder_geometry::vector::vec2f;
use warpui::elements::{
    Border, ConstrainedBox, Container, CornerRadius, Flex, ParentElement, Radius, Wrap,
};
use warpui::ui_components::components::{UiComponent, UiComponentStyles};
use warpui::units::IntoPixels;
use warpui::{AppContext, Element, SingletonEntity};

use crate::appearance::Appearance;
use crate::context_chips::display_chip::{
    GitLineChanges, UdiChipConfig, chip_container, render_git_diff_stats_content, render_udi_chip,
    udi_font_size,
};
use crate::context_chips::prompt_snapshot::PromptSnapshot;
use crate::context_chips::{ChipValue, ContextChipKind, github_pr_badge_label_from_url};
use crate::search::command_palette::navigation::search::SessionHighlightIndices;
use crate::search::command_palette::switcher_item::{
    pane_index_label, render_switcher_icon, render_switcher_row, render_title_with_suffix,
};
use crate::search::result_renderer::ItemHighlightState;
use crate::session_management::{CommandContext, SessionNavigationData};
use crate::settings::FontSettings;
use crate::terminal::SizeInfo;
use crate::terminal::blockgrid_element::BlockGridElement;
use crate::terminal::grid_size_util::grid_cell_dimensions;
use crate::terminal::ligature_settings::should_use_ligature_rendering;
use crate::terminal::model::blockgrid::BlockGrid;
use crate::terminal::model::grid::Dimensions;
use crate::terminal::safe_mode_settings::get_secret_obfuscation_mode;

/// Renders a navigation session.
pub fn render_navigation_session(
    session: &SessionNavigationData,
    appearance: &Appearance,
    item_highlight_state: ItemHighlightState,
    is_active_session: bool,
    _highlight_indices: &SessionHighlightIndices,
    app: &AppContext,
) -> Box<dyn Element> {
    let locator = session.pane_view_locator();
    let icon = render_switcher_icon(
        session.window_id(),
        locator.pane_group_id,
        Some(locator.pane_id),
        appearance,
        app,
    );
    let label = render_session_label(
        session,
        appearance,
        item_highlight_state,
        is_active_session,
        app,
    )
    .finish();
    render_navigation_session_internal(render_switcher_row(icon, label))
}

fn render_navigation_session_internal(label: Box<dyn Element>) -> Box<dyn Element> {
    ConstrainedBox::new(label)
        .with_height(styles::NAVIGATION_PALETTE_ITEM_HEIGHT)
        .finish()
}

fn render_session_label(
    session: &SessionNavigationData,
    appearance: &Appearance,
    item_highlight_state: ItemHighlightState,
    is_active_session: bool,
    app: &AppContext,
) -> Flex {
    let mut navigation_palette_item = Flex::column();

    let current_pill = is_active_session.then(|| render_current_session_pill(appearance));
    let pane_suffix = pane_index_label(session.pane_index());
    let title = render_title_with_suffix(
        session.tab_title(),
        pane_suffix.as_deref(),
        item_highlight_state,
        appearance,
        current_pill,
    );

    let prompt = if let Some(ps1_grid) = &session.prompt_elements().ps1_prompt_grid {
        render_prompt_ps1(ps1_grid, appearance, app)
    } else if let Some(snapshot) = &session.prompt_elements().prompt_chip_snapshot {
        render_prompt_udi(snapshot, appearance)
    } else {
        Container::new(Flex::row().finish()).finish()
    };

    navigation_palette_item.add_child(
        Container::new(title)
            .with_margin_right(styles::NAVIGATION_PALETTE_ROW_HORIZONTAL_SPACING)
            .finish(),
    );

    navigation_palette_item.add_child(
        Container::new(prompt)
            .with_margin_top(styles::NAVIGATION_PALETTE_ROW_VERTICAL_SPACING)
            .with_margin_right(styles::NAVIGATION_PALETTE_ROW_HORIZONTAL_SPACING)
            .finish(),
    );

    navigation_palette_item
}

fn render_current_session_pill(appearance: &Appearance) -> Box<dyn Element> {
    appearance
        .ui_builder()
        .span("Current".to_string())
        .with_style(UiComponentStyles {
            font_family_id: Some(appearance.monospace_font_family()),
            font_size: Some(appearance.monospace_font_size() * 0.85),
            font_color: Some(
                appearance
                    .theme()
                    .main_text_color(appearance.theme().background())
                    .into_solid(),
            ),
            ..Default::default()
        })
        .build()
        .with_padding_left(5.)
        .with_padding_right(5.)
        .with_margin_left(10.)
        .with_background_color(appearance.theme().background().into_solid())
        .with_corner_radius(CornerRadius::with_all(Radius::Pixels(4.)))
        .finish()
}

/// Renders the prompt as UDI-style context chips from a [`PromptSnapshot`].
fn render_prompt_udi(snapshot: &PromptSnapshot, appearance: &Appearance) -> Box<dyn Element> {
    let mut chip_row = Wrap::row().with_spacing(4.);

    for chip_result in snapshot.chips() {
        let Some(value) = chip_result.value() else {
            continue;
        };
        // GitDiffStats are rendered differently than other chips, so we handle them separately.
        // This ensures that the rendered chip matches the live input chip.
        if matches!(chip_result.kind(), ContextChipKind::GitDiffStats) {
            let line_changes = match value {
                ChipValue::GitDiffStats(g) => g.clone(),
                ChipValue::Text(raw) => {
                    let Some(parsed) = GitLineChanges::parse_from_git_output(raw) else {
                        continue;
                    };
                    parsed
                }
                ChipValue::GitBranchStatus(_) => continue,
            };
            let font_size = udi_font_size(appearance);
            let content = render_git_diff_stats_content(
                &line_changes,
                font_size,
                appearance.monospace_font_family(),
                font_size,
                appearance,
            );
            chip_row.add_child(chip_container(content, Some(Border::all(0.)), appearance).finish());
            continue;
        }

        let color = chip_result
            .kind()
            .default_styles(appearance, false)
            .value_color;
        let value_text = palette_chip_value_text(chip_result.kind(), value);
        let config = if let Some(icon) = chip_result.kind().udi_icon() {
            UdiChipConfig::new_with_icon(icon, color, value_text)
        } else {
            UdiChipConfig::new(color, value_text)
        }
        .with_border_override(Border::all(0.));
        chip_row.add_child(render_udi_chip(config, appearance));
    }

    let prompt_section = Container::new(chip_row.finish())
        .with_margin_right(styles::NAVIGATION_PALETTE_COMMAND_HINT_MARGIN * 2.);

    prompt_section.finish()
}

fn palette_chip_value_text(kind: &ContextChipKind, value: &ChipValue) -> String {
    match kind {
        ContextChipKind::GithubPullRequest => github_pr_badge_label_from_url(&value.to_string())
            .unwrap_or_else(|| value.to_string()),
        _ => value.to_string(),
    }
}

/// Renders the prompt from the raw PS1 terminal grid, preserving full
/// fidelity of the user's custom prompt (colors, glyphs, etc.).
fn render_prompt_ps1(
    prompt_grid: &BlockGrid,
    appearance: &Appearance,
    app: &AppContext,
) -> Box<dyn Element> {
    let cell_dimensions = grid_cell_dimensions(
        app.font_cache(),
        appearance.monospace_font_family(),
        appearance.monospace_font_size(),
        appearance.line_height_ratio(),
    );
    // Derive the SizeInfo width from the grid's own column count so the
    // element renders at its natural size. The parent flex layout will
    // constrain it to the available palette width.
    let grid_width_px = prompt_grid.grid_handler().columns() as f32 * cell_dimensions.x();
    let size_info = SizeInfo::new(
        vec2f(grid_width_px, cell_dimensions.y()),
        cell_dimensions.x().into_pixels(),
        cell_dimensions.y().into_pixels(),
        0.0.into_pixels(),
        0.0.into_pixels(),
    );
    let enforce_minimum_contrast = *FontSettings::as_ref(app).enforce_minimum_contrast;
    let obfuscate_secrets = get_secret_obfuscation_mode(app);
    let mut block_grid_element = BlockGridElement::new(
        prompt_grid,
        appearance,
        enforce_minimum_contrast,
        obfuscate_secrets,
        size_info,
    );
    if should_use_ligature_rendering(app) {
        block_grid_element = block_grid_element.with_ligature_rendering();
    }

    let prompt_section = Container::new(block_grid_element.finish())
        .with_margin_right(styles::NAVIGATION_PALETTE_COMMAND_HINT_MARGIN * 2.);

    prompt_section.finish()
}

pub(super) fn command_hint_text(command_context: CommandContext) -> String {
    match command_context {
        CommandContext::RunningCommand { .. } | CommandContext::RunningAIBlock { .. } => {
            "Running...".to_string()
        }
        CommandContext::LastRunCommand {
            mins_since_completion,
            ..
        } => match mins_since_completion {
            Some(mins) if mins >= 60 => "Completed over 1 hour ago".to_string(),
            Some(mins) if mins == 1 => format!("Completed {mins} minute ago"),
            Some(mins) => format!("Completed {mins} minutes ago"),
            None => "No timestamp found".to_string(),
        },
        CommandContext::LastRunAIBlock { .. } => "Completed".to_string(),
        CommandContext::None => "Empty Session".to_string(),
    }
}

mod styles {
    pub const NAVIGATION_PALETTE_ITEM_HEIGHT: f32 = 70.;

    pub const NAVIGATION_PALETTE_ROW_VERTICAL_SPACING: f32 = 4.;

    pub const NAVIGATION_PALETTE_ROW_HORIZONTAL_SPACING: f32 = 5.;

    pub const NAVIGATION_PALETTE_COMMAND_HINT_MARGIN: f32 = 5.;
}

#[cfg(test)]
#[path = "render_tests.rs"]
mod tests;

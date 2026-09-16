use ordered_float::OrderedFloat;
use warpui::elements::{ConstrainedBox, Container, Flex, ParentElement};
use warpui::{AppContext, Element, SingletonEntity};

use crate::appearance::Appearance;
use crate::search::command_palette::mixer::CommandPaletteItemAction;
use crate::search::command_palette::switcher_item::{
    empty_palette_icon, render_branch_or_path_subtitle, render_switcher_icon, render_switcher_row,
    render_title_with_suffix,
};
use crate::search::item::{IconLocation, SearchItem as SearchItemTrait};
use crate::search::result_renderer::ItemHighlightState;
use crate::session_management::TabNavigationData;

/// These items appear in the ctrl-tab palette only, not the main command palette.
/// Scoring matches against queries is not supported since only ranking by recency is needed.
pub struct SearchItem {
    tab: TabNavigationData,
    mru_rank: usize,
}

impl SearchItem {
    pub fn new(tab: TabNavigationData, mru_rank: usize) -> Self {
        Self { tab, mru_rank }
    }
}

impl SearchItemTrait for SearchItem {
    type Action = CommandPaletteItemAction;

    fn is_multiline(&self) -> bool {
        true
    }

    fn render_icon(
        &self,
        _highlight_state: ItemHighlightState,
        _appearance: &Appearance,
    ) -> Box<dyn Element> {
        empty_palette_icon()
    }

    fn icon_location(&self, _appearance: &Appearance) -> IconLocation {
        IconLocation::Top { margin_top: 0. }
    }

    fn render_item(
        &self,
        highlight_state: ItemHighlightState,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let appearance = Appearance::as_ref(app);
        let icon = render_switcher_icon(
            self.tab.window_id,
            self.tab.pane_group_id,
            None,
            appearance,
            app,
        );
        let title = render_title_with_suffix(
            &self.tab.title,
            self.tab.group_name.as_deref(),
            highlight_state,
            appearance,
            None,
        );

        let mut contents = Flex::column().with_child(title);
        if let Some(subtitle) = render_branch_or_path_subtitle(
            self.tab.git_branch.as_deref(),
            self.tab.subtitle.as_deref(),
            highlight_state,
            appearance,
        ) {
            contents.add_child(Container::new(subtitle).with_padding_top(4.).finish());
        }

        ConstrainedBox::new(render_switcher_row(icon, contents.finish()))
            .with_height(50.)
            .finish()
    }

    fn score(&self) -> OrderedFloat<f64> {
        OrderedFloat::from(1000.0 - self.mru_rank as f64)
    }

    fn accept_result(&self) -> Self::Action {
        CommandPaletteItemAction::NavigateToTab {
            pane_group_id: self.tab.pane_group_id,
            window_id: self.tab.window_id,
        }
    }

    fn execute_result(&self) -> Self::Action {
        self.accept_result()
    }

    fn accessibility_label(&self) -> String {
        match self.tab.group_name.as_deref() {
            Some(group) => format!("Selected tab: {} in {}.", self.tab.title, group),
            None => format!("Selected tab: {}.", self.tab.title),
        }
    }

    fn accessibility_help_message(&self) -> Option<String> {
        Some(format!(
            "Press enter to navigate to tab: {}.",
            self.tab.title
        ))
    }
}

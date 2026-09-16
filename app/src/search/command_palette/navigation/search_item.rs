use ordered_float::OrderedFloat;
use warpui::elements::Container;
use warpui::{AppContext, Element, SingletonEntity};

use crate::appearance::Appearance;
use crate::pane_group::PaneId;
use crate::search::command_palette::mixer::CommandPaletteItemAction;
use crate::search::command_palette::navigation::render::render_navigation_session;
use crate::search::command_palette::navigation::search::MatchedSession;
use crate::search::command_palette::switcher_item::empty_palette_icon;
use crate::search::item::IconLocation;
use crate::search::result_renderer::ItemHighlightState;
use crate::session_management::SessionNavigationData;

/// Search item to render a session within the command palette.
pub struct SearchItem {
    matched_session: MatchedSession,
    /// The current active session. `None` if there is no active session or we were
    /// unable to determine which session is currently active.
    active_session: Option<PaneId>,
}

impl SearchItem {
    fn navigation_data(&self) -> &SessionNavigationData {
        &self.matched_session.session
    }

    pub fn new(matched_session: MatchedSession, active_session: Option<PaneId>) -> Self {
        Self {
            matched_session,
            active_session,
        }
    }
}

impl crate::search::item::SearchItem for SearchItem {
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
        let is_active_session = self
            .active_session
            .is_some_and(|id| self.navigation_data().is_for_session(id));

        let session_element = render_navigation_session(
            self.navigation_data(),
            appearance,
            highlight_state,
            is_active_session,
            self.matched_session.highlight_indices(),
            app,
        );
        Container::new(session_element).finish()
    }

    fn render_details(&self, _: &AppContext) -> Option<Box<dyn Element>> {
        // Navigation search items don't support rendering a details panel.
        None
    }

    fn score(&self) -> OrderedFloat<f64> {
        OrderedFloat::from(self.matched_session.score() as f64)
    }

    fn accept_result(&self) -> Self::Action {
        CommandPaletteItemAction::NavigateToSession {
            pane_view_locator: self.navigation_data().pane_view_locator(),
            window_id: self.navigation_data().window_id(),
        }
    }

    fn execute_result(&self) -> Self::Action {
        self.accept_result()
    }

    fn accessibility_label(&self) -> String {
        let title = self.navigation_data().tab_title();
        match self.navigation_data().pane_index() {
            Some(index) => format!("Selected {title} pane {index}."),
            None => format!("Selected {title}."),
        }
    }

    fn accessibility_help_message(&self) -> Option<String> {
        Some("Press enter to navigate to this session.".into())
    }
}

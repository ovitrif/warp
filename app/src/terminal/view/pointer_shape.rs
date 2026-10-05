use warp_terminal::model::pointer_shape::PointerShape;
use warpui::ViewContext;
use warpui::platform::Cursor;

use super::TerminalView;

/// The mouse cursor for a program-requested pointer shape. `None` leaves Warp's normal cursor
/// logic in charge, which is also the fallback for shapes the platform has no cursor for.
pub(super) fn cursor_for_pointer_shape(shape: PointerShape) -> Option<Cursor> {
    match shape {
        PointerShape::Pointer => Some(Cursor::PointingHand),
        PointerShape::Text | PointerShape::VerticalText => Some(Cursor::IBeam),
        PointerShape::Crosshair
        | PointerShape::Cell
        | PointerShape::NeResize
        | PointerShape::NwResize
        | PointerShape::SeResize
        | PointerShape::SwResize
        | PointerShape::NeswResize
        | PointerShape::NwseResize => Some(Cursor::Crosshair),
        PointerShape::Grab | PointerShape::Move => Some(Cursor::OpenHand),
        PointerShape::Grabbing => Some(Cursor::ClosedHand),
        PointerShape::NotAllowed | PointerShape::NoDrop => Some(Cursor::NotAllowed),
        PointerShape::EResize | PointerShape::WResize | PointerShape::EwResize => {
            Some(Cursor::ResizeLeftRight)
        }
        PointerShape::NResize | PointerShape::SResize | PointerShape::NsResize => {
            Some(Cursor::ResizeUpDown)
        }
        PointerShape::Copy | PointerShape::Alias => Some(Cursor::DragCopy),
        PointerShape::Default
        | PointerShape::Help
        | PointerShape::Progress
        | PointerShape::Wait
        | PointerShape::ZoomIn
        | PointerShape::ZoomOut => None,
    }
}

impl TerminalView {
    /// Applies the pointer shape that the running program requested for the grid cell under the
    /// mouse, and restores Warp's cursor once the program no longer controls that cell.
    ///
    /// Warp's own cursors win: link hover and text selection suppress the program's shape.
    pub(super) fn sync_program_pointer_shape(&mut self, ctx: &mut ViewContext<Self>) {
        let desired = self.desired_program_cursor(ctx);
        if desired == self.program_pointer_cursor {
            return;
        }
        let previous = std::mem::replace(&mut self.program_pointer_cursor, desired);
        match desired {
            Some(cursor) => {
                log::debug!("Applying program-requested mouse cursor {cursor:?}");
                ctx.set_cursor_shape(cursor);
            }
            // A highlighted link has just claimed the cursor, so it must not be reset.
            None if previous.is_some() && self.highlighted_link.is_none() => {
                log::debug!("Restoring the default mouse cursor");
                ctx.reset_cursor();
            }
            None => {}
        }
    }

    fn desired_program_cursor(&self, ctx: &mut ViewContext<Self>) -> Option<Cursor> {
        let position = self.last_grid_hover.as_ref()?;
        if self.highlighted_link.is_some() || self.is_navigated_away_from_window(ctx) {
            return None;
        }
        let model = self.model.lock();
        if self.terminal_is_selecting(&model, ctx) {
            return None;
        }
        cursor_for_pointer_shape(model.program_pointer_shape_at(position)?)
    }
}

#[cfg(test)]
#[path = "pointer_shape_tests.rs"]
mod tests;

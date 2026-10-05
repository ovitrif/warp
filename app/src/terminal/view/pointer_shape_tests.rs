use warp_terminal::model::pointer_shape::PointerShape;
use warpui::platform::Cursor;

use super::cursor_for_pointer_shape;

#[test]
fn maps_basic_shapes_to_native_cursors() {
    let cursor = |name| cursor_for_pointer_shape(PointerShape::from_name(name).unwrap());
    assert_eq!(cursor("pointer"), Some(Cursor::PointingHand));
    assert_eq!(cursor("hand2"), Some(Cursor::PointingHand));
    assert_eq!(cursor("text"), Some(Cursor::IBeam));
    assert_eq!(cursor("xterm"), Some(Cursor::IBeam));
    assert_eq!(cursor("crosshair"), Some(Cursor::Crosshair));
    assert_eq!(cursor("ew-resize"), Some(Cursor::ResizeLeftRight));
    assert_eq!(cursor("ns-resize"), Some(Cursor::ResizeUpDown));
    assert_eq!(cursor("grab"), Some(Cursor::OpenHand));
    assert_eq!(cursor("grabbing"), Some(Cursor::ClosedHand));
    assert_eq!(cursor("not-allowed"), Some(Cursor::NotAllowed));
}

#[test]
fn default_and_unrepresentable_shapes_keep_the_normal_cursor() {
    let cursor = |name| cursor_for_pointer_shape(PointerShape::from_name(name).unwrap());
    assert_eq!(cursor("default"), None);
    assert_eq!(cursor("left_ptr"), None);
    assert_eq!(cursor("wait"), None);
    assert_eq!(cursor("help"), None);
}

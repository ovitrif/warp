//! Mouse pointer shapes that programs request with OSC 22.
//!
//! See <https://sw.kovidgoyal.net/kitty/pointer-shapes/>.

/// Kitty requires terminals to keep at least this many entries per stack.
const MAX_STACK_DEPTH: usize = 16;

const CURRENT_QUERY: &str = "__current__";
const DEFAULT_QUERY: &str = "__default__";
const GRABBED_QUERY: &str = "__grabbed__";

/// The CSS pointer shapes that kitty defines for OSC 22.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerShape {
    Alias,
    Cell,
    Copy,
    Crosshair,
    Default,
    EResize,
    EwResize,
    Grab,
    Grabbing,
    Help,
    Move,
    NResize,
    NeResize,
    NeswResize,
    NoDrop,
    NotAllowed,
    NsResize,
    NwResize,
    NwseResize,
    Pointer,
    Progress,
    SResize,
    SeResize,
    SwResize,
    Text,
    VerticalText,
    WResize,
    Wait,
    ZoomIn,
    ZoomOut,
}

impl PointerShape {
    /// Resolves a CSS shape name, or one of the xterm cursor-font aliases, to a shape.
    pub fn from_name(name: &str) -> Option<Self> {
        let shape = match name {
            "alias" => Self::Alias,
            "cell" => Self::Cell,
            "copy" => Self::Copy,
            "crosshair" | "cross" | "tcross" => Self::Crosshair,
            "default" | "left_ptr" | "arrow" => Self::Default,
            "e-resize" | "right_side" => Self::EResize,
            "ew-resize" | "sb_h_double_arrow" | "h_double_arrow" => Self::EwResize,
            "grab" | "hand1" => Self::Grab,
            "grabbing" => Self::Grabbing,
            "help" | "question_arrow" => Self::Help,
            "move" | "fleur" => Self::Move,
            "n-resize" | "top_side" => Self::NResize,
            "ne-resize" | "top_right_corner" => Self::NeResize,
            "nesw-resize" => Self::NeswResize,
            "no-drop" => Self::NoDrop,
            "not-allowed" | "circle" => Self::NotAllowed,
            "ns-resize" | "sb_v_double_arrow" | "v_double_arrow" => Self::NsResize,
            "nw-resize" | "top_left_corner" => Self::NwResize,
            "nwse-resize" => Self::NwseResize,
            "pointer" | "hand2" | "hand" => Self::Pointer,
            "progress" => Self::Progress,
            "s-resize" | "bottom_side" => Self::SResize,
            "se-resize" | "bottom_right_corner" => Self::SeResize,
            "sw-resize" | "bottom_left_corner" => Self::SwResize,
            "text" | "xterm" | "ibeam" => Self::Text,
            "vertical-text" => Self::VerticalText,
            "w-resize" | "left_side" => Self::WResize,
            "wait" | "watch" => Self::Wait,
            "zoom-in" => Self::ZoomIn,
            "zoom-out" => Self::ZoomOut,
            _ => return None,
        };
        Some(shape)
    }

    /// The canonical CSS name of the shape.
    pub fn name(self) -> &'static str {
        match self {
            Self::Alias => "alias",
            Self::Cell => "cell",
            Self::Copy => "copy",
            Self::Crosshair => "crosshair",
            Self::Default => "default",
            Self::EResize => "e-resize",
            Self::EwResize => "ew-resize",
            Self::Grab => "grab",
            Self::Grabbing => "grabbing",
            Self::Help => "help",
            Self::Move => "move",
            Self::NResize => "n-resize",
            Self::NeResize => "ne-resize",
            Self::NeswResize => "nesw-resize",
            Self::NoDrop => "no-drop",
            Self::NotAllowed => "not-allowed",
            Self::NsResize => "ns-resize",
            Self::NwResize => "nw-resize",
            Self::NwseResize => "nwse-resize",
            Self::Pointer => "pointer",
            Self::Progress => "progress",
            Self::SResize => "s-resize",
            Self::SeResize => "se-resize",
            Self::SwResize => "sw-resize",
            Self::Text => "text",
            Self::VerticalText => "vertical-text",
            Self::WResize => "w-resize",
            Self::Wait => "wait",
            Self::ZoomIn => "zoom-in",
            Self::ZoomOut => "zoom-out",
        }
    }
}

/// A stack operation requested by an OSC 22 sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointerShapeOp {
    /// Replaces the top of the stack. An empty list removes the top entry.
    Set(Vec<PointerShape>),
    Push(Vec<PointerShape>),
    Pop,
}

/// A parsed OSC 22 request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointerShapeRequest<'a> {
    Op(PointerShapeOp),
    /// The comma-separated names of a `?` query, unparsed.
    Query(Vec<&'a str>),
}

impl<'a> PointerShapeRequest<'a> {
    /// Parses the payload that follows `OSC 22 ;`. Returns `None` when the request names shapes
    /// but none of them are known, so that such a request leaves the stack untouched.
    pub fn parse(payload: &'a str) -> Option<Self> {
        let shapes = |names: &str| {
            let shapes: Vec<_> = names
                .split(',')
                .filter_map(|name| PointerShape::from_name(name.trim()))
                .collect();
            (!shapes.is_empty() || names.trim().is_empty()).then_some(shapes)
        };
        let request = match payload.chars().next() {
            Some('?') => Self::Query(payload[1..].split(',').map(str::trim).collect()),
            Some('>') => Self::Op(PointerShapeOp::Push(shapes(&payload[1..])?)),
            Some('<') => Self::Op(PointerShapeOp::Pop),
            Some('=') => Self::Op(PointerShapeOp::Set(shapes(&payload[1..])?)),
            _ => Self::Op(PointerShapeOp::Set(shapes(payload)?)),
        };
        Some(request)
    }
}

/// Terminal state that OSC 22 queries can ask about.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PointerShapeReport {
    pub current: Option<PointerShape>,
    /// Whether a program has enabled mouse reporting.
    pub grabbed: bool,
}

/// Builds the reply to a `?` query: a comma-separated entry per queried name. Shape names answer
/// `1` when supported and `0` otherwise, and the special names answer with their value.
pub fn format_query_reply(names: &[&str], report: PointerShapeReport, terminator: &str) -> String {
    let entries: Vec<String> = names
        .iter()
        .map(|name| match *name {
            CURRENT_QUERY => report
                .current
                .map(|shape| shape.name().to_owned())
                .unwrap_or_default(),
            DEFAULT_QUERY => PointerShape::Default.name().to_owned(),
            GRABBED_QUERY => u8::from(report.grabbed).to_string(),
            name => u8::from(PointerShape::from_name(name).is_some()).to_string(),
        })
        .collect();
    format!("\x1b]22;{}{terminator}", entries.join(","))
}

/// The shapes a program has pushed for one screen.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PointerShapeStack {
    entries: Vec<PointerShape>,
}

impl PointerShapeStack {
    pub fn current(&self) -> Option<PointerShape> {
        self.entries.last().copied()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn apply(&mut self, op: PointerShapeOp) {
        match op {
            PointerShapeOp::Set(shapes) => {
                self.entries.pop();
                self.push_all(shapes.last().copied());
            }
            PointerShapeOp::Push(shapes) => self.push_all(shapes),
            PointerShapeOp::Pop => {
                self.entries.pop();
            }
        }
    }

    fn push_all(&mut self, shapes: impl IntoIterator<Item = PointerShape>) {
        for shape in shapes {
            if self.entries.len() == MAX_STACK_DEPTH {
                self.entries.remove(0);
            }
            self.entries.push(shape);
        }
    }
}

#[cfg(test)]
#[path = "pointer_shape_tests.rs"]
mod tests;

use super::*;

fn parse(payload: &str) -> Option<PointerShapeRequest<'_>> {
    PointerShapeRequest::parse(payload)
}

#[test]
fn resolves_css_names_and_xterm_aliases() {
    assert_eq!(
        PointerShape::from_name("pointer"),
        Some(PointerShape::Pointer)
    );
    assert_eq!(
        PointerShape::from_name("hand2"),
        Some(PointerShape::Pointer)
    );
    assert_eq!(
        PointerShape::from_name("left_ptr"),
        Some(PointerShape::Default)
    );
    assert_eq!(PointerShape::from_name("xterm"), Some(PointerShape::Text));
    assert_eq!(
        PointerShape::from_name("sb_h_double_arrow"),
        Some(PointerShape::EwResize)
    );
    assert_eq!(PointerShape::from_name("no-such-shape"), None);
}

#[test]
fn canonical_names_round_trip() {
    for name in [
        "alias",
        "cell",
        "copy",
        "crosshair",
        "default",
        "e-resize",
        "ew-resize",
        "grab",
        "grabbing",
        "help",
        "move",
        "n-resize",
        "ne-resize",
        "nesw-resize",
        "no-drop",
        "not-allowed",
        "ns-resize",
        "nw-resize",
        "nwse-resize",
        "pointer",
        "progress",
        "s-resize",
        "se-resize",
        "sw-resize",
        "text",
        "vertical-text",
        "w-resize",
        "wait",
        "zoom-in",
        "zoom-out",
    ] {
        assert_eq!(
            PointerShape::from_name(name).map(PointerShape::name),
            Some(name)
        );
    }
}

#[test]
fn parses_set_push_pop_and_query() {
    let op = |op| Some(PointerShapeRequest::Op(op));
    assert_eq!(
        parse("pointer"),
        op(PointerShapeOp::Set(vec![PointerShape::Pointer]))
    );
    assert_eq!(
        parse("=crosshair"),
        op(PointerShapeOp::Set(vec![PointerShape::Crosshair]))
    );
    assert_eq!(
        parse(">grab,grabbing"),
        op(PointerShapeOp::Push(vec![
            PointerShape::Grab,
            PointerShape::Grabbing
        ]))
    );
    assert_eq!(parse("<"), op(PointerShapeOp::Pop));
    assert_eq!(
        parse("?pointer,bogus"),
        Some(PointerShapeRequest::Query(vec!["pointer", "bogus"]))
    );
}

#[test]
fn empty_set_resets_and_unknown_names_are_ignored() {
    assert_eq!(
        parse(""),
        Some(PointerShapeRequest::Op(PointerShapeOp::Set(vec![])))
    );
    assert_eq!(parse("bogus"), None);
    assert_eq!(parse(">bogus"), None);
    assert_eq!(
        parse("bogus,text"),
        Some(PointerShapeRequest::Op(PointerShapeOp::Set(vec![
            PointerShape::Text
        ])))
    );
}

#[test]
fn stack_set_replaces_top_and_empty_set_removes_it() {
    let mut stack = PointerShapeStack::default();
    stack.apply(PointerShapeOp::Set(vec![PointerShape::Pointer]));
    stack.apply(PointerShapeOp::Set(vec![PointerShape::Text]));
    assert_eq!(stack.current(), Some(PointerShape::Text));
    stack.apply(PointerShapeOp::Set(vec![]));
    assert!(stack.is_empty());
}

#[test]
fn stack_push_and_pop() {
    let mut stack = PointerShapeStack::default();
    stack.apply(PointerShapeOp::Push(vec![PointerShape::Wait]));
    stack.apply(PointerShapeOp::Push(vec![
        PointerShape::Grab,
        PointerShape::Grabbing,
    ]));
    assert_eq!(stack.current(), Some(PointerShape::Grabbing));
    stack.apply(PointerShapeOp::Pop);
    assert_eq!(stack.current(), Some(PointerShape::Grab));
    stack.apply(PointerShapeOp::Pop);
    stack.apply(PointerShapeOp::Pop);
    stack.apply(PointerShapeOp::Pop);
    assert_eq!(stack.current(), None);
}

#[test]
fn stack_drops_the_oldest_entry_when_full() {
    let mut stack = PointerShapeStack::default();
    for _ in 0..MAX_STACK_DEPTH {
        stack.apply(PointerShapeOp::Push(vec![PointerShape::Text]));
    }
    stack.apply(PointerShapeOp::Push(vec![PointerShape::Pointer]));
    assert_eq!(stack.entries.len(), MAX_STACK_DEPTH);
    assert_eq!(stack.current(), Some(PointerShape::Pointer));
}

#[test]
fn query_reply_answers_per_name() {
    let report = PointerShapeReport {
        current: Some(PointerShape::Text),
        grabbed: true,
    };
    assert_eq!(
        format_query_reply(&["pointer", "bogus", "hand2"], report, "\x1b\\"),
        "\x1b]22;1,0,1\x1b\\"
    );
    assert_eq!(
        format_query_reply(
            &["__current__", "__default__", "__grabbed__"],
            report,
            "\x07"
        ),
        "\x1b]22;text,default,1\x07"
    );
    assert_eq!(
        format_query_reply(&["__current__"], PointerShapeReport::default(), "\x07"),
        "\x1b]22;\x07"
    );
}

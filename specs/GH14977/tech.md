# Tech Spec: Cmd-hold tab shortcut overlay

**Product spec:** `specs/GH14977/product.md`
**Issue:** [warpdotdev/warp#14977](https://github.com/warpdotdev/warp/issues/14977)
**Prefer rebasing** [PR #15221](https://github.com/warpdotdev/warp/pull/15221) and swapping paint, not a second modifier tracker.

## Context

Bindings already exist in `app/src/workspace/mod.rs`:

- `workspace:activate_first_tab` … `eighth_tab` → `ActivateTabByNumber(1..=8)`
- `workspace:activate_last_tab` → `ActivateLastTab` (`cmdorctrl-9`)

#15221 adds:

- `TabShortcutModifierState` singleton + `Event::ModifierKeyChanged`
- `TAB_ACTIVATE_BINDING_NAMES`, `reveals_tab_shortcut_hints`
- `keybinding_name_to_display_string` labels
- Blur reset on `Workspace::handle_window_state_change`
- Tests in `tab.rs` / `vertical_tabs_tests.rs`

That plumbing is the right substrate. This spec only changes **where and what** is painted.

## Paint

Trailing keystroke via `theme.sub_text_color(theme.background())` — no
hardcoded colors.

| Surface | Anchor |
| --- | --- |
| Horizontal `TabComponent` | trailing, replacing nothing but sitting next to title |
| Vertical Panes view with tab header | trailing on `render_group_header` |
| Vertical pane row when it *is* the tab (no header) | trailing on the title line |
| Inner pane rows of a multi-pane tab | never |
| Tab-group header (`render_grouped_tabs_header`) | never — those are not Cmd+N targets |

## Mapping

```rust
fn tab_shortcut_number(index: usize, tab_count: usize) -> Option<u8> {
    if index < 8 {
        return Some((index + 1) as u8);
    }
    if tab_count >= 9 && index + 1 == tab_count {
        return Some(9);
    }
    None
}
```

Paint the last-tab binding (`workspace:activate_last_tab`) on the last tab
when `tab_count >= 9`. The label is the effective keystroke (⌘9 by default).

Collapsed group members are not painted. Visible rows still use **global**
index so Cmd+N never lies.

## Files (if implementing as overlay on #15221)

- Keep `TabShortcutModifierState` and reveal gating from #15221.
- `app/src/tab.rs` — replace `render_shortcut_hint` text with overlay.
- `app/src/workspace/view/vertical_tabs.rs` — overlay on
  `render_pane_icon_with_status` / summary icon, drop trailing
  `shortcut_hint` in `render_row_title_line`.
- Tests: `shows_shortcut_hint` truth table plus overlay mapping cases
  (1, 8, 9-of-9, 9-of-10, collapsed group).

## Delay

#15221's PR text describes a debounce. Match Cursor at ~400ms show, 0ms hide:
gate `reveals_tab_shortcut_hints` on hold elapsed so incidental re-renders
during the delay cannot flash hints. Hide immediately on release.

# Product Spec: Cmd-hold tab shortcut hints

**Issue:** [warpdotdev/warp#14977](https://github.com/warpdotdev/warp/issues/14977)
**In-flight PR:** [warpdotdev/warp#15221](https://github.com/warpdotdev/warp/pull/15221)
**Do not file a new issue.** Cherry-pick #15221 and restyle so Panes-view
hints sit on the tab, not on inner pane rows.

## Summary

While the modifier used by `workspace:activate_*_tab` is held, Warp paints
the effective keystroke (⌘1…⌘8, and ⌘9 on the last tab) on each **tab**.
Release hides it. Navigation is unchanged.

Hold-to-reveal matches the product call on #14977. Trailing placement matches
#15221. The restyle is: in vertical Panes view, the colored card is the tab
and inner rows are panes — paint once on the tab header.

## Problem

Cmd+1…8 and Cmd+9 (last tab) already switch tabs. Nothing in the UI shows
which tab is which until you try. Vertical tabs with colored groups make
this worse: the visual hierarchy is groups, the shortcut model is a flat
index.

#14977 originally asked for an always-on setting. The team switched to
hold-to-reveal (no setting). #15221 implements that with trailing labels.

## Goals

- Reveal indexes only while the relevant modifier is held (Cmd by default).
- Trailing shortcut on the **tab**, matching PR #15221. In vertical Panes
  view the colored card is the tab; inner rows are panes and stay unlabeled.
- Same mapping as the bindings: tabs 1–8, and 9 on the last tab when there
  are 9+ tabs.
- Horizontal tab bar, compact tabs, and vertical grouped tabs.
- Binding-aware: remapped modifiers reveal with the remapped key; unbound
  tabs show nothing.
- ~100ms show delay (Codex). Hide immediately on release or window blur.

## Non-goals

- A new Settings toggle (#14977 dropped this).
- Group-relative indexes (#13398). Tab-group headers are not Cmd+N targets.
- Always-visible tab numbers (#4028, #9046).
- Painting overflowed / scrolled-off / collapsed-group members.
- New navigation actions.

## User-visible behavior

1. Hold Cmd (or the modifier the tab-switch bindings actually use) for
   ~100ms. Trailing ⌘1–⌘8 appear on the first eight tabs, in global order.
2. If there are 9+ tabs, the last visible tab also gets 9 (`activate_last_tab`).
   A ninth tab that is not last stays unlabeled.
3. Release, press a non-modifier, or unfocus the window: overlays vanish.
4. Vertical Panes view: the hint sits on the tab header (the colored card
   title). Inner pane rows stay unlabeled.
5. Compact horizontal tabs: trailing label still, shrinking with the tab.
6. Linux / Windows follow the binding (Ctrl unless remapped).

## Related

| Item | Relation |
| --- | --- |
| #14977 | Canonical request. Hold-to-reveal, no setting. |
| #15221 | Open implementation: same trigger, trailing `⌘N` paint. Prefer reuse of its modifier state. |
| #13398 | Two-level group navigation. Out of scope. |
| #4028, #9046 | Always-on numbers. Superseded. |

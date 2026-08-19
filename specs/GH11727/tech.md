# Tech Spec: Identify Grok Build (`grok`) as a first-class CLI agent

**Issue:** [warpdotdev/warp#11727](https://github.com/warpdotdev/warp/issues/11727)
**Product spec:** `specs/GH11727/product.md`

## Context

CLI-agent identity is a single enum, `CLIAgent` in
`app/src/terminal/cli_agent.rs`. Detection is prefix-based: the first
command token's basename is compared to `command_prefixes()`. A match
creates a `CLIAgentSession` that:

- drives the vertical-tab / sidebar mark via
  `terminal_view_agent_icon_variant` → `IconWithStatusVariant::CLIAgent`
  (`app/src/ui_components/agent_icon.rs`,
  `app/src/ui_components/icon_with_status.rs`)
- shows the CLI-agent footer (`app/src/terminal/view/use_agent_footer`)
- appears in Settings (`enum_iterator::all::<CLIAgent>()`)

`Icon::GrokLogo` already exists (`crates/warp_core/src/ui/icons.rs`) and
points at `app/assets/bundled/svg/grok.svg` — the official Grok black-hole
logomark, already used for the xAI LLM provider. The SVG uses the same
`#FF0000` mask fill as `claude.svg` / `openai.svg` so Warp can tint it.

`grok` is **not** in `command_prefixes()`, `CLIAgentType`, the plugin
manager, the session listener, the rich-input submit strategy, or the
input-classifier one-off shell keywords.

Related open work (do not duplicate blindly):

- [#14228](https://github.com/warpdotdev/warp/pull/14228) — fuller Grok
  Build stack (plugin installer under `$GROK_HOME/hooks`, dedicated OSC 9
  handler). Blocked; last updated 2026-08-11.
- [#14780](https://github.com/warpdotdev/warp/pull/14780) — `Grok` /
  `QwenCode` / `KimiCode` recognition with `icon() = None`.

This change is the identification slice: detect `grok`, reuse
`Icon::GrokLogo`, wire exhaustive matches, add tests.

## Proposed changes

### 1. `CLIAgent::Grok` (`app/src/terminal/cli_agent.rs`)

Add the variant (before `WarpTui`, keep `Unknown` last) and fill every
exhaustive arm:

| Method | Value |
| --- | --- |
| `command_prefixes()` | `["grok"]` |
| `display_name()` | `"Grok Build"` |
| `icon()` | `Some(Icon::GrokLogo)` |
| `brand_color()` | black (`ColorU { r:0, g:0, b:0, a:255 }`) — same family as Codex / Warp TUI |
| `brand_icon_color()` | white (default arm) so the mark reads as Grok_Logomark_Light on the black tile |
| `supported_skill_providers()` | `&[SkillProvider::Agents]` (Grok Build reads `.agents/skills`) |
| `supports_bash_mode()` | `false` until `!` mode is verified |
| `supports_cli_agent_footer()` | `true` (not `WarpTui`) |
| `From<CLIAgent> for CLIAgentType` | `CLIAgentType::Grok` |

`from_harness` stays unchanged: there is no `Harness::Grok`.

New `GROK_COLOR` constant documents the brand black.

### 2. Telemetry (`app/src/server/telemetry/events.rs`)

Add `CLIAgentType::Grok` next to the other known agents.

### 3. Session listener (`cli_agent_sessions/listener/mod.rs`)

Add `CLIAgent::Grok` to `is_agent_supported` and `create_handler`'s
`DefaultSessionListener` cohort (same as Auggie / Pi / Droid / WarpTui).
Grok may already emit OSC 777 via a user-installed plugin; we listen, we
do not install. No Codex-style OSC 9 handler in this change.

### 4. Plugin manager (`cli_agent_sessions/plugin_manager/mod.rs`)

Add `CLIAgent::Grok` to the `None` arm. No `GrokPluginManager`.

### 5. Rich-input submit (`view/use_agent_footer/mod.rs`)

Put `CLIAgent::Grok` on `RichInputSubmitStrategy::DelayedEnter` with
Claude / OpenCode / Gemini / Auggie / Cursor. Grok Build is a TUI; a
same-buffer `\r` is the failure mode that DelayedEnter exists for.

### 6. Input classifier (`crates/input_classifier/src/util.rs`)

Add `"grok"` to `ONE_OFF_SHELL_COMMAND_KEYWORDS` so bare `grok` classifies
as shell, matching `claude` / `codex` / `gemini` / `agy` / `omp` / `warp`.

### 7. Tests

- Extend `test_detect_known_agents` with `("grok", CLIAgent::Grok)`.
- Add `test_grok_variant_properties` (prefix, display name, icon, color,
  telemetry round-trip, no bash mode) modeled on
  `test_warp_tui_variant_properties`.
- Add `test_grok_matches_command` / negative lookalikes
  (`grokking`, `mygrok`, empty).
- Listener: `is_agent_supported(&CLIAgent::Grok)` and a parse/forward
  smoke test for an OSC 777 `stop` payload with `"agent":"grok"`.
- Plugin manager: `plugin_manager_for(CLIAgent::Grok)` is `None`.
- Footer: `rich_input_submit_strategy(CLIAgent::Grok) == DelayedEnter`.

Existing `test_serialized_name_round_trips_known_agents` and the Settings
dropdown `all::<CLIAgent>()` loop pick the new variant up automatically
(serde name `Grok`).

## Data flow

```
user runs `grok` in a pane
  → CLIAgent::detect("grok", …) == Some(CLIAgent::Grok)
  → CLIAgentSessionsModel records a session
  → terminal_view_agent_icon_variant → IconWithStatusVariant::CLIAgent { Grok }
  → sidebar / vertical tabs paint Icon::GrokLogo on GROK_COLOR
  → use_agent_footer shows the Grok Build toolbar
  → optional OSC 777 events update session status
```

## Tradeoffs

- **Reuse `Icon::GrokLogo` rather than adding Light/Dark official files.**
  Warp's CLI-agent pipeline tints a single monochrome SVG onto a brand
  circle (Claude, Codex, Gemini). Dual official Light/Dark SVGs from the
  SpaceXAI zip would fight that pipeline and risk implying endorsement
  via an unaltered lockup. Geometry is the official black-hole logomark;
  framing matches every other agent tile.
- **No plugin installer.** Identity does not depend on hooks. #14228 can
  layer install on top of `CLIAgent::Grok`.
- **DelayedEnter without a Grok-specific paste study.** Safer default for
  a TUI; one-line change later if Grok needs BracketedPaste.

## Testing and validation

| Product invariant | How verified |
| --- | --- |
| 1. `grok` detects as Grok Build | Unit: `CLIAgent::detect("grok", …)` and path/args cases |
| 2. Sidebar shows official logomark | Unit: `icon() == Some(Icon::GrokLogo)`, `brand_color() == black`; manual `./script/run` + computer-use screenshot of a live `grok` pane |
| 3. Other agents unchanged | Existing detect tests still pass; visual check of a Codex / Warp Agent pane |
| 4. Lookalikes do not match | Unit negatives (`grokking`, `mygrok`) |
| 5. Settings lists Grok Build | `all::<CLIAgent>()` includes Grok; `display_name()` assertion |
| 6. `grok` is shell, not NL | Keyword added to `ONE_OFF_SHELL_COMMAND_KEYWORDS` |
| 7. OSC 777 optional | Listener supported; plugin manager is `None` |
| 8. Session end restores terminal icon | Existing session teardown; manual check |

Manual: `./script/run` (OSS `WarpOss.app` when `warp-channel-config` is
absent), run `grok` in a pane, confirm the sidebar mark and toolbar,
confirm a neighboring Codex / Warp Agent pane is unchanged.

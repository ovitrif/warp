# Product Spec: Identify Grok Build (`grok`) as a first-class CLI agent

**Issue:** [warpdotdev/warp#11727](https://github.com/warpdotdev/warp/issues/11727)
**Figma:** none provided

## Summary

Warp should recognize a running Grok Build session (`grok`) the same way it
already recognizes Codex, Claude Code, Gemini CLI, Warp Agent, and the other
built-in CLI agents. A pane whose active command is `grok` must show Grok's
official black-hole logomark in the sidebar / vertical tabs, surface Grok
Build as the agent identity in the CLI-agent toolbar, and participate in the
same baseline CLI-agent surfaces (rich input, settings list, telemetry)
without requiring a custom toolbar regex.

## Problem

Users who mix Codex, Claude Code, Warp Agent, and Grok Build cannot tell
which sidebar entries are Grok sessions. Codex and Warp Agent get branded
circular marks; a `grok` pane stays a generic terminal glyph. Warp already
ships `Icon::GrokLogo` (the official Grok black-hole logomark) for the xAI
model picker, but `CLIAgent` has no `Grok` variant and `grok` is not a
detected command prefix.

A custom `cli_agent_toolbar_enabled_commands` regex can force the toolbar on,
but the serialized value deserializes to `CLIAgent::Unknown`, whose `icon()`
is `None`. That is a partial workaround, not first-class identity.

## Goals

- Auto-detect the `grok` command (basename, path, env-prefix, aliases) as
  Grok Build.
- Render the official Grok black-hole logomark on the same circular
  brand-color tile used by other CLI agents, in vertical tabs / sidebar and
  other agent-icon surfaces.
- Show the CLI-agent toolbar / rich-input footer for a detected Grok
  session, with display name **Grok Build**.
- List Grok Build in Settings → CLI Agents alongside the other known
  agents.
- Treat bare `grok` as a shell command in input classification so Agent Mode
  does not steal the launch.

## Non-goals

- A Warp-owned notification-plugin *installer* for Grok (hooks under
  `$GROK_HOME/hooks`, marketplace package). Listening for OSC 777 events that
  Grok already emits is in scope; shipping a new install/update flow is not.
- Cloud `Harness::Grok` / mapping Grok into Oz cloud dispatch.
- Treating the Cursor CLI basename `agent` as Grok.
- Adding Grok to onboarding marketing copy or the public docs site (docs
  live outside this monorepo).
- A dedicated `SkillProvider::Grok` / `.grok/skills` scanner. Grok Build
  already reads `.agents/skills`; that existing provider is enough for the
  rich-input slash menu.
- Replacing or restyling the existing xAI model-picker `Icon::GrokLogo`
  asset. This change reuses that official logomark.

## User-visible behavior

1. When the active command's executable basename is `grok` (including
   `/path/to/grok`, `FOO=1 grok`, or an alias that expands to `grok`), Warp
   records a CLI-agent session with identity **Grok Build**.
2. Vertical tabs / sidebar for that pane render the official Grok black-hole
   logomark on a black circular tile, matching the Codex / Claude treatment
   (brand circle + tinted monochrome mark). The pane is no longer a generic
   terminal glyph.
3. Other agent-icon surfaces that already consume `CLIAgent::icon()` /
   `brand_color()` (pane header, conversation list when a CLI session is
   selected, settings agent dropdown) use the same mark and black brand
   color.
4. The CLI-agent toolbar / footer appears for a detected Grok session, the
   same as for Codex or Claude Code. Rich-input placeholder copy uses
   "Grok Build".
5. `grok` with arguments (`grok --resume …`, `grok --yolo`) still detects as
   Grok Build. Lookalikes (`grokking`, `mygrok`) do not.
6. Settings → CLI Agents lists **Grok Build** with the same logomark. Users
   can bind a custom command pattern to Grok Build (not only to Unknown).
7. Typing `grok` in the Warp input is classified as a shell command, not as
   natural-language Agent Mode, matching `claude` / `codex` / `gemini`.
8. If a Grok session already emits Warp's structured OSC 777
   (`warp://cli-agent`) notifications — for example via a user-installed
   plugin under `~/.grok/` — Warp listens and can show rich session status.
   Lack of those events does not block identification or the toolbar.
9. Ending the `grok` process returns the pane to a normal terminal
   indicator. Other agents are unchanged.

## Success criteria

1. Running `grok` in a pane shows the Grok black-hole mark in the sidebar
   without any custom regex.
2. The same pane shows the CLI-agent toolbar labeled for Grok Build.
3. `codex`, `claude`, and Warp Agent panes keep their existing marks.
4. `grokking` / `mygrok` / `cargo run` do not detect as Grok Build.
5. Settings lists Grok Build as a selectable CLI agent.

## Open questions

None that block this spec. Existing PRs (#14228 first-class plugin install,
#14780 recognition without a licensed icon) overlap; this spec is the
identification + official-logomark slice that unblocks the user-visible
sidebar problem. Plugin auto-install can land as a follow-up once this
identity exists.

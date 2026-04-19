# Cross-Reference Gap Analysis — All Open Documents

> **Date:** 2026-04-06
> **Purpose:** Identify valuable knowledge present in the brochure, snapshot,
> or handbook that is **missing** from `architecture_findings.md`.

## Methodology

Each item below was found in one of the other three documents but has
**no corresponding entry** (or only a shallow mention) in
`architecture_findings.md`. Items already well-covered are omitted.

---

## Missing from `architecture_findings.md`

### From `codex_solo_developer_brochure.md` (Section 6 Golden Nuggets)

| # | Topic | Source | Gap |
|---|-------|--------|-----|
| 1 | Mid-Stream Permission Escalation | §6.B | `request_permissions.rs` — agents dynamically request escalated privileges mid-turn. Not in findings. |
| 2 | `codex.emitImage()` Bridging API | §6.C | js_repl Deno env exposes `codex.emitImage({ bytes })` to pipe screenshots into model visual cortex without disk I/O. Not in findings. |
| 3 | Multiplexer Awareness (Zellij/Tmux) | §6.D | Codex detects multiplexers and drops ratatui alternate screen to preserve scrollback. Not in findings. |
| 4 | PasteBurst State Machine | §6.E | Catches high-speed keystrokes during large pastes to prevent mid-paste submission. Not in findings. |
| 5 | Instant Draft Recovery | §6.F | Prompts with image attachments cached locally; Up after Ctrl+C fully rehydrates them. Not in findings. |
| 6 | Dynamic Tool Registration | §6.I | `DynamicToolHandler` ingests new tools on the fly via `DynamicToolCallRequest`. Not in findings. |
| 7 | Real-Time Audio and VU Meters | §6.J | `voice.rs` captures CPAL mic streams, renders live ASCII VU meter. Not in findings. |
| 8 | Backtrack Engine | §6.K | `app_backtrack.rs` provides conversation rewind trimming HistoryCells. Not in findings. |
| 9 | ARC Safety Monitor | §6.L | `arc_monitor.rs` halts tool execution mid-turn, pings safety endpoint. Not in findings. |
| 10 | Turn Diff Tracker | §6.N | `turn_diff_tracker.rs` takes baseline memory snapshots, tracks renames via `similar` crate. Not in findings. |
| 11 | Autonomous Memory Pipeline | §6.O | `memories/mod.rs` runs dual-phase consolidation into `memory_summary.md`. Not in findings. |
| 12 | Shimmer and Diff Rendering | §6.S | `shimmer.rs` time-synced visual sweeps; `diff_render.rs` 90k+ byte IDE-grade diffs. Not in findings. |
| 13 | OpenTelemetry with mTLS | §6.T | `otlp.rs` gRPC/HTTP OTel exporter with mTLS certificate support. Not in findings. |
| 14 | Codex as its own MCP Server | §6.U | mcp-server exposes `codex` and `codex-reply` tools for recursive subagent spawning. Not in findings. |
| 15 | MITM Proxy Internals | §6.W | `mitm.rs` generates leaf TLS certs, evaluates payloads against `NetworkPolicyDecider`. Not in findings. |
| 16 | Code-Mode Exec Spawning | §6.CC | `code-mode` defines `exec`/`wait` with `DEFAULT_EXEC_YIELD_TIME_MS`. Not in findings. |
| 17 | Universal Tool Registry | §6.EE | `tools/src/lib.rs` consolidates all tool types into `ResponsesApiTool`. Not in findings. |
| 18 | Ollama Auto-Provisioning | §6.FF | `ollama/src/lib.rs` auto-pulls missing weights with progress streaming. Not in findings. |

### From `codex_snapshot_april_2026.md`

| # | Topic | Gap |
|---|-------|-----|
| 19 | codex-analytics crate | Externalized fact/event collection spin-out. Not in findings. |
| 20 | codex-login crate | Decoupled PKCE and Auth management. Not in findings. |
| 21 | codex-shell-command crate | 2.3k LOC isolated command safety parser. Not in findings. |
| 22 | Q2 2026 Action Plan | Concrete refactoring priorities not captured as strategic context. |

### From `codex_architectural_handbook.md`

| # | Topic | Gap |
|---|-------|-----|
| 23 | The "Turn" Lifecycle | 7-step flow not documented as a sequence in findings. |
| 24 | PathBuf Anti-Pattern | "Never manually concatenate paths" — critical gotcha not in findings. |
| 25 | Best-Fit Project Types | Strategic positioning for Codex use not in findings. |
| 26 | Tooling Foundation | Mise, Bazel, UV, Cargo-Insta build toolchain not documented. |

---

## Summary

**Total gaps found: 26 items** containing valuable, actionable knowledge
for Jamie that are not present in the central `architecture_findings.md`.

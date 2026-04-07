# Refactoring Roadmap

This document outlines the known technical debts and the strategic targets for architectural refactoring.

## Recognized Technical Debts to Avoid
The "Thin Core" initiative was spawned precisely because of four specific legacy hotspots which **should not** be grown further:
1. `codex-rs/tui/src/chatwidget.rs` (11,071 LOC <!-- check-loc: codex-rs/tui/src/chatwidget.rs -->, artifact: `raw/loc/tokei.json`)
2. `codex-rs/app-server/src/codex_message_processor.rs` (9,669 LOC <!-- check-loc: codex-rs/app-server/src/codex_message_processor.rs -->, artifact: `raw/loc/tokei.json`)
3. `codex-rs/tui/src/history_cell.rs` (4,120 LOC)
4. `codex-rs/protocol/src/protocol.rs` (3,781 LOC)
To maintain velocity, new capabilities mapped via the app-server should be scaffolded entirely in new <500-line modular crates.

## Q2 2026 Strategic Refactoring Targets
Four concrete hotspot decompositions are planned for Q2 2026. Knowing these prevents you from adding to the wrong files.
- **Targets:**
  1. **`chatwidget.rs` decomposition** — the 11,071 LOC <!-- check-loc: codex-rs/tui/src/chatwidget.rs --> monolith should be split into focused sub-modules (composer, history cell, diff view, event dispatcher).
  2. **v1 → v2 API migration** — all remaining protocol v1 surface area is being sunsetted; new API work goes to v2 exclusively.
  3. **Core thinning** — resist adding to `codex-core`; spin out new crates instead.
  4. **Stylize adoption** — replace all manual `Style::default()` calls with Ratatui's `Stylize` trait helpers (`"text".red().bold()`) for consistency and readability.
- **Gotcha:** If you open a PR that touches `chatwidget.rs` and your change adds more than ~50 LOC, expect pushback during review asking you to extract a sub-module instead.

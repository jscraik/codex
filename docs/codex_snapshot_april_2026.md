# Codex Repository Snapshot — April 2026

## Executive Summary
As of April 2026, the Codex repository is in a state of **active architectural transition**. The codebase is moving away from a monolithic Rust structure (`codex-core`) toward a highly distributed, service-oriented architecture with a primary focus on the **v2 App-Server API**.

However, significant "monolith hotspots" remain, particularly in the TUI (10k+ LOC widgets) and legacy sandboxing modules. Technical debt is being actively managed via strict LOC limits (500 LOC/module) for new code, but legacy debt in the `tui` and `app-server` crates is extreme.

---

## Technical Pulse
| Metric | Value | Status |
| :--- | :--- | :--- |
| **Active RPC Version** | v2 (CamelCase) | ✅ Primary |
| **Legacy RPC Version** | v1 (SnakeCase) | ⚠️ Maintenance |
| **Lines of Code (TUI ChatWidget)** | 10,221 | 🛑 CRITICAL DEBT |
| **Lines of Code (App Server Message Processor)** | 8,998 | 🛑 CRITICAL DEBT |
| **Module LOC Target** | < 500 lines | 🟢 Enforced for new code |
| **Tooling Foundation** | Mise, Bazel, UV (Python) | ✅ Modern |

---

## Core Architecture Trends

### 1. The v2 App-Server Pivot
The repository is consolidating around the `app-server-protocol/v2` namespace.
- **CamelCase on the wire**: All v2 interactions are standardized to CamelCase.
- **Strict Optionality**: Request payloads (`*Params`) now use `#[ts(optional = nullable)]` and `Option<T>` for everything except mandatory fields.
- **Cursor Pagination**: Standardized across all `list` methods.

### 2. Service Decentralization
The project is successfully spinning out capabilities from `codex-core`:
- **`codex-analytics`**: Externalized fact/event collection.
- **`codex-login`**: Decoupled PKCE and Auth management.
- **`codex-shell-command`**: Isolated parsing and safety logic (2.3k LOC parser).
- **`codex-windows-sandbox-rs`**: A platform-specific powerhouse for secure execution on Windows.
- **`git-utils` & `sleep-inhibitor`**: Extracted process-stability blocks managing Ghost Commits tracking and system sleep prevention.
- **`codex-client` & `rmcp-client`**: Migrated Custom-TLS HTTP engine and Remote MCP authentication structures decoupled from the main protocol loop.

### 3. TUI Complexity
The TUI remains the primary interface for Codex, but its internal architecture is struggling under the weight of the `chatwidget.rs` module.
- **Total lines in `chatwidget.rs`**: 10,221.
- **Total lines in `history_cell.rs`**: 4,120.
- **Recommendation**: Immediate vertical slicing of these modules into smaller, event-driven components.

---

## 4. Priority Issues (Action Table)
| Rank | Issue | Impact | Root Cause | File(s) | Fix Strategy | Confidence |
|------|-------|--------|------------|---------|--------------|------------|
| 1 | TUI Monolith Deadlock | CRITICAL (Time/Scale) | 10k LOC accretion | `tui/src/chatwidget.rs` | Vertical extraction of bottom pane. | High |
| 2 | Orchestrator God-Object | HIGH (Bottleneck) | 9k LOC match statement | `app-server/src/codex_message_processor.rs` | Route-handler trait pattern | High |
| 3 | V1/V2 Protocol Overlap | MED (Friction/Bugs) | Legacy API bridging | `protocol/src/protocol.rs` | Hard-deprecate v1, enforce CamelCase. | High |

---

## 5. Fix Implementation Plans (Codex-Ready)

### Issue: TUI Monolith Deadlock Risk
- **Problem:** `chatwidget.rs` is over 10,200 lines, making edits extremely prone to breaking Ratatui state borrows and violating 500-LOC module limits.
- **Root Cause:** Features, styling, and key-handlers for the chat interface were dumped directly into the main widget implementation rather than nested sub-controllers.
- **Why it matters:** AI agents and human developers suffer massive context-window exhaustion and merge-conflict risks.

#### Implementation Plan
1. Create `codex-rs/tui/src/bottom_pane/chat_composer.rs` and `footer.rs`.
2. Extract keyboard event handlers and layout calculations out of `chatwidget.rs`.
3. Consolidate manual styling to Ratatui's `.stylize()` syntax.

#### Validation
- `cargo test -p codex-tui` and `cargo insta accept -p codex-tui` must pass, ensuring zero pixel regressions.

### Issue: Orchestrator God-Object
- **Problem:** `codex_message_processor.rs` handles over 360 functions parsing incoming Turn payloads.
- **Root Cause:** Unbounded `match` block interpreting JSON-RPC methods instead of dynamically registered dispatchers.
- **Why it matters:** Hard-couples discrete capabilities (e.g., analytics) directly into the main loop.

#### Implementation Plan
1. Define a `TurnDispatcherExt` trait.
2. Explode the matching function into isolated generic router modules (`handlers/turn_cmd.rs`).

#### Validation
- `just write-app-server-schema` generates without error and test suite `core_test_support::responses` validates identical HTTP payloads.

---

## 6. Developer Friction Report
- **AI Context Obfuscation**: 10k+ LOC files completely destroy LLM reasoning. **Fix**: Strictly enforce the existing 500-LOC rule in `AGENTS.md` and break up `chatwidget.rs`.
- **"Magic" Behavior**: Invisible process state. **Fix**: Explicitly document `shell-escalation` Unix interceptors and `arg0` identity switching so logic is accessible.
- **Mixed Styling Norms**: Cluttered API calls. **Fix**: Eliminate manual `Style::default()` via automated `.into()` / `.stylize()` refactoring across the TUI.

---

## 7. Architecture Risks & Missing Capabilities
- **Scalability**: The `app-server` event loop blocks heavily on single threads during monolithic payload processing.
- **Security**: Editing `shell-command/parse_command.rs` without extreme precision risks authorizing `-rf` or path-escape destructiveness natively.
- **Missing Capabilities**: 
  - Trace logging via OTel correlation IDs across the internal `shell-escalation` intercepts.
  - Native JSON validation layers *before* deep Rust struct deserialization (to reject malformed traffic early).
  - Explicit UI silent-panic handling to catch `history_cell` template crashes rather than dropping frames.

---

## 8. Codex Optimization Layer (AI Effectiveness)
**How to improve this repo for AI-assisted development:**
- **Missing Repo Map**: Generate `.codex/map.txt` detailing decoupled boundaries instantly.
- **Targeted TUI Prompting**: Inject strict UI rules directly into `AGENTS.md` (e.g. forbidding manual `Style`).
- **File Discoverability**: Maintain high-level routing markers bridging legacy `v1` vs targeted `v2` payload structures to avoid agent confusion.

---

## 9. Strict Execution Order
1. **Fix TUI Monolith (`chatwidget.rs` & `history_cell.rs`)** - *Reason: Unblocks velocity massively by solving context-exhaustion.*
2. **Decompose `codex_message_processor.rs`** - *Reason: Decouples the primary orchestrator before spinning out more core logic.*
3. **Add V2 Schema CI Verification** - *Reason: Establishes strong API boundaries before extracting further subsystems.*

---

## Run Data Index
- **Authorization**: [.recon/authorization.json](file:///Users/jamiecraik/dev/codex/.recon/authorization.json)
- **Latest Plan**: [.recon/probe-plan.json](file:///Users/jamiecraik/dev/codex/.recon/probe-plan.json)
- **Latest Report**: [.recon/runs/openai-codex-v2/run-01/derived/report.md](file:///Users/jamiecraik/dev/codex/.recon/runs/openai-codex-v2/run-01/derived/report.md)
- **Architecture Map**: [.recon/runs/openai-codex-v2/run-01/raw/oss.architecture_map/architecture_map.mmd](file:///Users/jamiecraik/dev/codex/.recon/runs/openai-codex-v2/run-01/raw/oss.architecture_map/architecture_map.mmd)

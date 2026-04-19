# API & Code Quality Audit: `codex-rs`

This audit evaluates the current internal state of `codex-rs` against the **Architectural Standards** defined in the [Codex Developer Brochure](file:///Users/jamiecraik/dev/codex/docs/codex_solo_developer_brochure.md) and [AGENTS.md](file:///Users/jamiecraik/dev/codex/AGENTS.md).

---

## 📊 Summary Scorecard

| Category | Status | Confidence | Compliance |
| :--- | :--- | :--- | :--- |
| **A. Idiomatic Rust** | ⚠️ Warning | 70% | **Low (48k wildcards found)** |
| **B. Bulletproof API** | ❌ Failing | 90% | **Very Low (Positional Booleans)** |
| **C. Crate Discipline** | 💀 Critical | 100% | **Extremely Low (11k+ LOC Files)** |
| **D. TUI Styling** | ⚠️ Warning | 80% | **Medium (98x Style::default)** |
| **E. E2E Testing** | ✅ Passing | 95% | **High (Insta + SSE Mocks)** |
| **F. Protocol Wire Contracts** | ❌ Failing | 90% | **Low (423x skip_serializing_if)** |

---

## 🔍 Detailed Findings

### Category B: Bulletproof API Design
**Standard:** Avoid ambiguous positional arguments like `(true, None)`. Require inline `/*param_name*/` comments.
- **Violation:** `backend-client/src/types.rs` uses `(true, true) => None` pattern in match arms, making context invisible to external readers.
- **Violation:** `core-skills/src/loader.rs` passes `false` into `scan()` without parameter commentary.
- **Impact:** Increases cognitive load for reviewers and contributors; leads to "mystery booleans" in complex orchestration logic.

### Category C: Crate Discipline (The Blotch)
**Standard:** Files exceeding 800 LOC require architectural extraction.
- **Critical Failure:** The codebase has evolved massive "god modules" that stifle review and parallel development.
- **Top Blocker:** `tui/src/chatwidget.rs` is **11,071 LOC**.
- **Top Blocker:** `tui/src/app.rs` is **10,929 LOC**.
- **Impact:** These files are essentially un-reviewable by agents or humans in single turns. They represent the primary source of merge conflicts and technical drag.

### Category D: TUI (Ratatui) Styling guidelines
**Standard:** Use ratatui's `Stylize` trait (e.g., `"text".red()`). Avoid `Style::default()`.
- **Violation:** Found **98 instances** of `Style::default()` used to construct manual style states.
- **Concentration:** `tui/src/diff_render.rs` is a high-density violator.
- **Impact:** Breaks the "design system" approach. Styling becomes ad-hoc rather than utility-driven, making theme-wide changes (like high-contrast Mode) difficult to implement.

### Category F: Protocol Wire Contracts
**Standard:** Never use `#[serde(skip_serializing_if = "Option::is_none")]` for v2 payloads. Favor explicit nulls.
- **Systemic Violation:** Found **423 instances** of `skip_serializing_if` on `Option` fields.
- **Impact:** This causes "Ghost Properties" on the frontend. A property might exist in one response but be completely missing in the next, forcing the TypeScript consumer to constantly check for key existence rather than just handling a `null` value. It undermines the strict wire contract.

---

## 🛠️ Recommended Remediation (Short-Term)

1. **Hydrate Parameter Comments:** Run a sweep of `codex-rs` using `just fix` (if it includes the parameter comment lint) or manually add `/*param_name*/` to the positional literals identified in `core-skills` and `backend-client`.
2. **The Great Extraction:** Prioritize splitting `chatwidget.rs` and `app.rs`. These should be broken into component modules (e.g., `chat_history.rs`, `action_dispatcher.rs`) immediately.
3. **Serialization Lockdown:** Remove `skip_serializing_if` from `app-server-protocol/src/protocol/v2.rs` to stabilize the TypeScript consumer contracts.
4. **Style Migration:** Convert the 44 instances of manual styling in `tui/src/diff_render.rs` to the `Stylize` trait helpers.

# Architectural Standards Compliance Profile (Snapshot Audit)

An internal inspection was run against the **6 Golden Code Practices** defined in the solo developer brochure to measure real-world standard debt across `codex-rs/`.

## A. Idiomatic Rust Standards
**Standard:** Inline format arguments, concise iterators, and exhaustive matching without wildcards.
- **Status:** **Partially Failing**
- **Findings:** Over 860 <!-- check-metric: wildcard_match_arms_naive | codex-rs (repo-wide) --> wildcard matches (`_ => {}`) were located, particularly in `parse_command.rs` and `cloud-tasks`. Several non-inlined format strings also exist across core crates (e.g., `format!("{}", body.len())`, `format!("{}", i32::MAX)`). Redundant closures however, appear broadly eliminated.

## B. Bulletproof API Design
**Standard:** Avoid ambiguous positional arguments like `(true, None)`. Require inline `/*param_name*/` comments.
- **Status:** **Failing**
- **Findings:** Found 8 direct violations where boolean and `None` literals are passed ambiguously in tuples or heavily nested parameters (`(true, None)`). Heavy concentration of this exists inside `codex_message_processor.rs` and `config_tests.rs`. 

## C. Scaling the Monolith (Crate Discipline)
**Standard:** Module caps at 500 LOC (excluding tests). Files exceeding 800 LOC require architectural extraction.
- **Status:** **Critical Fail (Blocker)**
- **Findings:** The hard limits are profoundly breached leading to systemic maintainability risk.
Top offenders include:
1. `tui/src/chatwidget.rs` (11,071 LOC <!-- check-loc: codex-rs/tui/src/chatwidget.rs -->)
2. `tui/src/app.rs` (10,929 LOC <!-- check-loc: codex-rs/tui/src/app.rs -->)
3. `app-server/src/codex_message_processor.rs` (9,669 LOC <!-- check-loc: codex-rs/app-server/src/codex_message_processor.rs -->)

## D. TUI (Ratatui) Styling Guidelines
**Standard:** Use ratatui's `Stylize` trait (e.g., `"text".red().bold()`). Never construct manual `Style::default()`. 
- **Status:** **Failing**
- **Findings:** Strict hardcoded variables like `.white()` are properly avoided. However, over **98 <!-- check-metric: style_default_usage | codex-rs (repo-wide) --> instances** heavily construct manual structures using `Style::default()`. `tui/` alone violates this 87 <!-- check-metric: style_default_usage | codex-rs/tui --> times.

## E. E2E Testing with SSE Mocks
**Standard:** Visual changes must have `insta` snapshot coverage. Server protocols must leverage `ResponseMock` and `ev_completed()`.
- **Status:** **Passing**
- **Findings:** Healthy integration verified. The UI crate actively leverages `insta::assert_*` (32 tests in `tui/`). The application server heavily utilizes the requested `mount_sse` patterns and `ResponseMock` models (7 comprehensive test locations).

## F. TypeScript/Wire Protocol Contracts
**Standard:** Strict `camelCase`. `Option<T>` must be manually `nullable`. `skip_serializing_if = "Option::is_none"` is restricted.
- **Status:** **Partially Failing**
- **Findings:** `camelCase` is enforced securely (426 declarations against 16 configuration-only `snake_case` exceptions). `#[ts(optional = nullable)]` is correctly attached to over 115 structures. However, `skip_serializing_if = "Option::is_none"` violates standard bounds by appearing **71 <!-- check-metric: skip_serializing_if_option | codex-rs/app-server-protocol/src/protocol/v2.rs --> times** specifically inside `protocol/v2.rs`, actively dropping properties from the JSON tree instead of guaranteeing explicit `{ "item": null }` outputs.

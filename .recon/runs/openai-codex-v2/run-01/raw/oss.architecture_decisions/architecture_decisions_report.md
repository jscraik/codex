# Architecture Decisions Analysis

**Status:** informal_evidence
**Generated:** 2026-04-06T17:49:04.747592+00:00
**Coverage Score:** 100%

## Required Sections (from ADR mappings)

- **Problem & Job (Context/Problem):** yes
  - `docs/tui-alternate-screen.md:7` The Problem
  - `docs/tui-stream-chunking-review.md:6` Problem
  - `docs/tui-chat-composer.md:14` What problem is being solved?
  - `codex-rs/README.md:29` Model Context Protocol Support
  - `codex-rs/otel/README.md:125` Trace context
- **Success Criteria (Decision/Goals):** yes
  - `docs/tui-stream-chunking-tuning.md:25` Tuning goals
  - `docs/tui-stream-chunking-review.md:12` Design goals
  - `docs/tui-stream-chunking-review.md:20` Non-goals
  - `codex-rs/utils/pty/README.md:52` });
  - `codex-rs/utils/stream-parser/README.md:66` }
- **Scope (Consequences/Constraints):** yes
  - `docs/tui-stream-chunking-tuning.md:6` Scope
  - `docs/tui-stream-chunking-validation.md:6` Scope
  - `codex-rs/network-proxy/README.md:50` Use exact hosts or scoped wildcards like `*.openai.com` or `**.openai.com`.
  - `codex-rs/utils/pty/README.md:52` });
  - `codex-rs/utils/stream-parser/README.md:66` }
- **Primary User Journey (Status/Implementation):** yes
  - `docs/tui-alternate-screen.md:67` Implementation Details
  - `codex-cli/README.md:663` Classic, JS implementation that includes small, native binaries for Linux sandboxing.
  - `codex-cli/README.md:691` Use either one of the commands according to which implementation you want to work with
  - `codex-cli/README.md:701` Use either one of the commands according to which implementation you want to work with
  - `codex-cli/README.md:710` Use either one of the commands according to which implementation you want to work with

## Architecture Decision Records

### Install using npm
**File:** `README.md`
**Status:** unknown
**Sections:** none

### TUI Stream Chunking Tuning Guide
**File:** `docs/tui-stream-chunking-tuning.md`
**Status:** unknown
**Sections:** scope, success_criteria

### Exit and shutdown flow (tui)
**File:** `docs/exit-confirmation-prompt-design.md`
**Status:** unknown
**Sections:** none

### Authentication
**File:** `docs/authentication.md`
**Status:** unknown
**Sections:** none

### license
**File:** `docs/license.md`
**Status:** unknown
**Sections:** none

### AGENTS.md
**File:** `docs/agents_md.md`
**Status:** unknown
**Sections:** none

### sandbox
**File:** `docs/sandbox.md`
**Status:** unknown
**Sections:** none

### open-source-fund
**File:** `docs/open-source-fund.md`
**Status:** unknown
**Sections:** none

### Clone the repository and navigate to the root of the Cargo workspace.
**File:** `docs/install.md`
**Status:** unknown
**Sections:** none

### TUI Alternate Screen and Terminal Multiplexers
**File:** `docs/tui-alternate-screen.md`
**Status:** unknown
**Sections:** problem_and_job, primary_user_journey

### Sample configuration
**File:** `docs/example-config.md`
**Status:** unknown
**Sections:** none

### TUI Stream Chunking Validation Process
**File:** `docs/tui-stream-chunking-validation.md`
**Status:** unknown
**Sections:** scope

### Execution policy
**File:** `docs/execpolicy.md`
**Status:** unknown
**Sections:** none

### Getting started with Codex CLI
**File:** `docs/getting-started.md`
**Status:** unknown
**Sections:** none

### Configuration
**File:** `docs/config.md`
**Status:** unknown
**Sections:** none

### Request user input overlay (TUI)
**File:** `docs/tui-request-user-input.md`
**Status:** unknown
**Sections:** none

### Skills
**File:** `docs/skills.md`
**Status:** unknown
**Sections:** none

### Individual Contributor License Agreement (v1.0, OpenAI)
**File:** `docs/CLA.md`
**Status:** unknown
**Sections:** none

### contributing
**File:** `docs/contributing.md`
**Status:** unknown
**Sections:** none

### TUI Stream Chunking
**File:** `docs/tui-stream-chunking-review.md`
**Status:** unknown
**Sections:** problem_and_job, success_criteria

### Non-interactive mode
**File:** `docs/exec.md`
**Status:** unknown
**Sections:** none

### JavaScript REPL (`js_repl`)
**File:** `docs/js_repl.md`
**Status:** unknown
**Sections:** none

### Slash commands
**File:** `docs/slash_commands.md`
**Status:** unknown
**Sections:** none

### Chat Composer state machine (TUI)
**File:** `docs/tui-chat-composer.md`
**Status:** unknown
**Sections:** problem_and_job

### or
**File:** `codex-cli/README.md`
**Status:** unknown
**Sections:** primary_user_journey

### Codex CLI (Rust Implementation)
**File:** `codex-rs/README.md`
**Status:** unknown
**Sections:** primary_user_journey, problem_and_job

### Containerized Development
**File:** `.devcontainer/README.md`
**Status:** unknown
**Sections:** none

### Workflow Strategy
**File:** `.github/workflows/README.md`
**Status:** unknown
**Sections:** none

### Codex App Server Python SDK (Experimental)
**File:** `sdk/python/README.md`
**Status:** unknown
**Sections:** none

### Codex SDK
**File:** `sdk/typescript/README.md`
**Status:** unknown
**Sections:** none

### Codex CLI Runtime for Python SDK
**File:** `sdk/python-runtime/README.md`
**Status:** unknown
**Sections:** none

### Python SDK Examples
**File:** `sdk/python/examples/README.md`
**Status:** unknown
**Sections:** none

### oai-codex-ansi-escape
**File:** `codex-rs/ansi-escape/README.md`
**Status:** unknown
**Sections:** none

### codex-network-proxy
**File:** `codex-rs/network-proxy/README.md`
**Status:** unknown
**Sections:** scope

### codex-tools
**File:** `codex-rs/tools/README.md`
**Status:** unknown
**Sections:** none

### codex-core
**File:** `codex-rs/core/README.md`
**Status:** unknown
**Sections:** none

### codex-process-hardening
**File:** `codex-rs/process-hardening/README.md`
**Status:** unknown
**Sections:** none

### codex-linux-sandbox
**File:** `codex-rs/linux-sandbox/README.md`
**Status:** unknown
**Sections:** none

### codex-exec-server
**File:** `codex-rs/exec-server/README.md`
**Status:** unknown
**Sections:** none

### codex-otel
**File:** `codex-rs/otel/README.md`
**Status:** unknown
**Sections:** problem_and_job

### codex-protocol
**File:** `codex-rs/protocol/README.md`
**Status:** unknown
**Sections:** none

### codex-app-server
**File:** `codex-rs/app-server/README.md`
**Status:** unknown
**Sections:** primary_user_journey, problem_and_job

### codex-app-server-client
**File:** `codex-rs/app-server-client/README.md`
**Status:** unknown
**Sections:** none

### App Server Test Client
**File:** `codex-rs/app-server-test-client/README.md`
**Status:** unknown
**Sections:** problem_and_job

### codex-execpolicy
**File:** `codex-rs/execpolicy/README.md`
**Status:** unknown
**Sections:** none

### codex-client
**File:** `codex-rs/codex-client/README.md`
**Status:** unknown
**Sections:** none

### codex-execpolicy-legacy
**File:** `codex-rs/execpolicy-legacy/README.md`
**Status:** unknown
**Sections:** none

### ChatGPT
**File:** `codex-rs/chatgpt/README.md`
**Status:** unknown
**Sections:** none

### codex-api
**File:** `codex-rs/codex-api/README.md`
**Status:** unknown
**Sections:** none

### codex-stdio-to-uds
**File:** `codex-rs/stdio-to-uds/README.md`
**Status:** unknown
**Sections:** none

### codex-shell-escalation
**File:** `codex-rs/shell-escalation/README.md`
**Status:** unknown
**Sections:** none

### codex-responses-api-proxy
**File:** `codex-rs/responses-api-proxy/README.md`
**Status:** unknown
**Sections:** none

### codex_file_search
**File:** `codex-rs/file-search/README.md`
**Status:** unknown
**Sections:** none

### codex-git-utils
**File:** `codex-rs/git-utils/README.md`
**Status:** unknown
**Sections:** none

### codex-debug-client
**File:** `codex-rs/debug-client/README.md`
**Status:** unknown
**Sections:** none

### @openai/codex-responses-api-proxy
**File:** `codex-rs/responses-api-proxy/npm/README.md`
**Status:** unknown
**Sections:** none

### Overview
**File:** `codex-rs/apply-patch/tests/fixtures/scenarios/README.md`
**Status:** unknown
**Sections:** none

### codex-utils-cargo-bin runfiles strategy
**File:** `codex-rs/utils/cargo-bin/README.md`
**Status:** unknown
**Sections:** none

### codex-utils-pty
**File:** `codex-rs/utils/pty/README.md`
**Status:** unknown
**Sections:** problem_and_job, success_criteria, scope, primary_user_journey

### codex-utils-template
**File:** `codex-rs/utils/template/README.md`
**Status:** unknown
**Sections:** none

### codex-utils-stream-parser
**File:** `codex-rs/utils/stream-parser/README.md`
**Status:** unknown
**Sections:** problem_and_job, success_criteria, scope, primary_user_journey

### `codex-core` config loader
**File:** `codex-rs/core/src/config_loader/README.md`
**Status:** unknown
**Sections:** none

### Memories Pipeline (Core)
**File:** `codex-rs/core/src/memories/README.md`
**Status:** unknown
**Sections:** none

### argument-comment-lint
**File:** `tools/argument-comment-lint/README.md`
**Status:** unknown
**Sections:** none

### npm releases
**File:** `codex-cli/scripts/README.md`
**Status:** unknown
**Sections:** none


## Scan Patterns

- README.md
- docs/**/*.md
- **/README.md

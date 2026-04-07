# Codex: The Architectural Handbook (April 2026)

## Executive Summary
Codex is a powerful autonomous coding agent system designed to interact safely and efficiently with a local workstation. As of Q2 2026, the project is undergoing a major **"Monolith Thinning"** initiative, moving logic from a central Rust core into specialized, decoupled services and crates.

The system is defined by its strict security guardrails, a modern terminal-based user interface (TUI), and a robust **v2 App-Server API** that provides a high-fidelity bridge between the agent and the workstation.

---

## 🏗️ System Architecture

### 1. The Gateway: `codex-app-server`
The `app-server` is the orchestrator. It manages the lifecycle of agent "turns," handles message routing, and provides the API surface for TUIs, IDE extensions, and the SDK.
- **Message Processor**: The engine that interprets incoming JSON-RPC requests and dispatches them to the relevant agent or tool handler.
- **Protocol v2**: The current standardized API. It enforces **CamelCase** on the wire and uses explicit TypeScript definitions for all payloads.

### 2. The Interface: `codex-tui`
Built on **Ratatui**, the TUI provides a visual and interactive way to collaborate with Codex.
- **The Monolith Hotspot**: `chatwidget.rs` is currently the largest file in the repo (10k+ LOC), containing the bulk of interaction logic.
- **Rich Rendering**: Uses custom markdown rendering, ANSI escape sequences for terminal emulation, and stylized UI components.

### 3. The Shield: `sandboxing` & `execpolicy`
Codex executes code in a high-security environment to prevent unauthorized system modification.
- **Seatbelt (macOS)**: Uses Apple's `sandbox-exec` to limit filesystem and network access.
- **Windows Sandbox**: A platform-native implementation for Windows safety.
- **`shell-command` Parser**: A 2.3k LOC security layer that audits CLI strings for "unsafe" flags (e.g., `-rf`) and unauthorized paths.

### 4. The Hidden Infrastructure: Execution & Interception
Deep integration below the user interface handles process identity, undo buffers, and sandboxing escapes.
- **`shell-escalation`**: Patches internal shell processes, routing `exec()` requests over a Unix socket (`CODEX_ESCALATE_SOCKET`) so sandboxed commands can selectively "break out" (escalate) when approved.
- **`arg0` Dispatch**: A busybox-style multiplexer. The single Codex binary seamlessly switches identities (`codex-linux-sandbox`, `apply_patch`, etc.) using session-locked symlinks.
- **Ghost Commits (`git-utils`)**: Tracks state silently via hidden commits on the live working tree. Enables instant, branch-free undo of agent modifications.
- **`sleep-inhibitor`**: Cross-platform OS integration (IOKit/systemd) that halts machine sleep automatically during lengthy automated turns.

### 5. Advanced Transport (`codex-client` & `rmcp-client`)
- **`codex-client`**: A bespoke HTTP layer engineered to enforce Custom Certificate Authorities natively, bypassing corporate TLS-inspection firewalls.
- **`rmcp-client`**: An enterprise-grade Remote MCP implementation featuring dynamic tool discovery and authenticated OAuth PKCE login flows.

---

## 📡 Interaction Flows

### The "Turn" Lifecycle
1.  **User Input**: User submits a command via TUI or SDK.
2.  **App Server Reception**: The `app-server` creates a new "Turn" and persists the state.
3.  **Agent Reasoning**: The LLM determines which "tools" to invoke (e.g., `list_dir`, `read_file`, `shell`).
4.  **Security Audit**: Every tool call is routed through the `execpolicy` and `validation` layers.
5.  **Execution**: The tool runs (optionally in a sandbox).
6.  **Observation**: The output is fed back to the LLM to decide the next step.
7.  **Finalize**: The Turn is closed, and the TUI renders the final result.

---

## 🚀 How to Integrate the App-Server
The **App-Server** (`codex-rs`) is the heart of the system. If you are building a new IDE plugin, a web dashboard, or a custom automation tool, you should integrate it using the **TypeScript SDK**.

### The Integration Formula
The SDK provides a high-level, event-driven interface that abstracts away the underlying JSON-RPC/WebSocket complexity.

```typescript
import { Codex } from "@openai/codex-sdk";

// 1. Initialize the Engine
const codex = new Codex();

// 2. Open a Communication Channel
const thread = codex.startThread();

// 3. Execute an Interactive Turn
const { events } = await thread.runStreamed("Refactor this module and split out the unit tests.");

// 4. Handle Real-Time Feedback
for await (const event of events) {
  if (event.type === "item.completed") {
    console.log(`✅ Action completed: ${event.item.type}`);
  }
}
```

---

## ✅ Positive Engineering Patterns (The "Codex Way")

| Category | Pattern | Rationale |
| :--- | :--- | :--- |
| **Architecture** | **Thin Core** | New features should live in their own crates (e.g., `codex-analytics`, `codex-login`) rather than growing `codex-core`. |
| **API** | **Strict v2 Types** | Use `#[ts(export_to = "v2/")]` and CamelCase wire format. This ensures frontend-backend alignment. |
| **TUI** | **Stylize Trait** | Prefer `"text".red().bold()` over manual `Style::default().fg(Color::Red)`. It's concise and readable. |
| **Testing** | **Snapshot Tests** | Use `insta` for TUI and protocol snapshots. This catches visual and structural regressions instantly. |
| **Complexity** | **500 LOC Modules** | Keep new modules under 500 lines. This keeps the codebase maintainable and reduces merge conflicts. |

---

## 🛑 Negative Engineering Patterns (Developer Friction)

1.  **AI Context Obfuscation (Monoliths)**: Adding unrelated logic to `chatwidget.rs` or `codex_message_processor.rs`. A 10k LOC file completely destroys LLM reasoning windows. If a module exceeds **500 lines**, it must be refactored to unblock human/AI velocity.
2.  **"Magic" Unsearchable Behavior**: Hiding process state. Always document `shell-escalation` Unix interceptors and `arg0` identity switching overrides so the execution logic is accessible to tooling.
3.  **Hardcoded TUI Colors**: Avoid manual `Style::default()` bloat. Stick to the `.into()` / `.stylize()` syntax to eliminate visual API clutter and ensure fast agentic UI editing.
4.  **Implicit v2 Serialization**: Never use `#[serde(skip_serializing_if = "Option::is_none")]` in v2 API response payloads. Ambiguous payload states confuse client generators.

---

## 📈 Technical Debt & Missing Capabilities

As of April 2026, the following architectural gaps represent the highest risk to scale and execution:

| Risk Category | Issue | Mitigation / Missing Capability |
| :--- | :--- | :--- |
| **Scalability** | `app-server` event loop blocked by 9k LOC `codex_message_processor.rs` | Implement `TurnDispatcherExt` trait and split dispatching into discrete generic route modules. |
| **Maintainability** | `chatwidget.rs` (10,221 LOC) & `history_cell.rs` (4,120 LOC) | Vertical extraction into bottom-pane `chat_composer.rs` and `footer.rs`. Enforce strict 500-LOC rule. |
| **Observability** | Invisible Sandbox/Shell intercept execution trace failures. | Add trace logging via OTel correlation IDs across the internal `shell-escalation` intercepts. |
| **Validation** | Panics on malformed TS JSON-RPC payload translation. | Add native JSON validation layers *before* deep Rust struct deserialization. |
| **Stability** | Silent failure swallowing on UI render layout math. | Add explicit UI silent-panic handling to catch `history_cell` template crashes, tracking dropped frames. |

---

## 🤖 Codex Optimization Layer (AI Effectiveness)

To radically improve this repository for AI-assisted development:
1. **Missing Repo Map**: Generate `.codex/map.txt` detailing bounded sub-packages instantly so agents don't have to guess architectural boundaries.
2. **Targeted TUI Prompting**: Inject strict UI rules directly into `AGENTS.md` (e.g. forbidding manual `Style`).
3. **File Discoverability**: Maintain high-level routing markers bridging legacy `v1` vs targeted `v2` payload structures to avoid agent hallucination over which API to target.

---

## 🧪 Best-Fit Projects for Codex
Codex isn't just "another LLM client"; it’s a **local engineering environment**.

- **Legacy Refactoring**: Excels at breaking down 1,000+ line "hotspots" into modular crates.
- **Security-Sensitive Apps**: Built-in **Seatbelt/Sandbox** ensures code execution is hardware-isolated.
- **Monorepo Management**: The native `app-server` has direct, low-latency access to search and index the entire repo.
- **API/Protocol Design**: Uses a strict, versioned protocol (v2) which provides a gold standard for consistency.

---

## 🛠️ Tooling Foundation
Codex relies on a modern, high-performance toolchain:
- **Mise**: For tool version management (Go, Rust, Python, Node).
- **Bazel**: For hermetic, distributed builds and linting.
- **UV**: For lightning-fast Python dependency management in the SDK.
- **Cargo-Insta**: For snapshot-driven development.

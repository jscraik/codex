# Architecture Inventory

This document captures the core architectural components and scaffolding philosophy of the agent framework discovered during the intelligence audit.

## 1. Unified Modular Capability Bundling (`plugin.json`)
Instead of organizing tools into heterogeneous and independent distributions, the architecture centralizes distribution around a universal `plugin.json` format (`PluginManifest`). A single plugin directory can package together `apps` (local UI/CLI logic), `mcpServers` (domain connections), and `skills` (autonomous capabilities). This allows developers to bundle all layers of a feature into one logical, drop-in integration unit (`core/src/plugins/manifest.rs`). 

## 2. Event Interceptors & Behavior Overrides
The core agent loop does not just run sequential prompts; it implements a profound, programmable event hook layer natively intercepting `PreToolUse`, `PostToolUse`, `SessionStart`, and `UserPromptSubmit`. These hooks have absolute authority—they run standalone binaries asynchronously or synchronously to either entirely block execution (`should_block`), or silently inject additional `DeveloperInstructions` context directly into the agent trace, guiding it dynamically. (`core/src/hook_runtime.rs`).

## 3. Remote Cloud Tasks & Off-box Diff Fetching
Agents possess an escape hatch for complex logic via `codex cloud` integrated in `cloud-tasks/src/lib.rs`. Rather than relying on rigid local client capability, the toolchain can deploy workloads over secure ChatGPT backend endpoints. It supports sophisticated multi-turn attempts where the CLI interacts securely via `.auth()` and OS keyrings to retrieve, navigate (`select_attempt`), and preflight (`apply_task_preflight`) external agent patch attempts directly onto the local filesystem.

## 4. Concurrent Data-Partitioned Agent Swarms
The platform avoids sequential task starvation via recursive tool primitives, specifically `spawn_agents_on_csv`. Instead of a main agent doing O(N) linear work, the system exposes an abstract map-reduce tool where the main agent passes a CSV array and an instruction string. Codex natively fans this out, spawning independent worker sandbox agents according to `max_concurrency`, enforcing that all jobs `report_agent_job_result`, generating outputs fully in parallel (`tools/src/agent_job_tool.rs`).

## 5. First-class Multi-OS Secret Enclaves
OAuth integration for generic MCP server connections is heavily guarded by the `rmcp-client/src/oauth.rs` credential store. Instead of lazily keeping credentials on disk in cleartext, the architecture enforces native secret enclaves on every major platform (macOS Keychain, Linux DBus Secret Service/keyutils, Windows Credential Manager). It handles cross-process safety, calculates token refresh times via system hooks, and only gracefully falls back to a limited scope `.credentials.json` if native secure enclaves are definitively unavailable.

## 6. Pre-Warmer Async SQLite Rollout Ledger
Codex prevents heavy JSON-parsing overhead and massive filesystem reads for large sessions through an advanced shadow database mechanism (`rollout/src/state_db.rs`). Background Tokio pre-warmers (`schedule_startup_prewarm`) stream thread rollouts silently into a highly-indexed SQLite state DB as the conversation evolves. This allows fast queries for metadata, dynamic capabilities, and thread cursors natively via SQLite queries while relying on filesystem transparent read-repair if a drift occurs, decoupling memory complexity from raw file loading.

## 7. Distributed Remote Exec Engine & Transport
Rather than just executing bash strictly on the local machine within the same binary, Codex provides a full native `ExecServer` inside `exec-server/src`. By executing command endpoints and local filesystem manipulation payloads as streamable RPC events (`ExecOutputDeltaNotification`) over TCP or Unix Domain Sockets, orchestrators safely mount workloads inside or outside headless containers while streaming standard streams back dynamically.

## 8. Advanced Interceptor Pre-Hooks (Native Binaries)
Deeper than simple text callbacks, the hooks feature (`hooks/src/`) defines a strict schema compilation where hooks are loaded as standalone, dynamically linked binaries logic hooks operating natively. This means any arbitrary behavior can be attached (including OS IPC notifications like `user_notification.rs`) and can structurally mutate payloads, trace telemetry, or hard-halt executing bounds securely during execution (`PendingInputHookDisposition::Blocked`).

## 9. OS-Native Hardened Container Sandboxing
Codex enforces prompt and logic boundaries at the raw Operating System layer via `linux-sandbox/src` and `windows-sandbox-rs/src`. Rather than relying strictly on process policies or prompt guardrails, Linux executions are automatically enclosed inside a Bubblewrap (`bwrap`) container configured with Landlock LSM and read-only mounts. On Windows, executions run isolated via DPAPI, custom ACL manipulation, and explicitly restricted process tokens, representing a hyper-secure environment for potentially untrusted LLM commands.

## 10. Native Network Disconnection & Proxy Redirection
To ensure security of external data connections, agents can be put into a `ProxyOnly` network isolation mode natively configured alongside Bubblewrap. Inside `linux-sandbox/src/bwrap.rs`, the setup strips the raw OS network namespace (`--unshare-net`) allowing the host to seamlessly establish an unescapable traffic bridge specifically routed into Codex's MITM proxy, rather than hoping the LLM plays nicely. Advanced logic completely abstracts networking topology from the invoked shell execution.

## 11. Pre-main() Process Anti-Tampering (`process-hardening` crate)
To prevent malicious agents from using `gdb`, `ptrace`, or heap inspections to break out of jail and read raw memory of the main Codex process, Codex employs a pure C-style `#[ctor::ctor]` hook in `process-hardening/src/lib.rs`. It hooks directly into the ELF/MachO startup sequence to globally disable Core Dumps (`RLIMIT_CORE = 0`) and unconditionally invoke `prctl(PR_SET_DUMPABLE, 0)` (Linux) or `ptrace(PT_DENY_ATTACH, 0)` (macOS) to self-shield against debugger attachment, preventing any reverse-engineering from its own subprocesses.

## 12. Local Secure Responses API Sidecar Proxy
Rather than injecting sensitive user API keys directly into environment variables where terminal tools or LLM code snippets could echo or scrape them, Codex spins up a local transparent proxy sidecar (`codex-rs/responses-api-proxy`). This loopback server only accepts `POST /v1/responses`, internally reading the valid auth token securely injected from `stdin` at initialization. This allows fully isolated LLMs/tools to recursively query models securely.

## 13. V8 Embedded Proof-of-Concept
Found in `codex-rs/v8-poc`, this implies long-term groundwork to move JavaScript sandbox executions completely out of the slower `node.js` dependency ring into an embedded Google V8 engine isolate invoked directly by Rust for blazing fast and securely instrumented JavaScript/WASM execution.

## 14. Sandbox "sudo-like" Shell Escalation (`shell-escalation` crate)
To allow agents restricted inside strict `Seatbelt` or `Bubblewrap` sandboxes to execute explicitly permitted safe side-effect processes (like formatting tools, git commits, or compilation steps) outside the sandbox, Codex implements an elegant Unix domain-socket based Privilege Escalation daemon. When an agent runs a command via the `execve_wrapper`, it intercepts the `exec` request, sends the binary path and arguments to the `EscalateServer` running *outside* the sandbox environment. The server uses `EscalationPolicy` to determine if the command is safe; if it is, the server forks the process natively and uses Unix Socket `SCM_RIGHTS` fd passing (`SuperExecMessage`) to stitch the untrusted agent's `stdin`/`stdout`/`stderr` zero-copy directly to the privileged program's descriptors.

---

## 🏗️ Project Scaffolding Philosophy ("Monolith Thinning")

The highly decoupled, heavily sub-crated structure of the `codex-rs` workspace is an intentional architectural response to early monolithic bloat. Historically, the `codex-core` crate and components like `chatwidget.rs` or `codex_message_processor.rs` grew to massive sizes (8k-10k+ lines of code) which produced bottlenecks. 

**Does this make it easier to work on?** Absolutely, for several critical reasons:

1. **Feature Isolation & Compile Times**: Instead of coupling all networking, protocol execution, sandboxing, and UI code together, splitting features into hyper-specific crates (`network-proxy`, `sandboxing`, `shell-escalation`) allows developers to build and test these deeply complex, OS-specific components independently. It significantly speeds up local `cargo test -p <crate>` development loops without invoking the entire system graph.
2. **"Thin Core" Initiative**: The `AGENTS.md` strictly enforces a policy to "resist adding code to `codex-core`". If you need a new feature, you must instantiate a new Rust crate. This actively mitigates merge conflicts and scopes boundaries.
3. **Multi-Language Hermetic Builds (Bazel + Cargo)**: The project orchestrates Rust (`codex-rs`), TypeScript/JavaScript (`sdk/`, root `package.json`), and OS-level resources inside a unified Bazel-managed monorepo. While developers can use standard `cargo` or `just` scripts locally to stay agile, the overarching Bazel structure (seen via `MODULE.bazel`) ensures that when protocol definitions (`app-server-schema`) change, both the backend Rust and the frontend TypeScript SDK correctly align in a single deterministic, reproducible CI state. 
4. **Targeted Testing Isolation**: It simplifies robust verification strategies like snapshot testing with `cargo-insta`. A UI developer can run visual ratatui rendering snapshots (`cargo test -p codex-tui`) without touching sandbox internals.

Ultimately, the heavily componentized scaffold is a strict technical enforcement of boundaries designed to keep the project agile despite its rapidly expanding and highly-sensitive capability set.

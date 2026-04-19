# Technical Decisions Report

**Generated:** 2026-04-06T17:49:22.385969+00:00
**Status:** OK
**Coverage Score:** 100%

## Required Sections

- **Problem & Job:** yes
- **Success Criteria:** yes
- **Scope:** yes
- **Primary User Journey:** yes

## Conditional Sections

- **Assumption & Risk Map:** no
- **Opportunity Solution Tree:** no
- **Build Breakdown:** no
## Required Sections Analysis

### Problem And Job

**Status:** Detected

**Evidence:**
- quick-xml (title/narrative)
- ratatui (title/narrative)
- codex-execpolicy (title/narrative)
- assert_cmd (title/narrative)
- insta (title/narrative)
- ... and 251 more

### Success Criteria

**Status:** Detected

**Evidence:**
- reqwest (analysis)
- codex-utils-json-to-toml (analysis)
- codex-secrets (analysis)
- codex-state (analysis)
- allocative (analysis)
- ... and 251 more

### Scope

**Status:** Detected

**Evidence:**
- codex-connectors (version: workspace)
- codex-utils-stream-parser (version: workspace)
- codex-utils-path (version: workspace)
- codex-git-utils (version: workspace)
- predicates (version: 3)
- ... and 251 more

### Primary User Journey

**Status:** Detected

**Evidence:**
- tracing-appender (narrative)
- indexmap (narrative)
- ratatui (narrative)
- time (narrative)
- tokio (narrative)
- ... and 251 more

## Conditional Sections Analysis

### Assumption Risk Map

**Status:** Not detected

### Opportunity Solution Tree

**Status:** Not detected

### Build Breakdown

**Status:** Not detected

## Technical Decisions

Total decisions analyzed: 256

### app_test_support

**Version:** app-server/tests/common
**Type:** workspace

**Title:** Use app_test_support for workspace dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-analytics

**Version:** workspace
**Type:** production

**Title:** Use codex-analytics for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-ansi-escape

**Version:** workspace
**Type:** production

**Title:** Use codex-ansi-escape for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-api

**Version:** workspace
**Type:** production

**Title:** Use codex-api for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-app-server

**Version:** workspace
**Type:** production

**Title:** Use codex-app-server for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-app-server-client

**Version:** workspace
**Type:** production

**Title:** Use codex-app-server-client for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-app-server-protocol

**Version:** workspace
**Type:** production

**Title:** Use codex-app-server-protocol for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-app-server-test-client

**Version:** workspace
**Type:** production

**Title:** Use codex-app-server-test-client for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-apply-patch

**Version:** workspace
**Type:** production

**Title:** Use codex-apply-patch for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-arg0

**Version:** workspace
**Type:** production

**Title:** Use codex-arg0 for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-async-utils

**Version:** workspace
**Type:** production

**Title:** Use codex-async-utils for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-backend-client

**Version:** workspace
**Type:** production

**Title:** Use codex-backend-client for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-chatgpt

**Version:** workspace
**Type:** production

**Title:** Use codex-chatgpt for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-cli

**Version:** cli
**Type:** workspace

**Title:** Use codex-cli for workspace dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-client

**Version:** workspace
**Type:** production

**Title:** Use codex-client for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-collaboration-mode-templates

**Version:** workspace
**Type:** production

**Title:** Use codex-collaboration-mode-templates for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-cloud-requirements

**Version:** workspace
**Type:** production

**Title:** Use codex-cloud-requirements for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-cloud-tasks-client

**Version:** workspace
**Type:** production

**Title:** Use codex-cloud-tasks-client for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-cloud-tasks-mock-client

**Version:** workspace
**Type:** production

**Title:** Use codex-cloud-tasks-mock-client for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-code-mode

**Version:** workspace
**Type:** production

**Title:** Use codex-code-mode for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-config

**Version:** workspace
**Type:** production

**Title:** Use codex-config for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-connectors

**Version:** workspace
**Type:** production

**Title:** Use codex-connectors for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-core

**Version:** workspace
**Type:** production

**Title:** Use codex-core for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-core-skills

**Version:** workspace
**Type:** production

**Title:** Use codex-core-skills for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-exec

**Version:** workspace
**Type:** production

**Title:** Use codex-exec for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-exec-server

**Version:** workspace
**Type:** production

**Title:** Use codex-exec-server for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-execpolicy

**Version:** workspace
**Type:** production

**Title:** Use codex-execpolicy for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-experimental-api-macros

**Version:** workspace
**Type:** production

**Title:** Use codex-experimental-api-macros for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-features

**Version:** workspace
**Type:** production

**Title:** Use codex-features for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-feedback

**Version:** workspace
**Type:** production

**Title:** Use codex-feedback for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-file-search

**Version:** workspace
**Type:** production

**Title:** Use codex-file-search for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-git-utils

**Version:** workspace
**Type:** production

**Title:** Use codex-git-utils for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-hooks

**Version:** workspace
**Type:** production

**Title:** Use codex-hooks for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-instructions

**Version:** workspace
**Type:** production

**Title:** Use codex-instructions for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-keyring-store

**Version:** workspace
**Type:** production

**Title:** Use codex-keyring-store for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-linux-sandbox

**Version:** workspace
**Type:** production

**Title:** Use codex-linux-sandbox for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-lmstudio

**Version:** workspace
**Type:** production

**Title:** Use codex-lmstudio for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-login

**Version:** workspace
**Type:** production

**Title:** Use codex-login for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-mcp

**Version:** workspace
**Type:** production

**Title:** Use codex-mcp for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-mcp-server

**Version:** workspace
**Type:** production

**Title:** Use codex-mcp-server for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-model-provider-info

**Version:** workspace
**Type:** production

**Title:** Use codex-model-provider-info for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-models-manager

**Version:** workspace
**Type:** production

**Title:** Use codex-models-manager for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-network-proxy

**Version:** workspace
**Type:** production

**Title:** Use codex-network-proxy for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-ollama

**Version:** workspace
**Type:** production

**Title:** Use codex-ollama for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-otel

**Version:** workspace
**Type:** production

**Title:** Use codex-otel for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-plugin

**Version:** workspace
**Type:** production

**Title:** Use codex-plugin for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-process-hardening

**Version:** workspace
**Type:** production

**Title:** Use codex-process-hardening for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-protocol

**Version:** ../protocol
**Type:** production

**Title:** Use codex-protocol for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-responses-api-proxy

**Version:** workspace
**Type:** production

**Title:** Use codex-responses-api-proxy for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-response-debug-context

**Version:** workspace
**Type:** production

**Title:** Use codex-response-debug-context for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-rmcp-client

**Version:** workspace
**Type:** production

**Title:** Use codex-rmcp-client for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-rollout

**Version:** workspace
**Type:** production

**Title:** Use codex-rollout for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-sandboxing

**Version:** workspace
**Type:** production

**Title:** Use codex-sandboxing for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-secrets

**Version:** workspace
**Type:** production

**Title:** Use codex-secrets for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-shell-command

**Version:** workspace
**Type:** production

**Title:** Use codex-shell-command for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-shell-escalation

**Version:** workspace
**Type:** production

**Title:** Use codex-shell-escalation for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-skills

**Version:** workspace
**Type:** production

**Title:** Use codex-skills for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-state

**Version:** workspace
**Type:** production

**Title:** Use codex-state for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-stdio-to-uds

**Version:** workspace
**Type:** production

**Title:** Use codex-stdio-to-uds for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-terminal-detection

**Version:** workspace
**Type:** production

**Title:** Use codex-terminal-detection for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-tools

**Version:** workspace
**Type:** production

**Title:** Use codex-tools for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-tui

**Version:** workspace
**Type:** production

**Title:** Use codex-tui for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-utils-absolute-path

**Version:** workspace
**Type:** production

**Title:** Use codex-utils-absolute-path for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-utils-approval-presets

**Version:** workspace
**Type:** production

**Title:** Use codex-utils-approval-presets for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-utils-cache

**Version:** workspace
**Type:** production

**Title:** Use codex-utils-cache for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-utils-cargo-bin

**Version:** workspace
**Type:** production

**Title:** Use codex-utils-cargo-bin for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-utils-cli

**Version:** workspace
**Type:** production

**Title:** Use codex-utils-cli for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-utils-elapsed

**Version:** workspace
**Type:** production

**Title:** Use codex-utils-elapsed for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-utils-fuzzy-match

**Version:** workspace
**Type:** production

**Title:** Use codex-utils-fuzzy-match for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-utils-home-dir

**Version:** workspace
**Type:** production

**Title:** Use codex-utils-home-dir for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-utils-image

**Version:** workspace
**Type:** production

**Title:** Use codex-utils-image for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-utils-json-to-toml

**Version:** workspace
**Type:** production

**Title:** Use codex-utils-json-to-toml for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-utils-oss

**Version:** workspace
**Type:** production

**Title:** Use codex-utils-oss for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-utils-output-truncation

**Version:** workspace
**Type:** production

**Title:** Use codex-utils-output-truncation for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-utils-path

**Version:** workspace
**Type:** production

**Title:** Use codex-utils-path for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-utils-plugins

**Version:** workspace
**Type:** production

**Title:** Use codex-utils-plugins for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-utils-pty

**Version:** workspace
**Type:** production

**Title:** Use codex-utils-pty for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-utils-readiness

**Version:** workspace
**Type:** production

**Title:** Use codex-utils-readiness for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-utils-rustls-provider

**Version:** workspace
**Type:** production

**Title:** Use codex-utils-rustls-provider for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-utils-sandbox-summary

**Version:** workspace
**Type:** production

**Title:** Use codex-utils-sandbox-summary for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-utils-sleep-inhibitor

**Version:** workspace
**Type:** production

**Title:** Use codex-utils-sleep-inhibitor for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-utils-stream-parser

**Version:** workspace
**Type:** production

**Title:** Use codex-utils-stream-parser for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-utils-string

**Version:** workspace
**Type:** production

**Title:** Use codex-utils-string for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-utils-template

**Version:** workspace
**Type:** production

**Title:** Use codex-utils-template for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-v8-poc

**Version:** v8-poc
**Type:** workspace

**Title:** Use codex-v8-poc for workspace dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-windows-sandbox

**Version:** workspace
**Type:** production

**Title:** Use codex-windows-sandbox for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### core_test_support

**Version:** ../../../core/tests/common
**Type:** production

**Title:** Use core_test_support for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### mcp_test_support

**Version:** mcp-server/tests/common
**Type:** workspace

**Title:** Use mcp_test_support for workspace dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### age

**Version:** workspace
**Type:** production

**Title:** Use age for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### allocative

**Version:** workspace
**Type:** production

**Title:** Use allocative for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### ansi-to-tui

**Version:** workspace
**Type:** production

**Title:** Use ansi-to-tui for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### anyhow

**Version:** 1.0
**Type:** production

**Title:** Use anyhow for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### arboard

**Version:** 3
**Type:** workspace

**Title:** Use arboard for workspace dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### arc-swap

**Version:** workspace
**Type:** production

**Title:** Use arc-swap for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### assert_cmd

**Version:** workspace
**Type:** production

**Title:** Use assert_cmd for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### assert_matches

**Version:** 1.5.0
**Type:** workspace

**Title:** Use assert_matches for workspace dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### async-channel

**Version:** workspace
**Type:** production

**Title:** Use async-channel for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### async-stream

**Version:** workspace
**Type:** production

**Title:** Use async-stream for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### async-trait

**Version:** workspace
**Type:** production

**Title:** Use async-trait for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### axum

**Version:** workspace
**Type:** production

**Title:** Use axum for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### base64

**Version:** workspace
**Type:** production

**Title:** Use base64 for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### bm25

**Version:** workspace
**Type:** production

**Title:** Use bm25 for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### bytes

**Version:** workspace
**Type:** production

**Title:** Use bytes for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### chardetng

**Version:** workspace
**Type:** production

**Title:** Use chardetng for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### chrono

**Version:** 0.4.42
**Type:** production

**Title:** Use chrono for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### clap

**Version:** workspace
**Type:** production

**Title:** Use clap for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### clap_complete

**Version:** workspace
**Type:** production

**Title:** Use clap_complete for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### color-eyre

**Version:** workspace
**Type:** production

**Title:** Use color-eyre for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### constant_time_eq

**Version:** workspace
**Type:** production

**Title:** Use constant_time_eq for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### crossbeam-channel

**Version:** workspace
**Type:** production

**Title:** Use crossbeam-channel for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### crossterm

**Version:** workspace
**Type:** production

**Title:** Use crossterm for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### csv

**Version:** workspace
**Type:** production

**Title:** Use csv for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### ctor

**Version:** workspace
**Type:** production

**Title:** Use ctor for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### derive_more

**Version:** workspace
**Type:** production

**Title:** Use derive_more for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### diffy

**Version:** workspace
**Type:** production

**Title:** Use diffy for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### dirs

**Version:** workspace
**Type:** production

**Title:** Use dirs for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### dotenvy

**Version:** workspace
**Type:** production

**Title:** Use dotenvy for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### dunce

**Version:** 1.0
**Type:** production

**Title:** Use dunce for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### encoding_rs

**Version:** workspace
**Type:** production

**Title:** Use encoding_rs for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### env-flags

**Version:** workspace
**Type:** production

**Title:** Use env-flags for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### env_logger

**Version:** workspace
**Type:** production

**Title:** Use env_logger for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### eventsource-stream

**Version:** workspace
**Type:** production

**Title:** Use eventsource-stream for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### futures

**Version:** workspace
**Type:** production

**Title:** Use futures for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### gethostname

**Version:** workspace
**Type:** production

**Title:** Use gethostname for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### globset

**Version:** workspace
**Type:** production

**Title:** Use globset for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### hmac

**Version:** 0.12.1
**Type:** production

**Title:** Use hmac for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### http

**Version:** workspace
**Type:** production

**Title:** Use http for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### iana-time-zone

**Version:** workspace
**Type:** production

**Title:** Use iana-time-zone for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### icu_decimal

**Version:** workspace
**Type:** production

**Title:** Use icu_decimal for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### icu_locale_core

**Version:** workspace
**Type:** production

**Title:** Use icu_locale_core for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### icu_provider

**Version:** workspace
**Type:** production

**Title:** Use icu_provider for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### ignore

**Version:** workspace
**Type:** production

**Title:** Use ignore for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### image

**Version:** workspace
**Type:** production

**Title:** Use image for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### include_dir

**Version:** workspace
**Type:** production

**Title:** Use include_dir for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### indexmap

**Version:** workspace
**Type:** production

**Title:** Use indexmap for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### insta

**Version:** 1.46.3
**Type:** workspace

**Title:** Use insta for workspace dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### inventory

**Version:** workspace
**Type:** production

**Title:** Use inventory for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### itertools

**Version:** workspace
**Type:** production

**Title:** Use itertools for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### jsonwebtoken

**Version:** workspace
**Type:** production

**Title:** Use jsonwebtoken for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### keyring

**Version:** workspace
**Type:** production

**Title:** Use keyring for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### landlock

**Version:** 0.4.4
**Type:** workspace

**Title:** Use landlock for workspace dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### lazy_static

**Version:** workspace
**Type:** production

**Title:** Use lazy_static for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### libc

**Version:** workspace
**Type:** production

**Title:** Use libc for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### log

**Version:** workspace
**Type:** production

**Title:** Use log for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### lru

**Version:** workspace
**Type:** production

**Title:** Use lru for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### maplit

**Version:** 1.0.2
**Type:** workspace

**Title:** Use maplit for workspace dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### mime_guess

**Version:** workspace
**Type:** production

**Title:** Use mime_guess for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### multimap

**Version:** workspace
**Type:** production

**Title:** Use multimap for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### notify

**Version:** workspace
**Type:** production

**Title:** Use notify for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### nucleo

**Version:** workspace
**Type:** production

**Title:** Use nucleo for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### once_cell

**Version:** workspace
**Type:** production

**Title:** Use once_cell for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### openssl-sys

**Version:** *
**Type:** workspace

**Title:** Use openssl-sys for workspace dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### opentelemetry

**Version:** workspace
**Type:** production

**Title:** Use opentelemetry for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### opentelemetry-appender-tracing

**Version:** workspace
**Type:** production

**Title:** Use opentelemetry-appender-tracing for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### opentelemetry-otlp

**Version:** workspace
**Type:** production

**Title:** Use opentelemetry-otlp for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### opentelemetry-semantic-conventions

**Version:** workspace
**Type:** production

**Title:** Use opentelemetry-semantic-conventions for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### opentelemetry_sdk

**Version:** workspace
**Type:** production

**Title:** Use opentelemetry_sdk for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### os_info

**Version:** workspace
**Type:** production

**Title:** Use os_info for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### owo-colors

**Version:** workspace
**Type:** production

**Title:** Use owo-colors for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### path-absolutize

**Version:** workspace
**Type:** production

**Title:** Use path-absolutize for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### pathdiff

**Version:** workspace
**Type:** production

**Title:** Use pathdiff for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### portable-pty

**Version:** workspace
**Type:** production

**Title:** Use portable-pty for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### predicates

**Version:** 3
**Type:** workspace

**Title:** Use predicates for workspace dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### pretty_assertions

**Version:** workspace
**Type:** production

**Title:** Use pretty_assertions for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### pulldown-cmark

**Version:** workspace
**Type:** production

**Title:** Use pulldown-cmark for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### quick-xml

**Version:** workspace
**Type:** production

**Title:** Use quick-xml for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### rand

**Version:** 0.8
**Type:** production

**Title:** Use rand for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### ratatui

**Version:** workspace
**Type:** production

**Title:** Use ratatui for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### ratatui-macros

**Version:** workspace
**Type:** production

**Title:** Use ratatui-macros for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### regex

**Version:** workspace
**Type:** production

**Title:** Use regex for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### regex-lite

**Version:** workspace
**Type:** production

**Title:** Use regex-lite for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### reqwest

**Version:** workspace
**Type:** production

**Title:** Use reqwest for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### rmcp

**Version:** workspace
**Type:** production

**Title:** Use rmcp for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### runfiles

**Version:** workspace
**Type:** production

**Title:** Use runfiles for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### rustls

**Version:** workspace
**Type:** production

**Title:** Use rustls for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### rustls-native-certs

**Version:** workspace
**Type:** production

**Title:** Use rustls-native-certs for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### rustls-pki-types

**Version:** workspace
**Type:** production

**Title:** Use rustls-pki-types for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### schemars

**Version:** workspace
**Type:** production

**Title:** Use schemars for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### seccompiler

**Version:** 0.5.0
**Type:** workspace

**Title:** Use seccompiler for workspace dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### semver

**Version:** workspace
**Type:** production

**Title:** Use semver for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### sentry

**Version:** 0.46
**Type:** production

**Title:** Use sentry for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### serde

**Version:** 1.0
**Type:** production

**Title:** Use serde for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### serde_json

**Version:** 1.0
**Type:** production

**Title:** Use serde_json for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### serde_path_to_error

**Version:** workspace
**Type:** production

**Title:** Use serde_path_to_error for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### serde_with

**Version:** workspace
**Type:** production

**Title:** Use serde_with for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### serde_yaml

**Version:** workspace
**Type:** production

**Title:** Use serde_yaml for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### serial_test

**Version:** 3.2.0
**Type:** workspace

**Title:** Use serial_test for workspace dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### sha1

**Version:** workspace
**Type:** production

**Title:** Use sha1 for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### sha2

**Version:** workspace
**Type:** production

**Title:** Use sha2 for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### shlex

**Version:** workspace
**Type:** production

**Title:** Use shlex for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### similar

**Version:** workspace
**Type:** production

**Title:** Use similar for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### socket2

**Version:** workspace
**Type:** production

**Title:** Use socket2 for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### sqlx

**Version:** workspace
**Type:** production

**Title:** Use sqlx for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### starlark

**Version:** workspace
**Type:** production

**Title:** Use starlark for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### strum

**Version:** workspace
**Type:** production

**Title:** Use strum for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### strum_macros

**Version:** workspace
**Type:** production

**Title:** Use strum_macros for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### supports-color

**Version:** workspace
**Type:** production

**Title:** Use supports-color for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### syntect

**Version:** 5
**Type:** production

**Title:** Use syntect for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### sys-locale

**Version:** workspace
**Type:** production

**Title:** Use sys-locale for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### tempfile

**Version:** 3
**Type:** production

**Title:** Use tempfile for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### test-log

**Version:** workspace
**Type:** production

**Title:** Use test-log for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### textwrap

**Version:** workspace
**Type:** production

**Title:** Use textwrap for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### thiserror

**Version:** workspace
**Type:** production

**Title:** Use thiserror for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### time

**Version:** workspace
**Type:** production

**Title:** Use time for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### tiny_http

**Version:** workspace
**Type:** production

**Title:** Use tiny_http for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### tokio

**Version:** workspace
**Type:** production

**Title:** Use tokio for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### tokio-stream

**Version:** workspace
**Type:** production

**Title:** Use tokio-stream for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### tokio-test

**Version:** 0.4
**Type:** workspace

**Title:** Use tokio-test for workspace dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### tokio-tungstenite

**Version:** workspace
**Type:** production

**Title:** Use tokio-tungstenite for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### tokio-util

**Version:** workspace
**Type:** production

**Title:** Use tokio-util for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### toml

**Version:** workspace
**Type:** production

**Title:** Use toml for production dependencies

**Why this choice?**

Primary consideration: API design.

**Alternatives considered:**

- **json5** - Architecture or API mismatch with project requirements
- **yaml** - Architecture or API mismatch with project requirements

---

### toml_edit

**Version:** workspace
**Type:** production

**Title:** Use toml_edit for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### tracing

**Version:** workspace
**Type:** production

**Title:** Use tracing for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### tracing-appender

**Version:** workspace
**Type:** production

**Title:** Use tracing-appender for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### tracing-opentelemetry

**Version:** workspace
**Type:** production

**Title:** Use tracing-opentelemetry for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### tracing-subscriber

**Version:** workspace
**Type:** production

**Title:** Use tracing-subscriber for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### tracing-test

**Version:** 0.2.5
**Type:** workspace

**Title:** Use tracing-test for workspace dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### tree-sitter

**Version:** workspace
**Type:** production

**Title:** Use tree-sitter for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### tree-sitter-bash

**Version:** workspace
**Type:** production

**Title:** Use tree-sitter-bash for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### ts-rs

**Version:** workspace
**Type:** production

**Title:** Use ts-rs for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### tungstenite

**Version:** workspace
**Type:** production

**Title:** Use tungstenite for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### uds_windows

**Version:** 1.1.0
**Type:** workspace

**Title:** Use uds_windows for workspace dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### unicode-segmentation

**Version:** workspace
**Type:** production

**Title:** Use unicode-segmentation for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### unicode-width

**Version:** workspace
**Type:** production

**Title:** Use unicode-width for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### url

**Version:** workspace
**Type:** production

**Title:** Use url for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### urlencoding

**Version:** workspace
**Type:** production

**Title:** Use urlencoding for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### uuid

**Version:** workspace
**Type:** production

**Title:** Use uuid for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### v8

**Version:** workspace
**Type:** production

**Title:** Use v8 for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### vt100

**Version:** 0.16.2
**Type:** workspace

**Title:** Use vt100 for workspace dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### walkdir

**Version:** workspace
**Type:** production

**Title:** Use walkdir for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### webbrowser

**Version:** workspace
**Type:** production

**Title:** Use webbrowser for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### which

**Version:** workspace
**Type:** production

**Title:** Use which for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### wildmatch

**Version:** workspace
**Type:** production

**Title:** Use wildmatch for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### zip

**Version:** workspace
**Type:** production

**Title:** Use zip for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### zstd

**Version:** workspace
**Type:** production

**Title:** Use zstd for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### wiremock

**Version:** workspace
**Type:** production

**Title:** Use wiremock for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### zeroize

**Version:** workspace
**Type:** production

**Title:** Use zeroize for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-backend-openapi-models

**Version:** ../codex-backend-openapi-models
**Type:** production

**Title:** Use codex-backend-openapi-models for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### codex-cloud-tasks

**Version:** ../cloud-tasks
**Type:** production

**Title:** Use codex-cloud-tasks for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### proc-macro2

**Version:** 1
**Type:** production

**Title:** Use proc-macro2 for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### quote

**Version:** 1
**Type:** production

**Title:** Use quote for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### syn

**Version:** 2
**Type:** production

**Title:** Use syn for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### rama-core

**Version:** =0.3.0-alpha.4
**Type:** production

**Title:** Use rama-core for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### rama-http

**Version:** =0.3.0-alpha.4
**Type:** production

**Title:** Use rama-http for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### rama-http-backend

**Version:** =0.3.0-alpha.4
**Type:** production

**Title:** Use rama-http-backend for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### rama-net

**Version:** =0.3.0-alpha.4
**Type:** production

**Title:** Use rama-net for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### rama-socks5

**Version:** =0.3.0-alpha.4
**Type:** production

**Title:** Use rama-socks5 for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### rama-tcp

**Version:** =0.3.0-alpha.4
**Type:** production

**Title:** Use rama-tcp for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### rama-tls-rustls

**Version:** =0.3.0-alpha.4
**Type:** production

**Title:** Use rama-tls-rustls for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### oauth2

**Version:** 5
**Type:** production

**Title:** Use oauth2 for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### sse-stream

**Version:** 0.2.1
**Type:** production

**Title:** Use sse-stream for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### two-face

**Version:** 0.5
**Type:** production

**Title:** Use two-face for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### windows

**Version:** 0.58
**Type:** production

**Title:** Use windows for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### dirs-next

**Version:** 2.0
**Type:** production

**Title:** Use dirs-next for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### clippy_utils

**Version:** workspace
**Type:** production

**Title:** Use clippy_utils for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

### dylint_linting

**Version:** 5.0.0
**Type:** production

**Title:** Use dylint_linting for production dependencies

**Why this choice?**

No specific alternatives identified. This appears to be a standard choice for this category.

---

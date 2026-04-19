# Architecture Analysis Report

**Generated:** 2026-04-06T17:49:06.531624+00:00
**Total Modules:** 1447
**Total LOC:** 541,710

## Summary

- **Average Complexity:** 12.4
- **Max Coupling:** 385 imports
- **Circular Dependencies:** 0

## Language Distribution

- **python:** 77 modules
- **rust:** 1370 modules

## Partial Coverage

- **bash:** 16 files detected but not modeled in the architecture graph
- **c:** 1 files detected but not modeled in the architecture graph
- **javascript:** 3 files detected but not modeled in the architecture graph
- **json:** 233 files detected but not modeled in the architecture graph
- **text:** 1185 files detected but not modeled in the architecture graph
- **toml:** 110 files detected but not modeled in the architecture graph
- **typescript:** 444 files detected but not modeled in the architecture graph
- **yaml:** 40 files detected but not modeled in the architecture graph

## Hotspots

### Large Modules (>500 LOC)

- **sdk/python/scripts/update_sdk_artifacts.py** (829 LOC, 48 functions)
- **sdk/python/src/codex_app_server/api.py** (656 LOC, 45 functions)
- **sdk/python/src/codex_app_server/generated/v2_all.py** (5566 LOC, 0 functions)
- **.codex/skills/babysit-pr/scripts/gh_pr_watch.py** (677 LOC, 42 functions)
- **codex-rs/terminal-detection/src/terminal_tests.rs** (778 LOC, 26 functions)
- **codex-rs/analytics/src/analytics_client_tests.rs** (758 LOC, 25 functions)
- **codex-rs/login/src/server.rs** (1127 LOC, 34 functions)
- **codex-rs/login/src/auth/auth_tests.rs** (703 LOC, 31 functions)
- **codex-rs/login/src/auth/manager.rs** (1461 LOC, 97 functions)
- **codex-rs/login/tests/suite/auth_refresh.rs** (900 LOC, 25 functions)

### High Coupling (>20 imports)

- **sdk/python/src/codex_app_server/generated/notification_registry.py** (53 imports)
  - Depends on: __future__, pydantic, v2_all, v2_all, v2_all...
- **codex-rs/analytics/src/reducer.rs** (38 imports)
  - Depends on: crate, crate, crate, crate, crate...
- **codex-rs/analytics/src/analytics_client_tests.rs** (59 imports)
  - Depends on: crate, crate, crate, crate, crate...
- **codex-rs/analytics/src/client.rs** (26 imports)
  - Depends on: crate, crate, crate, crate, crate...
- **codex-rs/login/src/lib.rs** (46 imports)
  - Depends on: api_bridge, auth, auth_env_telemetry, provider_auth, token_data...
- **codex-rs/login/src/server.rs** (45 imports)
  - Depends on: std, std, std, std, std...
- **codex-rs/login/src/auth/manager.rs** (37 imports)
  - Depends on: async_trait, chrono, reqwest, serde, serde...
- **codex-rs/login/src/auth/storage.rs** (25 imports)
  - Depends on: chrono, chrono, schemars, serde, serde...
- **codex-rs/login/tests/suite/auth_refresh.rs** (28 imports)
  - Depends on: anyhow, anyhow, base64, chrono, chrono...
- **codex-rs/shell-command/src/command_safety/powershell_parser.rs** (21 imports)
  - Depends on: base64, base64, serde, serde, std...

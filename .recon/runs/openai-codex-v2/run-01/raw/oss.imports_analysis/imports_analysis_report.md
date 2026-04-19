# Imports Analysis Report

**Repository**: `/Users/jamiecraik/dev/codex`
**Scan Date**: 2026-04-06T17:49:12.695542+00:00
**Files Scanned**: 448

## Summary

| Category | Count |
|----------|-------|
| npm imports | 41 |
| Relative imports | 27 |
| Alias imports | 0 |
| Workspace package imports | 3 |
| Dynamic expression imports | 5 |
| Unknown bare imports | 7 |
| **Total** | **83** |

## Internal Utilities Candidates

Found 3 small, frequently-imported local modules
that could potentially be extracted to standalone packages.

### 1. `sdk/typescript/samples/helpers.ts` (3 imports)

- **Size**: ~7 LOC
- **Category**: utility
- **Exports**: `codexPathOverride`
- **Imported by** (3 files):
  - `/Users/jamiecraik/dev/codex/sdk/typescript/samples/basic_streaming.ts`
  - `/Users/jamiecraik/dev/codex/sdk/typescript/samples/structured_output.ts`
  - `/Users/jamiecraik/dev/codex/sdk/typescript/samples/structured_output_zod.ts`

### 2. `sdk/typescript/src/threadOptions.ts` (3 imports)

- **Size**: ~16 LOC
- **Category**: types
- **Exports**: `type`
- **Imported by** (3 files):
  - `/Users/jamiecraik/dev/codex/sdk/typescript/src/codex.ts`
  - `/Users/jamiecraik/dev/codex/sdk/typescript/src/exec.ts`
  - `/Users/jamiecraik/dev/codex/sdk/typescript/src/thread.ts`

### 3. `sdk/typescript/tests/testCodex.ts` (3 imports)

- **Size**: ~89 LOC
- **Category**: types
- **Exports**: `codexExecPath`, `createMockClient`, `createTestClient`, `type`
- **Imported by** (3 files):
  - `/Users/jamiecraik/dev/codex/sdk/typescript/tests/abort.test.ts`
  - `/Users/jamiecraik/dev/codex/sdk/typescript/tests/run.test.ts`
  - `/Users/jamiecraik/dev/codex/sdk/typescript/tests/runStreamed.test.ts`

## Frequently Imported Internals

| Import | Count | Type |
|--------|-------|------|
| `./helpers.ts` | 3 | relative |
| `../src/exec` | 3 | relative |
| `./threadOptions` | 3 | relative |
| `./testCodex` | 3 | relative |

## Unknown Imports

Found 7 import statements that don't match npm, 
relative, or configured alias patterns:

### `@prev`
- **File**: `/Users/jamiecraik/dev/codex/codex-rs/core/src/tools/js_repl/kernel.js`
- **Line**: 1009
- **Type**: namespace

### `@eslint/js`
- **File**: `/Users/jamiecraik/dev/codex/sdk/typescript/eslint.config.js`
- **Line**: 1
- **Type**: default

### `@jest/globals`
- **File**: `/Users/jamiecraik/dev/codex/sdk/typescript/tests/abort.test.ts`
- **Line**: 1
- **Type**: named

### `@jest/globals`
- **File**: `/Users/jamiecraik/dev/codex/sdk/typescript/tests/exec.test.ts`
- **Line**: 5
- **Type**: named

### `@jest/globals`
- **File**: `/Users/jamiecraik/dev/codex/sdk/typescript/tests/run.test.ts`
- **Line**: 6
- **Type**: named

### `@jest/globals`
- **File**: `/Users/jamiecraik/dev/codex/sdk/typescript/tests/runStreamed.test.ts`
- **Line**: 1
- **Type**: named

### `@jest/globals`
- **File**: `/Users/jamiecraik/dev/codex/sdk/typescript/tests/setupCodexHome.ts`
- **Line**: 5
- **Type**: named

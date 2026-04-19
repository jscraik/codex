# Dependency & Bazel Health Audit

This audit evaluates the reliability, hermeticity, and maintenance health of the `codex-rs` build system, specifically focusing on the intersection of **Cargo** (inner-loop) and **Bazel** (outer-loop/CI).

---

## 🏗️ Build Infrastructure Scorecard

| Category | Status | Health | Description |
| :--- | :--- | :--- | :--- |
| **A. Toolchain Hermeticity** | ✅ High | 90% | Uses `@llvm` for cross-platform Clang/LLVM toolchains. |
| **B. Platform Compatibility** | ⚠️ Warning | 70% | High reliance on localized patches for Windows/gnullvm. |
| **C. Dependency Sync** | ✅ Stable | 95% | `just bazel-lock-check` protects against Cargo-Bazel drift. |
| **D. Advanced Patching** | 💎 Elite | 100% | Sophisticated patch stack for upstream Rust/LLVM fixes. |
| **E. RBE Readiness** | ⚠️ Warning | 50% | Local platform constraints currently favor host leaking. |

---

## 🔍 Critical Findings

### 1. The "Shadow" Patch Stack (High Value)
**Finding:** The project maintains a massive `/patches` directory (over 25 patches) targeting upstream build rules (`rules_rust`, `rules_rs`) and low-level libraries (`aws-lc-sys`, `abseil`, `llvm`).
- **Nugget:** This is a "Golden Nugget" for Jamie. The project is effectively running a **custom hardened fork** of the Rust Bazel ecosystem.
- **Risk:** High maintenance burden when upgrading Bazel modules; every upstream update risks breaking multiple local patches.

### 2. Windows gnullvm Specialization
**Finding:** Extensive effort has been invested in making **Windows-gnullvm** a first-class citizen.
- **Evidence:** `MODULE.bazel` includes specific patches for Abseil thread-identity and LLVM symlink extraction on Windows.
- **Nugget:** The project solves the "MinGW pthread TLS" mismatch by forcing abseil onto a portable C++11 path via a Bazel patch. This is a very non-obvious fix for cross-platform C++ interop in Rust.

### 3. Cargo-Bazel Convergence (`justfile` Integration)
**Finding:** The developer experience bridges the two systems via `just bazel-lock-update`.
- **Validation:** `scripts/check-module-bazel-lock.sh` is used in CI to ensure that whenever a developer changes `Cargo.toml`, the Bazel `MODULE.bazel.lock` is also updated. 
- **Health:** Excellent. This prevents "it works in Cargo but fails in PR" scenarios.

### 4. Non-Hermetic Leaks (`local_linux` platform)
**Finding:** The Bazel `local_linux` platform explicitly leaks host `glibc` constraints (`@llvm//constraints/libc:gnu.2.28`). 
- **Note:** This is documented as a workaround for musl-built rust failing to `dlopen` proc macros.
- **Risk:** This makes the build slightly non-hermetic. It may fail if moved to a CI runner with a significantly older or newer glibc than the developer's machine.

---

## 🛠️ Maintenance Recommendations

1. **Patch Upstreaming:** Identify which of the 25+ patches (especially those in `rules_rust`) can be upstreamed to reduce the local maintenance burden.
2. **glibc Standardize:** Consider moving to a container-based toolchain for `local_linux` to remove the host glibc dependency and achieve 100% hermeticity.
3. **Lockfile Enforcement:** Ensure the `bazel-lock-check` remains a **blocking** step in CI to maintain the tight synchronization between Cargo and Bazel.

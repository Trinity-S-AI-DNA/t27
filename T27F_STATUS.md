# T27F Status Report

**Last Updated:** 2026-10-30

## Executive Summary

**T27F — feature-complete, but not release-ready**

T27F has complete functional implementation but requires integration into the actual t27b build path and machine-verifiable proof before release.

## Status Matrix

```
T27F:
FEATURE_COMPLETE: ✅
INTEGRATION_COMPLETE: ⏳
RELEASE_READY: ⛔
RELEASED: ⛔

BLOCKERS:
- build_record.t27 not wired into t27b;
- REUSE proof missing;
- invalidation proof missing;
- artifact checksum proof missing;
- 5 open tracked/untracked items.
```

## Current State

### ✅ What's Complete

1. **Core Infrastructure**
   - Spec graph processing (exporter, derive, view, fold)
   - Visual rendering system with particle effects
   - Hot reload and streaming capabilities
   - TRI CLAW integration for spec validation
   - 94 T27F specifications in `specs/t27f/`

2. **Build Record Spec**
   - Complete `build_record.t27` spec (163 lines, 19 tests + invariants)
   - 4 build keys: closure spec hashes, binary identity, zig binary, flags
   - Artifact verification via shasum -a 256 -c
   - Decision logic: REUSE or first-change-wins invalidation

### ⏳ What's Pending

1. **Integration**
   - Wire `build_record.t27` into actual t27b build path
   - Create `cli/t27b/src/build_record.rs` module
   - Modify `cmd_build` function in `main.rs`

2. **Proofs**
   - REUSE proof: unchanged closure → no reparse/lower/encode
   - Invalidations proof: changed build key → first-change-wins
   - Artifact verification: shasum -a 256 -c passes

3. **Open Items**
   - #10554: External TypeScript MCP mount glue
   - #10861: Compiler defect #5980; gen-rust gap
   - #10727 Fix 1: t27c Rust change + owner-approved-foreign label
   - trinity#1648: Root T27F research epic
   - lesson_reel: Issue not yet filed

## Next Milestone

### T27F_BUILD_RECORD_WIRING_01

**Goal:** Wire `build_record.t27` into the actual t27b build path

**Acceptance Criteria:**

1. ✅ `build_record.t27` executes inside the real t27b build path
2. ✅ REUSE: unchanged closure → no reparse; no lower; no encode
3. ✅ INVALIDATION: changed build key → first-change-wins; stale outputs rejected
4. ✅ ARTIFACT: shasum -a 256 -c exits 0
5. ✅ All five open items are either closed or explicitly deferred
6. ✅ Immutable release tag is created only after all green evidence exists

**Issue:** https://github.com/Trinity-S-AI-DNA/t27/issues/12894

## Release Gate

```
NO WIRING => NO RELEASE.
NO REUSE PROOF => NO RELEASE.
NO INVALIDATION PROOF => NO RELEASE.
NO CHECKSUM PASS => NO RELEASE.
OPEN UNTRACKED GAP => NO RELEASE.

NEXT:
T27F_BUILD_RECORD_WIRING_01
```

## Required Evidence

For each proof step:

- **REUSE**
  - Input closure unchanged
  - Build executed twice
  - Second run reports cache/reuse
  - No reparse/lower/encode counters

- **Invalidation**
  - One build key changed
  - First-change-wins recorded
  - Affected artifact rebuilt
  - Unaffected artifacts reused

- **Artifact**
  - Checksum manifest generated
  - shasum -a 256 -c exit 0
  - Checksum output saved

- **Release**
  - Exact commands
  - Raw build logs
  - Exit codes
  - Commit SHA
  - build_record wiring diff
  - REUSE counters/logs
  - Invalidation case log
  - Checksum manifest
  - shasum verification output
  - Release tag
  - Rollback target

## Key Files

- **Spec:** `specs/tri/t27b/build_record.t27` (163 lines)
- **T27F Specs:** `specs/t27f/` (94 files)
- **Build Path:** `cli/t27b/src/main.rs` (cmd_build function)
- **Next Issue:** #12894 (T27F_BUILD_RECORD_WIRING_01)

## Conclusion

T27F is technically capable of its designed functionality, but cannot be formally released until:

1. Integration into t27b build path is complete
2. All proof steps are machine-verifiable
3. All blockers are resolved

**Status:** 🟡 Feature Complete / Integration Pending

**Next Action:** Begin T27F_BUILD_RECORD_WIRING_01 implementation

# Golden Master — Unified Spec Migration

## 🌟 STATUS: UNIFIED SPEC MIGRATION ACTIVATED

**Goal:** ALL specs live in `t27/specs/` as canonical source of truth.

> *Accepted: all specs from our repositories are transferred to a single `t27` as canonical source of truth.*
> *Other repositories may contain implementation/host adapters, but **specs, contracts, tests and golden artifacts live in `t27`**; in `golden-master` only what passed GOLDEN CHAIN enters.*

---

## 📁 Canonical Structure — `t27/specs/`

```
specs/
├── domains/           # Domain definitions (physics, math, etc.)
├── agents/            # Agent specs and behaviors
├── ml/                # Machine learning specs
├── runtime/           # Runtime-related specs
├── tools/             # Tool-related specs
├── tests/             # Test specification suites
├── golden/            # Golden-verified specs
├── archived/          # Duplicates/archived specs
├── compiler/          # Compiler-related specs
├── gateway/           # Gateway/interface specs
├── fpga/              # FPGA-related specs
├── protocol/          # Protocol definitions
├── tri/               # TRI-specific test specs
└── ci/                # CI-related specs
```

---

## 📦 Migration Rules — per ORDER

| # | Rule | Description |
|---|---|---|
| 1 | **One spec → One canonical path** | Each spec has exactly one home in `t27/specs/` |
| 2 | **Duplicates: choose canonical** | Vote by recency, authority, or stability; archive/remove others |
| 3 | **Preserve history** | Git history kept; `git log` on original path still works |
| 4 | **Preserve provenance** | Source/origin tracked in spec header comments |
| 5 | **No semantics change without tests** | Any change requires corresponding test update |
| 6 | **No implementation code migration** | Only specs, contracts, tests, golden artifacts move |
| 7 | **Other repos: specs → migrate** | All specs eventually move to t27 canonical home |
| 8 | **Other repos: implementation → remain** | Host adapters, build scripts stay in source repo |
| 9 | **Other repos: host adapters → remain** | t27 OS / platform-specific adapters remain where they are |
| 10 | **References → point to t27** | External repos reference `t27/specs/` canonical paths |

---

## 🏆 GOLDEN CHAIN Gate — 8 Conditions

Specs enter `golden-master` ONLY if ALL pass:

| # | Condition | Description |
|---|---|---|
| 1 | **t27b build PASS** | `cargo build --release -p t27b` succeeds on target platform |
| 2 | **tri test PASS** | `tri t27b test <spec>` passes |
| 3 | **golden test PASS** | Golden-specific validation (seals, coverage, invariants) |
| 4 | **regression PASS** | No regressions vs baseline (`tri diff` or equivalent) |
| 5 | **seals/gates PASS** | All seals and gates pass (own-language, conformance, dupe-ratchet) |
| 6 | **coverage delta recorded** | Coverage change documented in `docs/reports/coverage/` |
| 7 | **no shell violations** | No new `.sh` files; budget: ≤40 lines/file, ≤80 lines/PR |
| 8 | **no parallel-edit conflicts** | No concurrent edit conflicts with other working branches |

---

## 🌐 Other Repositories — Handling Policy

| Content | Action |
|---|---|
| **specs** | Migrate to `t27/specs/` canonical home |
| **implementation** | Remain in source repo if needed |
| **host adapters** | Remain if platform-specific; reference canonical spec |
| **references** | Point to `t27/specs/` canonical paths |
| **local spec forks** | Remove; use canonical t27 version |

---

## ⚡ CI Fast Path — golden-master

The golden-master CI fast path includes:

- **Caching**: opam package cache, compiler artifacts, test results
- **Path filters**: Only touched specs rebuilt (changed since last golden commit)
- **Fail-fast**: Early exit on first gate failure
- **Matrix splitting**: Fast compile/test vs heavy Coq proofs (separate nightly job)
- **Artifact reuse**: Between dependent jobs (build → test → seal)
- **Required vs advisory**: Only required gates block promotion; advisory gates inform but don't block
- **Nightly Coq proofs**: Heavy Coq proof compilation runs nightly, not on every PR

---

## 🛡️ Boundaries — Non-Negotiables

| Boundary | Rule |
|---|---|
| **No `.sh` files on critical path** | Use `tri` / `t27c` over shell scripts |
| **All new specs → `.t27`** | Language-first: LANG-EN (Law L3 — English identifiers only) |
| **Seals/gates not relaxed** | Current strictness maintained — no bypass |
| **No merge without green** | All 8 GOLDEN CHAIN gates must pass before merge |
| **No parallel edits** | Coordinate via milestones; stop on conflict, pass ownership |

---

## 📊 Final Report Format

```
REPOS_SCANNED:        <count>
SPECS_FOUND:          <count>
SPECS_MIGRATED:       <count>
DUPLICATES_RESOLVED:  <count>
GOLDEN_SPECS:         <count>
GOLDEN_MASTER_STATUS: <status (pass/fail/pending)>
CI_ACCELERATIONS:     <list>
BLOCKERS:             <list>
NEXT_ACTION:          <next concrete step>
```

---

## 📈 Migration Progress Tracker

| Metric | Current | Target | Status |
|---|---|---|---|
| Specs in t27/specs/ | 3252 | 3252 (unified) | ✅ Found |
| Duplicates resolved | 0 | Determined by inventory | 🔄 In Progress |
| Golden-verified specs | X | All passing GOLDEN CHAIN | 📊 Awaiting |
| CI acceleration ops | 7+ identified | All implemented | 📋 Planned |
| Repos scanned | 3+ | All | 🟡 Ongoing |

---

## 🎯 Next Actions — Immediate Priority

1. **Complete spec inventory** across all repos (~3376 total found)
2. **Identify duplicates** and choose canonical paths per voting criteria
3. **Move/mark** all specs to `t27/specs/` unified structure
4. **Archive/remove** non-canonical duplicates with history preserved
5. **Update references** in other repos to point to `t27/specs/` canonical paths
6. **Run GOLDEN CHAIN** on representative specs across domains
7. **Implement CI fast path** configurations for path filters, caching, incremental builds
8. **Generate final report** with all metrics

---

## 📋 Order Recap

| Order | Status |
|---|---|
| ✅ Golden master structure with GOLDEN CHAIN gate | Complete |
| ✅ Coq proofs fix (coq-mathcomp-algebra-tactics) | Complete |
| ✅ CI acceleration audit across 70+ workflows | Complete |
| ✅ Promotion pipeline (master → candidate → golden-master → release) | Complete |
| ✅ **Unified spec migration ACTIVATED** | **🟢 In Progress** |
| ✅ All specs live in t27 as canonical source | **Goal** |
| ✅ No experimental/WIP code in golden-master | ✅ Enforced |

---

**One agent — one worktree — one set of recording paths.** No seal/gate bypass; no parallel edits without coordination. All new specs go in `.t27`; LANG-EN (Law L3) identifiers only. Seals/gates at current strictness — no relaxation.

*Generated: Unified Spec Migration Activated — t27 canonical source of truth*

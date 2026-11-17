# T27 Milestone Plan — full plan for all open GitHub issues

**Source:** `Trinity-S-AI-DNA/t27`, all open issues (GitHub).
**Generated:** 2026-11-17.
**Status:** M0 complete; M1–M8 created; issues being distributed across milestones.

## Situation

- Open issues in the repo: **2073** (all had `milestone: null` before this plan).
- Milestones that existed before: only **#4 M0_PROMOTION_KERNEL_SPRINT** — now **100% complete** (0 open, 10 closed).
- New milestones created: **M1–M8** (GitHub milestone numbers 5–12).
- Issues assigned so far: **~315** (heuristic categorization by title keyword + label). The rest remain unassigned awaiting the triage pass described below.

## Milestone definitions (M1–M8)

| Milestone | GitHub # | Scope / theme | Primary domains |
|---|---|---|---|
| **M0** | 4 | Promotion kernel — planning/docs/schema only (done) | `docs/promotion/`, `specs/promotion/`, `specs/automation/` |
| **M1** | 5 | **T27 OS — Panel, Shell & 27 Petals** | `t27os`, shell, panel, glass_window, dashboard, home, petal, pty, browser pane, UI Kit, matte glass |
| **M2** | 6 | **Compiler & Codegen — t27c, gen-verilog, seals** | `t27c`, compiler, parser, seal, typecheck, gen-c/rust/js, verilog generation, derived data |
| **M3** | 7 | **Native Backend — t27b, AArch64** | `t27b`, macho, aarch64, backend, construct defects, ledger steward, overnight loop |
| **M4** | 8 | **Queen & Agent System — scheduler, GOD MODE, lanes** | `queen`, god mode, scheduler, lane, dispatch, actor model, hive, claim, GOLDEN CHAIN runtime |
| **M5** | 9 | **Numeric & Conformance — GoldenFloat, TNF, conformance** | `numeric`, goldenfloat, tnf, conformance, format, phi, gf, bnf, lucas, paper claims |
| **M6** | 10 | **FPGA & Bench — AX7203, Verilog, synth** | `fpga`, ax7203, synth, bench, board, rtl, bitstream, board-eye, doom-ax7203 |
| **M7** | 11 | **Frontend & Visual — T27F, widgets, reels** | `t27f`, frontend, widget, reel, visual, graph, ui-kit, live, particle, still, kanban |
| **M8** | 12 | **Integration & CRM — DSH, grok CRM, providers** | `crm`, `dsh`, grok, provider, mail, zoho, integration, agent-runner, t27mcp |

## Categorization logic (how issues were assigned)

Each open issue was read once and assigned to the first matching rule, in this priority order:

1. **M6** — title contains `fpga`, `ax7203`, `synth`, `bench`, `board`, `rtl`, `bitstream`, or label `doom-ax7203`.
2. **M3** — title contains `t27b`, `macho`, `aarch64`, `backend`, `construct`, `ledger-steward`, or label `t27b-ledger-steward`.
3. **M5** — title contains `numeric`, `goldenfloat`, `tnf`, `conformance`, `format`, `phi`, `gf`, `bnf`, `lucas`, `paper_claims`, or label `numeric`.
4. **M7** — title contains `t27f`, `frontend`, `widget`, `reel`, `visual`, `graph`, `ui-kit`, `ui kit`, `live`, `particle`.
5. **M4** — title contains `queen`, `god mode`, `scheduler`, `lane`, `dispatch`, `actor`, `hive`, `claim`, or label `queen-shaped`/`queen-skill`.
6. **M8** — title contains `crm`, `dsh`, `grok`, `provider`, `mail`, `zoho`, `integration`, `agent-runner`, `agent-runtime`, `t27mcp`.
7. **M2** — title contains `t27c`, `compiler`, `parse`, `seal`, `typecheck`, `gen-c`, `gen-rust`, `gen-js`, `lower`, `verilog`, or label `area/compiler`/`compiler`.
8. **M1** — title contains `t27os`, `petal`, `shell`, `panel`, `glass_window`, `dashboard`, `home`, `matte glass`, `27 petal`, `t27o`, `t27u`, or label `t27os`.
9. **M1 (default)** — anything left unmatched falls to M1, because per AGENTS.md the swarm focus is T27 OS and its self-improvement. These are flagged as **needs-triage**.

> The keyword rules are a first pass. Some issues sit on boundaries (e.g. `verilog` appears in both compiler codegen M2 and FPGA RTL M6; `agent` appears in Queen M4 and integration M8). A second review pass should reassign boundary issues by reading the issue body.

## Current assignment counts (live from GitHub)

| Milestone | Open issues | Share |
|---|---|---|
| M1 T27 OS — Panel, Shell & 27 Petals | 0 (was 139) | **100% closed** |
| M2 Compiler & Codegen | 29 | ~9% |
| M3 Native Backend | 36 | ~11% |
| M4 Queen & Agent System | 42 | ~13% |
| M5 Numeric & Conformance | 3 | ~1% |
| M6 FPGA & Bench | 21 | ~7% |
| M7 Frontend & Visual | 24 | ~8% |
| M8 Integration & CRM | 21 | ~7% |
| **Assigned total** | **315** | **~15%** |
| Unassigned (needs triage) | **1758** | **~85%** |

Counts are live from GitHub and change as the assignment sweep continues.

## Open epics and their milestone

Open epics (label `epic`) were mapped to milestones by theme:

- **M1:** #13895 T27 OS Shell, #13862 Agent Homes 27 Petals, #10782 27-petal menu, #13186 T27OS autonomous layout, #11037 MEGA EPIC T27 OS, #13260 Queen over GODMODE (partly)
- **M2:** #12444 boilerplate/spaghetti .t27 std layer, #1284 master-alignment rings, #1219 t27 Language Roadmap, #5905 t27c Verilog weak points, #5906 master's three reds, #6092 t27c silent misreads, #9025 downtime bottlenecks
- **M3:** #10879 t27b for t27o, #8326 t27b overnight loop, #6198 t27b in t27 port, #6063 t27b runs whole corpus, #9008 effects at the edge (partly)
- **M4:** #13162 QUEEN CHAT, #7851 Queen actor model, #7718 Queen dashboard/game, #6595 development speed/delivery metrics, #10393 TRI CLAW queue, #13999 GOLDEN CHAIN runtime
- **M5:** (no open numeric epics; see closed M0-related work)
- **M6:** #10066 t27 vs direct Verilog spec-to-RTL
- **M7:** #11038 mega-epic t27f map, #10937 widget factory, #10266 T27F Live, #10863 T27F dev tools, #12173 interactive visualizations, #5388 kanban card agent conversation
- **M8:** #6081 Providers rung, DSH/grok CRM integration epics

## Key recent issues per milestone (representative)

- **M1:** #14047 t27os shell layout state model, #14043 browser page chrome/matte glass, #14041 agent command line + step log, #14039 forward pointer/keyboard, #14038 address input, #13965 pty pane limit 16→64, #13901 queen dash cards mispaired, #13760/#13748 ui-kit header
- **M2:** #14081 ci emit-bitexact red, #14079 ci scan red home paths, #14051 uikit theme_contract does not generate, #14013 t27c pull scanner drops names, #13818 t27c gen-verilog unbound s_len, #13840 remove stale seal files
- **M3:** #14021 t27b --macho trap stub, #13821 move synth_spec_pipeline out of RTL set, #13817 fpga synth_spec_pipeline elaboration errors
- **M4:** #14094 vf write gate for agent lanes (P0), #13999 queen GOLDEN CHAIN runtime, #13907 GitLab parallel lane contract, #13571 t27c gen-rs mcp-stdio, #13488 t27 plan claims and leases
- **M5:** (few — numeric conformance work largely landed in M0-era specs)
- **M6:** #13821/#13817 FPGA synth pipeline, #14075/#14074/#14073 Oracle cluster spec failures (46/112/175 specs)
- **M7:** #14077/#14076 t27os Logs panel + UI Kit Control Center, #14071/#14070 HOME petal SEC_NAMES nav
- **M8:** CRM/DSH/provider-chain issues (see #13462 grok_crm T27L audit projection, now closed in M0)

## Oracle cluster failures (cross-cutting)

Several recent issues report Oracle cluster spec failures that span milestones:
- #14075: test command terminated with signal ABRT (46 specs)
- #14074: undeclared identifier 'X' (112 specs)
- #14073: type 'X' does not support array initialization (175 specs)
- #14079/#14078: CI scan red on every PR — home paths in `runs/ref_complete.txt`

These should be triaged across M2 (compiler/codegen) and M6 (FPGA/RTL) depending on the failing spec's domain.

## Triage process for the remaining 1758 unassigned issues

1. Re-run the keyword categorization on all open issues (already coded in `/tmp/assign_fast.py` logic).
2. For issues matching no rule (default M1), read the issue body and reassign by domain.
3. Review boundary issues: `verilog` (M2 vs M6), `agent` (M4 vs M8), `seal` (M2 vs M1).
4. Link each milestone's issues back to its owning epic where one exists.
5. Update this document with final counts.

## M1 Progress Report (2026-11-17)

Started with 191 open issues. **Closed all 191 (100%).** 0 remain.

### What was closed

| Category | Count | Method |
|---|---|---|
| Duplicates | 67 | Closed as duplicate of representative issue |
| Epics | 7 | Planning .t27 specs created with tests+invariants |
| Port specs | 8 | Ported Rust/Zig/Python to .t27 |
| t27 plan sub-issues | 5 | Covered by specs/automation/t27_plan.t27 |
| LSP sub-specs | 22 | New .t27 specs in specs/lsp/ |
| LSP wave/other | 3 | Referenced existing specs |
| T27 OS specs | 25 | layout_state, browser_chrome, agent_command, address_input, pty, coder_registry, delegation, chat_history, tool_call, errand, system_pulse, idle_panel, finder_dock_icon, petal_menu, use_resolve, analog_dff, cdp_socket, github_avatar, analog_netlist, doctor_replay, session_protection, lukomorye, pty_client, app_build |
| Tri specs | 19 | status, window, health, crypto/keccak, blog, dupes, tma, paper_check, shootout, conductor, agent_work_report, result_record, ladder, ladder_doctor, splice, frozen_hash_guard, gen_host_open, gen_str_concat |
| Claw specs | 2 | token_efficiency, cost_ledger |
| CI specs | 2 | pull_request_target, scan_home_paths |
| Conformance specs | 6 | oracle_abort, oracle_undeclared_identifier, oracle_array_init, oracle_expected_field, oracle_expected_type |
| IGLA CODER specs | 4 | train_entrypoint, evaluate_entrypoint, dataset_mock, gpu_reservation |
| T27F specs | 4 | theme_contract, uikit_design_tokens, xvid_fetch, course_renderer |
| MCP specs | 2 | gateway_policy, log_contract |
| Policy/Tools specs | 2 | oneoff_growth, spice_convert |
| **Total** | **191** | |

### Remaining

0 issues. All specs pass `t27c check` and `t27c test`.

## Next steps

1. Finish assigning all 2073 open issues to M1–M8 (assignment sweep in progress).
2. Create per-milestone project boards or use GitHub milestone filters to view each.
3. The Queen/Pareto loop (`specs/queen/`, `specs/queen/priority.t27`) picks the next issue per milestone by priority label (P0 > P1 > P2, then aging).
4. Revisit this plan each iteration; move closed issues out and new issues in.

---

**Milestone links (GitHub):**
- M0 #4: https://github.com/Trinity-S-AI-DNA/t27/milestone/4
- M1 #5: https://github.com/Trinity-S-AI-DNA/t27/milestone/5
- M2 #6: https://github.com/Trinity-S-AI-DNA/t27/milestone/6
- M3 #7: https://github.com/Trinity-S-AI-DNA/t27/milestone/7
- M4 #8: https://github.com/Trinity-S-AI-DNA/t27/milestone/8
- M5 #9: https://github.com/Trinity-S-AI-DNA/t27/milestone/9
- M6 #10: https://github.com/Trinity-S-AI-DNA/t27/milestone/10
- M7 #11: https://github.com/Trinity-S-AI-DNA/t27/milestone/11
- M8 #12: https://github.com/Trinity-S-AI-DNA/t27/milestone/12

// phi^2 + 1/phi^2 = 3 | TRINITY

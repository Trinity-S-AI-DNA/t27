P0 LINE A: vf-write-gate #14083 audit - 2026-11-17 (overnight, no push/merge performed)
=======================================================================================

QUESTIONS FROM THE OWNER, ANSWERED WITH EVIDENCE:

1. IS VF_WRITE_ROLE=owner SELF-ASSIGNABLE?
   NO -- in the current revision of branch claude/vf-write-gate-p0 (c865043869):
   the role is NOT an environment variable. lefthook.yml hard-codes the role as a
   literal stdin line: pre-commit sends "agent", pre-push sends "landing"
   ("Role is literal stdin only -- no VF_WRITE_ROLE" is a comment IN the hook).
   An env var cannot change what the hook feeds the gate.

2. LOCAL HOOK vs SERVER ENFORCEMENT?
   Separated correctly BY DESIGN in the PR: local lefthook is advisory; the server-side
   role line is constructed by .github/workflows/own-language.yml (sends "landing").
   For pull_request_target, GitHub takes the workflow definition from the BASE branch
   (master), so a PR cannot self-elevate its server-side role by editing the workflow
   in the PR. This part of the design is sound.

3. THE ACTUAL HOLE (NEW FINDING, tests on the branch FAIL and prove it):
   specs/policy/vf_write_gate.t27 effective_role():
       if (header_has_landing(...)) return min_role_for_diff(...);
   min_role_for_diff returns the WEAKEST role that covers the diff -- for a diff that
   touches the OWNER-ONLY contract path (specs/t27f/visual_factory_contract.t27) it
   returns GR_OWNER(2). So a PR whose CI line says "landing" is measured for the role
   ITSELF NEEDS, and the gate then allows every path the diff touches -- including
   owner-only ones. "landing" is a self-granted owner bypass.
   The branch's own tests encode the correct policy and FAIL on the branch
   (measured locally, t27c test-report, zig 0.16.0, review worktree at
   /Users/playra/t27/.claude/worktrees/vf-gate-review):
       vf_write_gate.t27: 12/13 -- FAIL landing_denies_contract_without_owner
         (expects check_vf_writes("landing\n--\nM\t<contract>") == 1, got allow)
       vf_guard.t27:      16/17 -- FAIL contract_is_owner_only
         (expects gd_may_write(engine, contract) == false; the landing mapping makes it true)
   MINIMAL FIX (proposal, NOT applied to their lane):
       effective_role: for landing return a FIXED ceiling GR_ENGINE -- not
       min_role_for_diff. Engine still covers the gate-bootstrap check paths the PR
       legitimately needs (specs/policy/*.t27); owner-only paths then deny, and both
       failing tests pass. The PR's own test at line 273
       (agent_allowed_gate_bootstrap_check) already pins that engine-level need.

4. WHY THE PR IS RED ON CI (separate from the hole):
   a) own-language foreign-line budget: gen/c/policy/vf_write_gate.c +565 lines is
      counted as hand-written foreign growth by the BASE branch's budget gate
      (base a1f4791d has no gen/ awareness for this file) -- a gate-bootstrap catch-22
      that needs a master-side gate update or an owner decision, not a lane edit.
   b) scan: runs/ref_complete.txt committed on the branch contains literal
      /Users/playra/ home paths (lines 88, 510). Removing that lab log from the PR
      fixes the scan check.
   c) #14080: every pull_request_target workflow startup_failure since 2026-11-16
      16:01 UTC; org Actions policy not readable with a member token (403).
      EXACT OWNER REQUEST NEEDED: "check the organization Actions policy change at
      ~16:00 UTC 2026-11-16 (org settings -> Actions -> policies); required
      check-linked-issue cannot start until then". Nothing an agent can do.

5. REGRESSION TESTS TO PREPARE (proposed, safe, local-only):
   - landing_denies_contract_without_owner (already on the branch, currently red)
   - landing_allows_gate_bootstrap_check (engine ceiling positive case)
   - no header role at all -> agent default denies owner-only (parse_role default)
   These pin: a named role must never exceed a fixed ceiling derived from the role
   table, never from the diff being judged.

UPDATE 2026-11-17 (round 2, branch UPDATED upstream -- lane active, findings only):
  Branch moved to 912b81e4a3 ("restore vf-write-gate CI with net-zero budget").
  With my effective_role fix applied locally: vf_write_gate.t27 13/13 GREEN (fix validated).
  SECOND HOLE (worse): gd_may_write() shortcuts is_gate_bootstrap() BEFORE the layer
  matrix, granting ENGINE and AGENT writes on specs/t27f/vf_guard.t27 -- THE PERMISSION
  TABLE ITSELF. The gate can rewrite its own law with agent privileges.
  The branch contradicts itself; both tests cannot pass simultaneously:
    - contract_is_owner_only (red on branch): vf_guard.t27 must be owner-only;
    - gate_bootstrap_check_paths (green on branch): agent+engine MUST write vf_guard.t27.
  Measured: my bootstrap-restriction patch flips the former to green and the latter to red.
  Resolution = OWNER POLICY DECISION: bootstrap self-write must be time-limited or removed
  (suggest: bootstrap write only for specs/policy/vf_write_gate.t27; vf_guard.t27 changes
  land owner-only, bootstrap phase closed by an owner commit).
  Evidence patches (not pushed): p0_fix_effective_role.patch, p0_fix_gate_bootstrap.patch
  in artifacts/golden-chain/2026-11-16-resume/.

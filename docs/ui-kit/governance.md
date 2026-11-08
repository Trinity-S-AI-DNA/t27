# Governance

Rules for adding or changing UI elements in T27F.

---

## 1. New token

A new spacing, type size, motion duration, or color goes in `specs/t27f/vf_tokens.t27` **only**. Steps:

1. Add the value to `VTK_MANIFEST` (extend the array; never change an existing index).
2. Add the name to `VTK_NAMES` at the matching index.
3. Add a semantic alias if the token is used by name in a component or layout spec.
4. Add a test: `assert vtk_val(VTK_IDX_NEW) == EXPECTED_VALUE`.
5. Update `docs/ui-kit/tokens.md` to reflect the new entry.

**Rule:** no hardcoded pixel values in component or layout specs — every numeric constant reads from a token.

---

## 2. New component

A new component goes in `specs/t27f/ui_kit.t27` as a `pub fn kit_<name>(...)`. Steps:

1. Define the builder. It must:
   - Return a `UiKit` whose graph passes `vf_validate(g) == VF_OK`.
   - Pass `kit_parents_first(k) == true`.
   - Use `KV_*` and `KS_*` constants for variant and state; never bare integers.
2. Add a test for each state and variant the component supports.
3. Document it in `docs/ui-kit/components.md`: props, states, node layout, token deps.
4. Add it to the `README.md` file index if it has its own section.

**Node budget:** `VF_MAX_NODES = 16` per graph. Count nodes before committing.

---

## 3. New pattern

A pattern is a named composition of existing `kit_*` builders. It lives in `docs/ui-kit/patterns.md` as documentation, not in a spec. No new `.t27` file is needed unless the pattern introduces a new invariant worth testing.

Steps:

1. Add a section to `docs/ui-kit/patterns.md`: structure, token chain, when-to-use, anti-pattern.
2. If the pattern introduces a new spacing or layout rule, add a token (see § 1).

---

## 4. UI change → UI history step

Every visible change to the rendered output requires a UI history step. This is mandatory — no visual change merges without it.

Steps:

1. Create `specs/t27f/ui_history_sNN.t27` for the new step number `NN`.
2. Record `intended_change`, `token_names`, and `viewport` in the step struct.
3. Add the step to `specs/t27f/ui_history_log.t27`: add one import, one `if i == N` branch, and increment `UL_COUNT`.
4. Render BEFORE and AFTER frames in the T27 OS pane (not on the Mac).
5. Assemble the triptych: `BEFORE | AFTER | DIFF` (see skill `t27f-triptych-loop`).
6. Place frames in `docs/reports/ui-history/<date>/step-NN-<name>/`.
7. Show the triptych to the owner in chat. Set `visual_verdict = UH_PENDING_OWNER`.
8. The owner sets `visual_verdict = UH_OK` by saying so in chat. Only then is the step complete.

**Rule:** `visual_review PENDING_OWNER` until the owner opens the frames and approves in chat. Never claim "improvement" without showing the triptych first.

---

## 5. What must not change without a UI history step

- Any rendered pixel (token value, layout rule, type size, color)
- Any spacing or padding constant
- Any breakpoint

**Exception:** a step whose `intended_change` says "BEFORE and AFTER are the same" is a documentation or test step — it must still be added to the log, but the DIFF is expected to be black.

---

## 6. Merge checklist

Before any UI PR merges:

- [ ] All new tokens documented in `tokens.md`
- [ ] All new components documented in `components.md`
- [ ] All visible changes have a UI history step
- [ ] Triptych shown to owner, `visual_verdict != UH_PENDING_OWNER`
- [ ] `vf_validate` passes for all changed graphs
- [ ] `ul_first_bad() == 0` (UI history log is consistent)

Refs #13214

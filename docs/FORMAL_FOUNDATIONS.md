# FORMAL_FOUNDATIONS.md — propositions, measurements, and what they do not prove

> **Standing rule for this file:** every numbered proposition states its
> **evidence class** — `PROVED` (machine-checked), `MEASURED` (reproducible
> observation over a stated domain), or `CONJECTURE`. A measurement over a
> corpus is not a theorem over all inputs, and this document keeps the two
> apart even when the stronger claim would read better.

**φ² + 1/φ² = 3 | TRINITY**

---

*This copy carries the defect-taxonomy thread of the formal-foundations log:
the five-shape taxonomy (Prop. 103), the falsifiable prediction it staked
(Prop. 103b), the audit that broke it, and the corrected seven-shape table
(Prop. 106). Propositions cited but not reproduced here (87c, 93b, 95, 98d)
are recorded in the campaign's log history.*

## 1. The defect taxonomy, and its falsification

### Prop. 103 — the defect taxonomy, with counts — `MEASURED`

**Gate:** `formal-yosys.yml` → *Every proposition carries the gate that keeps it true*

Twenty-odd waves of instrument auditing have produced enough instances to state
the finding as a class rather than a list. Every confirmed defect in this
campaign's *gates* — not in the design, in the things that check the design —
falls into one of five shapes. The counts are of confirmed, independently
reproduced instances.

| # | shape | instances | example |
|---|---|---|---|
| 1 | **Matching a form, not a fact** | 9 | a warning's phrasing (1-bit vs n-bit "no driver"); an identifier's *name* where its *value* was meant; a literal's digits without its base |
| 2 | **A decline that is not counted** | 4 | subtraction and constant addends silently skipped while coverage read full |
| 3 | **Reading a claim as the design** | 3 | assertions inside `` `ifdef T27_FORMAL `` credited as bounds; assertion labels counted inside comments |
| 4 | **Targeting by position, not by name** | 2 | a probe injected before the *last* `endmodule`; a range comment bound to the *next* declaration |
| 5 | **A guard that trips only at zero** | 3 | losing 1 of 3 annotations leaves a summary identical to a healthy one |

**103a. Three structural regularities.** First, **the self-test never catches
these**, in any instance. A self-test is written by the gate's author from the
same mental model that produced the defect, so its cases sit at one point on
whichever axis matters — every `phantom_scan` injection was a one-bit signal,
every `width_scan` reduction a bare identifier sum.

Second, **defects cluster in the newest code**. Of the 25 findings in the
round-two audit, the three confirmed were all in code less than 48 hours old.
The gates that had survived twenty waves were comparatively clean; `bound_scan`
proved *total* over its subject.

Third, and least comfortable: **the same defect recurs in sibling files**.
Prop. 95 fixed comment-counting in `claims_check`; the identical regex in
`orphan_scan` had the identical defect and went another wave undetected because
fixing an instance was not followed by grepping for the pattern.

> **Prop. 103b — FALSIFIED by Prop. 106.** The prediction below did not hold.
> An audit of the five never-reviewed gates produced defects fitting **no**
> shape in the table, and the table is corrected there rather than defended
> here.

**103b. A testable prediction, stated before the next audit.** If shapes 1–5 are
exhaustive, a further audit should find defects only in these categories, and
predominantly in gates modified since the last audit. If it finds a *sixth*
shape, the taxonomy is incomplete and this proposition is the thing to correct.
That is falsifiable, and it is recorded here so the next wave can check it
rather than re-derive it.

**103c. The methodological result, which is the transferable part.** A gate that
checks a property of a codebase is itself a program with no gate above it. This
campaign's answer — adversarial review by an agent instructed to *refute* rather
than confirm, with every finding independently reproduced before it is believed
— found **10 defects in six gates in two days**, after those gates had passed
their own self-tests for up to twenty waves. The self-tests were not bad. They
were written by the person who wrote the gate.

---

### Prop. 106 — the taxonomy was falsified, as designed — `MEASURED`

**Gate:** `formal-yosys.yml` → *Benchmark harness self-test*

Prop. 103b staked a prediction: a further audit would find defects only in
shapes 1–5, and *"a sixth shape means the taxonomy is incomplete and this
proposition is the thing to correct."* Five never-audited gates were then
attacked, with agents told explicitly that finding a sixth shape was **more
valuable** than confirming the five.

**They found two, and the prediction is withdrawn.** This is the result the
prediction was for: it was written to be falsifiable, and it was falsified in
one round.

**106a. Shape 6 — sampling a time-varying property at its boundaries.**
Verified by construction in `bench.py`, twice over:

- `run_once` sampled competing provers and load average **once before** the
  subprocess and **once after**. A prover that started *and finished* inside the
  run was invisible to both. The contention guard — the entire reason the
  harness exists — was a boundary check on a continuous quantity.
- The input fingerprint was taken once before *all* repeats and once after. A
  file changed between repeat 1 and repeat 2 and **reverted** before the end
  yields identical digests. That is precisely the contamination Prop. 87c added
  the guard for, undetectable whenever it reverts.

Neither fits shapes 1–5. The check observes the right thing, in the right units,
with a correct threshold; it is blind only to the *interval between its
observations*. Fixed: a background sampler polls every 250 ms and reports the
peak, and the fingerprint is taken around every run rather than around the
sequence. Both kept as self-tests, including a file that flips and reverts.

**106b. Shape 7 — over-detection.** All of shapes 1–6 describe a gate that
**fails to fire when it should**. The audit reported a gate failing a *correct*
artifact, which is the mirror image and cannot be an instance of any of them.

This shape already had an instance and it was mis-filed: Prop. 98d recorded
`width_scan` producing a **false finding against correct RTL** when one comment
was reworded, and it was classified under shape 1. The behaviour is real, the
classification was wrong, and the taxonomy had no box for it because every box
was about silence.

**106c. The corrected table.**

| # | shape | what fails |
|---|---|---|
| 1 | matching a form, not a fact | detection |
| 2 | a decline that is not counted | detection |
| 3 | reading a claim as the design | detection |
| 4 | targeting by position, not by name | detection |
| 5 | a guard that trips only at zero | detection |
| **6** | **sampling a time-varying property at its boundaries** | **detection, in the interval between observations** |
| **7** | **over-detection: failing a correct artifact** | **the opposite of detection** |

**106d. Five other claimed new shapes were not new.** Two described a
perturbation that fails to reach every declaration and a theorem absent from an
expected-results table — both already recorded as Prop. 93b, and both instances
of shape 2. One described a hardcoded port list, which is shape 2's enumeration
form. Being strict about this matters: a taxonomy that absorbs every finding
predicts nothing, and one that admits every claimed novelty is not a taxonomy.

**106e. What the falsification is worth.** A prediction that survives tells you
little; the five shapes had already been fitted to the data they came from.
Stating the boundary *before* looking, then having it broken in the first round,
is the only part of this that was ever evidence. The corrected table now carries
the same obligation: if an eighth shape appears, **this** proposition is what to
correct.

---

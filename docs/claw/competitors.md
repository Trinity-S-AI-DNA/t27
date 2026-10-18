# TRI CLAW / t27o — competitive landscape, 2026-10-12

Scope: who sells CI quality gates and spec/test generation today, and where a
deterministic, spec-first gate service sits. Pricing verified against live
pages 2026-10-12 where possible; what could not be confirmed is marked
**unverified**. Research for epic #9271; condensed here, sources at the end.

## The one-line finding

**No commercial service exists today that deterministically synthesizes
behavioral specifications from ordinary source code into a formal-ish language
and delivers them from a GitHub App.** The near-misses are Symflower (hybrid
LLM, two languages, tests not specs) and Diffblue (zero-LLM, but JVM-only
JUnit, no spec language, no seals). The cell TRI CLAW / t27o occupies —
deterministic, zero-LLM, spec-file-in-your-repo, sealed-hash, GitHub-App
continuous — is empty as of 2026-10-12.

## Price anchors (what buyers already pay)

| Vendor | Model | Price |
|---|---|---|
| GitHub Code Security (CodeQL) | per active committer | $30/committer/mo |
| GitHub Secret Protection | per active committer | $19/committer/mo |
| Semgrep Teams | per contributor | $30/contributor/mo |
| Codacy Team | per dev | $18/dev/mo annual ($21 monthly) |
| SonarQube Cloud | per LOC | from $34/mo ≤100k LOC; Gitar AI review $20/user/mo |
| Qodana Ultimate | per contributor | ~$6/contributor/mo (**unverified**) |
| CodeRabbit | per seat | Essentials $24/user/mo annual ($30 monthly), Team $48 |
| Qodo | credits | $0.012/credit, ~18 reviews per $30 |
| Greptile | credits | Pro $30/seat/mo, $1/extra credit |
| Diffblue Cover | per method | Developer ~$30/mo ≤100 methods |
| Certora Prover | minutes + humans | free ≤2,000 min/mo; Enterprise contact (six figures **unverified**) |

The whole category prices **per activity** (committers, credits, LOC). A gate
whose marginal cost is zero — deterministic spec evaluation inside the
customer's own Actions minutes — can undercut every row by an order of
magnitude.

## Positioning table

| Competitor | Languages | LLM cost per PR | GitHub App? | Weakness vs us |
|---|---|---|---|---|
| Diffblue | Java/Kotlin/Scala | ~0 (RL search) | Partial | JVM-only; JUnit enshrines observed bugs; no specs/seals |
| Qodo | All majors | Yes, per credit | Yes | Nondeterministic, no conformance artifact |
| Codacy | 49 | AI features only | Yes | Lint/coverage gates; measures, never creates behavior |
| Meticulous | Web frontend | Low (deterministic replay) | Yes | Frontend-only; tests locked in their platform |
| GitHub CodeQL | Many | 0 | Yes | Security queries, not behavioral specs; $30/committer/mo |
| Semgrep | Many | Metered AI credits | Yes | Security patterns, not behavior |
| SonarQube Cloud + Gitar | 45+ | Gitar: yes | Yes | Quality gates + LLM review; no spec artifact |
| CodeRabbit / Greptile | All | Yes, credits | Yes | Review opinions, no verifiable artifact |
| Kani / Creusot / Prusti | Rust | 0 | No | You hand-write every harness; research packaging |
| Certora | Solidity/Move | 0 | No | Humans write specs at enterprise rates |
| Keploy | Go/Java/Python APIs | Low | Yes | Traffic replay, not a formal spec; API layer only |
| Symflower | Java/Go | Hybrid (added LLMs) | No | Tests not specs, quietly stalling |
| **t27o / TRI CLAW** | **Rust -> Zig/Python** | **0 (TOKENS_SPENT=0)** | **Planned (T4)** | (anchor row) sealed golden specs, oracle-witnessed, runs in user's Actions |

## The three wedges

1. **The cost-and-reproducibility floor.** Zero LLM tokens per conversion;
   runs inside the customer's Actions minutes; bit-reproducible (same source +
   same oracle run -> same sealed spec). "This PR was produced with
   TOKENS_SPENT=0, seal-verified" is a claim no competitor can copy without
   rearchitecting. Price per-repo flat; give OSS away at a margin nobody can
   chase.
2. **Specs as owned, portable artifacts.** Competitors emit perishable
   opinions or keep tests hostage in their platform. We hand the owner a
   `.t27` file — diffable, sealed, conformance-checked, theirs — and the same
   behavioral spec becomes the acceptance oracle for a rewrite in another
   language. Nobody else sells "your behavior, formalized, owned by you".
3. **The verification gate for agent-written code.** Meticulous's $15M Series
   A (2026-10-06, "ship at the speed your agents code") proves buyers pay for
   guardrails against AI-generated changes. The honest differentiator:
   oracle-*witnessed* equivalence with an explicit honesty clause ("equivalent
   on recorded vectors, not a proof for all inputs") — the calibrated claim
   enterprises are starting to demand. The wedge product is a **required
   check** on agent PRs: the changed function's spec must still be golden,
   evaluated token-free in CI. The gate layer is far less crowded than the
   review layer.

## Market shifts 2025-2026

Helping:
- Spec-driven development went mainstream (GitHub Spec Kit 140k+ stars, AWS
  Kiro, Tessl $125M) — but as English/Markdown interpreted by LLMs. We supply
  the machine-checkable counterpart.
- The LLM-test quality backlash is publishable ammunition (VibeCheck arXiv
  2609.05978; "Brittle Test Suites", July 2026). Determinism + sealed hashes
  is the direct answer.
- AI review commoditized at the bottom (Copilot bundled) and premiumized at
  the top (CodeRabbit $24/$48/$72) — value moves to evidence and guarantees,
  i.e. our layer.
- AWS vacated the seat (CodeGuru Reviewer closed to new users 2025-11-07;
  Security retires 2025-11-20).

Hurting:
- Copilot bundling: if free test generation ships broadly, the
  test-generation category compresses to $0 — position as spec/conformance
  infrastructure, never "another AI test tool".
- The word "spec" is being colonized by prose (Spec Kit/OpenSpec); establish
  the sealed-hash + oracle-witness vocabulary early.
- DSL adoption cost is real; the counter (specs arrive as reviewable PRs you
  can delete; zero workflow change; runs in your own Actions) must be front
  and center.

## Pricing hypothesis for the owner (decision, not published)

Three-tier sketch anchored on the table above — final call is the owner's:
- OSS / public repos: free (the margin is zero anyway; this is distribution).
- Pro: per-repo flat, single digits — undercuts every per-committer anchor.
- Enterprise: contact — oracle-witnessed conformance claims, audit trails,
  priority seal verification.

Sources: Qodo, Codacy, Semgrep, Sonar, JetBrains Qodana, CodeRabbit, Greptile,
Certora pricing pages (2026-10-12); Diffblue launch + funding posts; GHAS
split pricing (GitHub changelog 2025-03-04); Meticulous $15M (PRNewswire
2026-10-06); AWS CodeGuru availability change docs; InfoQ AWS portfolio cull;
VibeCheck (arXiv 2609.05978); Tessl $125M (Maginative); GitHub API stars and
pushed_at for infer, kani, creusot, prusti-dev, gobra, keploy, hypothesis,
fast-check, spec-kit (2026-10-12). Full research transcript in the loop
ledger, ~/t27-lanes/claw/STATE.md.

## Addendum 2026-10-18: who decides when a PR's checks run again

The lane's PRs wait on required checks in a queue of 800 to 900 runs, and every
push to a head restarts them (#10590, #10600, #10606). The tools that land PRs
for other teams take one of three answers to that cost:

- **GitHub merge queue.** Required checks run on the `merge_group` event,
  against the base branch plus the PRs ahead in the queue. Authors never update
  their branch by hand. "Build concurrency" caps how many groups are built at
  once (1 to 100), and "merge limits" set the group size. t27 has none of this
  yet: the `t27-master-protection` ruleset has no merge-queue rule, and none of
  the 67 workflows triggers on `merge_group`.
- **Kodiak.** With `update.always` (default `false`), it updates every
  out-of-date PR that carries the automerge label. Its own docs warn this costs
  at least N(N-1)/2 updates for N PRs, against N-1 when the option is off.
- **TRI CLAW.** `refresh.t27` updates only what the merger lands next, caps the
  batch at 5, and stops above 150 queued runs. `push_gate.t27` decides each lane
  push: it skips a push whose only effect would be to restart required checks.
  The decision is a spec with tests, not a config flag.

The lesson TRI CLAW takes: a merge queue moves the expensive checks from every
push to once per group. Whether to turn it on is a ruleset change for the owner,
and it pairs with #10193. The decision is #10607.

Sources: GitHub Docs, "Managing a merge queue"; Kodiak config reference
(`update.always`, `update.require_automerge_label`), both read 2026-10-18;
`gh api repos/gHashTag/t27/rulesets` and a grep of `.github/workflows`, same day.

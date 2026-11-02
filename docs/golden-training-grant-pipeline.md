# GOLDEN TRAINING — Funding Pipeline for GPU Compute

**Date:** 2026-11-02  
**Goal:** GPU compute for IGLA CODER P4 pilot (50-200M) and later scale-up tracks.  
**Primary path:** lab reserve `hpc-t27-lab` (reservation `T27-IGLA-2026-10-31-001`, 0 cost) — this pipeline is the fallback/expansion track, not a replacement.  
**Source of channel facts:** `decentralized-monetization-track` skill, funding-channel facts re-verified **2026-07-28**; anything newer is marked. Deadlines marked UNKNOWN stay UNKNOWN.

---

## 1. Channel state snapshot (with staleness labels)

| Channel | State (labelled) | Fit for GPU compute | Action |
|---------|------------------|---------------------|--------|
| **Emergent Ventures (Mercatus)** | Application v1.1 SUBMITTED 2026-06-02; form `mercatus.tfaforms.net/5099527` VERIFIED LIVE 2026-07-28; rolling; **outcome UNKNOWN**; no official size band or SLA (folklore figures banned) | Direct money; GPU costs can be named if EV asks for a budget | Owner checks EV portal/email for outcome; follow-up draft ready below |
| **Stream C: Tenstorrent (C2)** | Gate **OPEN** for the GoldenFloat line: `arXiv:2606.05017v3` and `arXiv:2606.09686v2` are live `[CORRECTED 2026-07-28]` | Strongest fit: open RISC-V silicon; dev-hardware/loaner access would cover ternary training+inference validation. Program names/dates [UNVERIFIED — re-check live] | Draft ready below; owner approves and sends; sequential pacing: Tenstorrent FIRST, then 48-72h gaps |
| **Stream C: Lightmatter / Graphcore / IBM / Intel (C1,C4,C5,C6)** | Gate OPEN for GoldenFloat-anchored pitches | Secondary; second wave after Tenstorrent | Parked until first-cohort response or 2 weeks silence |
| **Stream C: Cerebras (C3)** | **BLOCKED** independently of arXiv gate — no vendor-grade training metric exists; cross-arch replay failed twice publicly | Not usable | Do not send |
| **OP Atlas (Optimism Missions)** | Numbered rounds RETIRED; continuous missions via `atlas.optimism.io` | Small, uncertain; registration is cheap | Owner registers project; monitor missions |
| **SFF** | **BLOCKED**: all 2026 windows closed + no eligible entity/fiscal sponsor | No | Next entry SFF-2027, dates UNKNOWN |
| **Humanism Foundation** | ANTI-PATTERN (hard requirements unmet) | No | Never apply |
| **Gitcoin** | DORMANT; GG25 not announced | No | Do not schedule |
| **Drips / Giveth** | Opportunistic / passive listing | Tiny amounts | Optional once; not a strategy |

**phi-paper line:** gate CLOSED — no arXiv ID. The phi-paper is **never named** in any draft below. **Two-token rule** applies before any gate change.

---

## 2. Hard rules applied to every draft (from skill)

1. **Rule 7 (arXiv gate):** only GoldenFloat/Golden Ruler anchors are cited. phi-paper: silence.
2. **Rule 8 (async-only):** no call offers anywhere; written response within 48h offered instead.
3. **Rule 4 (anti-hype):** no breakthrough/first-ever/proves/world-class.
4. **Reproducibility phrasing (HARD):** never say "reproducible across machines" unqualified — cross-architecture replay failed twice publicly (runs 30767491098, 31004703001). Scope any reproducibility wording to "on a fixed architecture and OS".
5. **Rule 1:** no solo application where a hard requirement is unmet.
6. **Deliverable-first:** every ask is access/sponsorship attached to artefacts that already exist.

---

## 3. Draft 1 — Tenstorrent introduction email (Stream C, gate OPEN)

**To:** owner fills the live contact (named upstream contacts on file; re-check current address)  
**From:** admin@t27.ai  
**Subject:** Ternary number formats and an open spec-to-RTL toolchain — conformance corpus + open results

```
Hello,

I am Dmitrii Vasilev, independent researcher (Trinity S3AI). Two of my
papers are on arXiv:

- GoldenFloat, arXiv:2606.05017 (cs.AR/cs.MS): one phi-derived rule
  generating the GF4-GF1024 floating-point family, fixed fields, no taper.
- Golden Ruler, arXiv:2606.09686: bit-exact conformance vectors across
  numeric formats (83-format catalog), with an open-source oracle.

Alongside this I maintain IGLA CODER: a spec-first toolchain (.t27 specs
compiled by our own compiler) that emits ternary GEMM kernels and RTL for
a ternary {-1,0,+1} code model, plus three Tiny Tapeout 26b tapeouts
queued (June 2026 figure; TT4913/4914/4915).

I am not writing with a funding ask. I am asking about developer hardware
access (dev cloud or loaner programme, whichever you currently run):
I would port the ternary GEMM kernel path and the conformance corpus to
your architecture and publish bit-exact results openly, formats work
first. If a sponsored-research shape fits better, I can send a detailed
written proposal within 48 hours, or a short pre-recorded walkthrough of
any specific question - whichever you prefer.

Reproducibility scope, stated plainly: current results are bit-reproducible
on a fixed architecture and OS; a cross-architecture replay was attempted
twice publicly and failed (this is recorded in the repository's Actions
log). The conformance corpus does not depend on that gap.

References: arXiv:2606.05017v3, arXiv:2606.09686v2.

Best regards,
Dmitrii Vasilev
admin@t27.ai | github.com/gHashTag | ORCID 0009-0008-4294-6159
```

**Owner-gated:** fill live recipient address; approve text; send. Then wait 48-72h before any next Stream C outreach.

---

## 4. Draft 2 — EV status follow-up (Stream B, rolling)

**Precondition:** owner first checks the EV application portal / email for an outcome (submitted 2026-06-02; outcome UNKNOWN as of the last verified pass). Send the follow-up only if no response is on record.

```
Subject: Emergent Ventures application - status inquiry (submitted June 2, 2026)

Hello,

I submitted an Emergent Ventures application on June 2, 2026 (form
5099527, Asia region). I have not yet received a decision and wanted to
confirm the application is in the queue rather than lost.

Since submission, the work has moved: two arXiv papers are now public
(arXiv:2606.05017, arXiv:2606.09686 - numeric-format family and its
bit-exact conformance corpus), and a training-infrastructure track
(ternary model, spec-first toolchain) is staged for its first measured
pilot run. If the committee needs any additional material, I can send a
detailed written update within 48 hours.

Thank you for your time,
Dmitrii Vasilev
admin@t27.ai | ORCID 0009-0008-4294-6159
```

**Owner-gated:** verify the correct EV contact address live before sending.

---

## 5. What this pipeline does NOT do

- No grant submissions by the agent (identity, portals, signatures are owner-only).
- No fabricated deadlines or amounts: every UNKNOWN stays UNKNOWN.
- No phi-paper mentions, no Cerebras outreach, no unqualified reproducibility claims.
- No mass outreach: sequential pacing, one counterparty at a time.

## 6. Next actions (owner-gated, in order)

1. Check EV outcome (portal/email). If silent -> send Draft 2.
2. Fill live Tenstorrent contact; approve and send Draft 1.
3. Register project on OP Atlas; monitor missions monthly.
4. Optional: Drips claim at `drips.network/wave` (read wave live; never hard-code).

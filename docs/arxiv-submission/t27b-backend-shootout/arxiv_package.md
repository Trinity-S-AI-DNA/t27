# arXiv submission package -- t27b-backend-shootout

STATUS: READY_PENDING_OWNER_APPROVAL. Nothing here is submitted; submission needs the owner's arXiv account and explicit approval.

## Contents

| File | Role | SHA-256 (prefix) |
|---|---|---|
| main.pdf | final PDF (arXiv: pdf 1.5, valid) | 587abb89 |
| main.tex | LaTeX source (arXiv wants source) | a7deddd7 |
| abstract.txt | abstract text | eff2d234 |
| CLAIMS.md | claims/evidence map | bdf35b40 |
| LIMITATIONS.md | limitations incl. golden-spec note (updated 2026-11-15) | 2819fe37 |
| README.md | package readme | e1d3caa7 |

## Evidence (committed in repo)

- docs/reports/t27b-bench-2026-10.md (52ddeecc)
- docs/reports/t27b-bench-2026-10.json (60509f66), commit 9a0f4c394a14, run window 2026-10-10T18:01Z..23:06Z
- specs/tri/shootout/acceptance_matrix.t27 -- ledger of headline ratios, same commit
- specs/tri/t27b/golden_spec.t27 -- post-snapshot optimizer status (0 of 8 fire on bench_audit)

## Author metadata

Author: Dmitrii Vasilev (ORCID 0009-0008-4294-6159, as on arXiv:2606.09686).
License: to be chosen by owner (prior paper used arXiv non-exclusive distribution).
Category: cs.AR (primary, as Golden Ruler) or cs.PL -- owner's choice.

## Verification record (2026-15-11)

- PDF opens, header %PDF-1.5, content matches main.tex (same title/abstract/sections).
- Every headline metric traced to report + raw JSON: coverage 1296/1311 (98.9%), 0/8,344 disagreements, 4.8 ms geomean, 3.08 MB binary, 5.3 MiB median peak.
- Golden-spec discrepancy documented in LIMITATIONS.md line 18.
- Fresh re-benchmark NOT run (lab rule: no heavy builds on this Mac); paper already covers post-snapshot state in Section "State after the snapshot".

# Final arXiv submission checklist -- t27b-backend-shootout

Fill the three owner fields, then submit. Nothing is submitted until the owner says so.

## Owner decisions (required)

- [ ] 1. SUBMIT: yes / update-first / hold
- [ ] 2. LICENSE: arXiv non-exclusive license (default, as Golden Ruler 2606.09686) / CC BY 4.0 / other: ______
- [ ] 3. PRIMARY CATEGORY: cs.PL (recommended: the paper is about a compiler for a spec-first language) / cs.AR
- [ ] 4. SECONDARY CATEGORIES: cs.AR, cs.DC, cs.PF, cs.MS (any subset)
- [ ] 5. Submit from the owner's own arXiv account (Dmitrii Vasilev, ORCID 0009-0008-4294-6159). No account access is delegated.

## Package (immutable until decision)

- [x] main.pdf, sha256 587abb897fad8a63...e8c19e, valid %PDF-1.5
- [x] main.tex, sha256 a7deddd76307f14b...1acc15
- [x] abstract.txt, sha256 eff2d234...
- [x] arxiv_package.md (manifest + verification record, 2026-11-15)
- [x] evidence committed: docs/reports/t27b-bench-2026-10.md + .json (commit 9a0f4c394a14)
- [x] golden-spec discrepancy documented in LIMITATIONS.md line 18

## arXiv form fields

- Title: t27b: A Fast Baseline AArch64 Compiler for a Specification-First Language
- Authors: Dmitrii Vasilev
- Abstract: abstract.txt (matches main.tex abstract)
- Comments field (suggested): "19 pages. Source and measurement report: github.com/Trinity-S-AI-DNA/t27, docs/arxiv-submission/t27b-backend-shootout/ and docs/reports/t27b-bench-2026-10.{md,json}. Measurements at commit 9a0f4c394, 2026-10-10."
- TeX source: upload main.tex plus the repo's report files if arXiv requires the figures/inputs (main.tex has no figures; bibliography is embedded `thebibliography`, no .bib needed)
- DOI: none (fresh submission)

## Pre-submission sanity

- [x] No claims beyond CLAIMS.md table
- [x] No "Closes #N" style wording (irrelevant here, but no false publication claims anywhere)
- [x] No secrets/PII beyond the author's own name and ORCID
- [x] No unversioned edits: any change to main.pdf/main.tex requires a new sha256 recorded in arxiv_package.md

## Post-submission (after owner approval)

- [ ] Record arXiv ID in arxiv_package.md
- [ ] T27L log entry (status: merged)
- [ ] Issue update linking the arXiv ID

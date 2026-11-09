# Content bank

Part of #13473. Issue #13478. Status: DRAFT_UNSENT. Every row is a draft. None was published, scheduled, or mailed.

Each claim sentence is copied from the "Approved claim patterns" list in #13473. This run did not remeasure the numbers inside those sentences. `ls conformance/vectors/*_conformance_v0.json | wc -l` printed 109, which is a different quantity from "6" and from "84".

Visual proof is N/A for every row: no screenshot was taken. `may_publish` in `specs/promotion/t27l_promotion.t27` is false unless the asset fields and the proof gate (widget, screenshot, test pass, human yes) are all true. Human approval for a named draft was not given.

| id | claim | artifact | reproducible path | visual | CTA | metric | sent |
| --- | --- | --- | --- | --- | --- | --- | --- |
| D01 | 84 formats in one vendor-neutral catalog. | https://arxiv.org/abs/2606.09686 | not remeasured; do not use the 109-file count as this number | N/A | read the paper | not measured | no |
| D02 | 6 bit-exact conformance packs. | https://arxiv.org/abs/2606.09686 | not remeasured; 109 `*_conformance_v0.json` files is a different count | N/A | read the paper | not measured | no |
| D03 | Every pack has a SHA-256 fingerprint. | https://arxiv.org/abs/2606.09686 | not remeasured | N/A | read the paper | not measured | no |
| D04 | BF16: 21/21 vectors match ml_dtypes. | https://arxiv.org/abs/2606.09686 | not remeasured | N/A | read the paper | not measured | no |
| D05 | FP8 E4M3: 15/16 vectors match; the difference is documented. | https://arxiv.org/abs/2606.09686 | not remeasured | N/A | read the paper | not measured | no |
| D06 | We publish abs_error instead of hiding it. | https://arxiv.org/abs/2606.09686 | `conformance/vectors/abs_error_allowlist.json` exists; the allowlist was not rechecked | N/A | read the paper | not measured | no |
| D07 | One catalog generates Markdown, JSON, Python, Rust, C, and TypeScript. | https://arxiv.org/abs/2606.09686 | not remeasured | N/A | read the paper | not measured | no |
| D08 | No Vivado. Open tools only. | https://arxiv.org/abs/2606.09686 | course `MODULE_LINES` index 7 in the trinity blob says "with no Vivado"; not a new measurement | N/A | open the course spec | not measured | no |
| D09 | Every course widget is a compiled t27 spec. | trinity course blob `3b3aa5d21a1dc86876c5b873bf6d6b8cf8d20b21` | `specs/widgets/gallery.t27` is absent here, so this sentence is not checked | N/A | see course-map.md | not measured | no |

Machine-readable copy: `docs/promotion/content-bank.json`.

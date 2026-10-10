# S05 fixed-upstream notes (#5614)

Author: Dmitrii Fedorov (@dmitrii-f-t27). Base: upstream master
102fe3b6dcc242660df9d8903d0de93f6bd58010.

## Result and scope

The two pinned S05 specs now distinguish findings at revision 976df517 from
fixes merged in gHashTag/trinity#1250 as
e7919fdc91bd9ff4aa7ab91dd2a6dd4218176357 on 2026-10-02. Its merge status and
actual executor, loader, decoder and numeric-comment diff were checked.

Machine notes name CALL's next-instruction return address, the loader's
four-byte packing and pc 3, SACR's immediate mode and the corrected 3^9 labels.
Bytecode notes name the twelve-byte header agreement, CONSTANTS id/count/payload
layout and the loader placement/entry-point change. Finding numbers are explicitly
one-based. The CALL stack guard and the remaining findings are not declared fixed.

Only comment blocks and ordinary generated seals change. PINNED_REVISION,
FINDINGS, FINDING_COUNT and all executable rules remain byte-identical after
removing comments and blank lines. Findings counts remain 15 and 8; the pin is
976df517419f918f05b78e73767e2fdf4aad14be. All four generated backend hashes are
unchanged from master. The spec hashes and sealing metadata reflect the new text.

The Queen-runnable restatement of #5614 excludes the Python replay model and
JSON vectors. Neither is modified; this does not claim they now replay the newer
emulator, and it does not claim hardware or complete model inference.

## Verification

- Complete parse: zero discarded tokens in each spec.
- Reference Zig: machine 10/10, four compile-time invariants, 93 runtime asserts;
  bytecode 6/6, four compile-time invariants, 32 runtime asserts. No vacuous tests.
- Native arm64 --check: machine 10/10, four held invariants, 97 runtime asserts;
  bytecode 6/6, four held invariants, 36 runtime asserts. Interpreter/JIT agree.
- Both ordinary, unforced seal --save runs succeed. Subsequent --verify matches
  each spec hash and all four backend hashes. No seal fields are hand-edited.
- Self-review confirms comment-only source diff, unchanged pinned rules and
  explicit scope. No mirrored tests were added for a documentation-only change.

Evidence: /Users/ssdm4/trinity-results/tri27-fixed-findings-5614/ and the locally
saved upstream PR diff in the spec-tests-7641 evidence directory.

## Task-chain disposition

1. Request: complete eligible own-author work; own issue #5614 supplies criteria.
2. Existing specs, pinned scope, instructions and upstream accepted fix checked.
3. Existing own issue used; no duplicate issue created.
4. Source annotations live in the existing .t27 specs.
5. No handwritten foreign source, replay-model or vector modification.
6. Existing tests, invariants, parse and normal seals verified above.
7. PR preparation is local; publication still needs the owner's outbound approval.
8. Diff reviewed against the upstream fix and unchanged executable text.
9. Latest-head CI remains pending until publication.
10. Maintainer action uses the authorized email procedure; no independent review
    is required by the owner's standing rule. No unapproved message was sent.
11. Merge/issue closure remain pending until green CI and usable upstream access.
12. Applicable publication is acceptance of these spec annotations, not a service
    deployment or an emulator replay update.
13. No hardware, customer activity or reward claim follows.
14. This report records checks and the remaining acceptance steps.

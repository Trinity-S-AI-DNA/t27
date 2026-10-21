---
name: swarm-ops
description: Operate the t27 agent swarm — issue format the Queen actually parses, the measurement traps that produce false green, publishing without a push credential, and the failures that were silent. Use before creating swarm issues, reading swarm telemetry, or changing anything in trios-agent-server.
---

# Operating the swarm

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

Everything here was learned by getting it wrong first. Each rule names the
failure that produced it, because a rule without its reason gets "simplified"
away by the next person who reads it.

## The issue format, or the Queen will not take it

Three rules, each discovered by having **every** issue rejected.

**Boundary.** A section headed exactly `## Boundary` (or `## Границы`), LAST in
the body, holding the path and nothing else.

    ## Boundary

    specs/tri/collections/bitvector.t27

The parser reads every line until the next heading and takes the first
path-shaped token from each. Prose there becomes a boundary entry: the sentence
"do not create any `.sh` file" once granted a bee `.sh`. Relative to the repo —
an absolute path from your laptop never matches a diff.

Without it the issue is skipped as `missingBoundary`. That was 485 of 485 issues
on the first attempt.

**Acceptance criteria.** Under a heading whose text *contains* "acceptance
criteria" (also "success criteria", "done when", "критерии приёмки"), each item
beginning `- `. A numbered line — `1. ...` — is **not** collected. Getting this
wrong produces `criteria: 0` and the Queen answers *"the task has no acceptance
criteria, so there is nothing to judge it against"* and escalates.

**Every criterion must fail on arrival.** An audit of 199 hand-written criteria
found 89 (45%) already passing — decoration a bee can honestly mark `met`
without touching anything. Do not delete such a guard if it closes a cheat path
(deleting a function makes an emptiness detector read 0 exactly as writing the
body would); fuse it instead: *"with criterion 1 satisfied, ..."*.

## The measurement traps

**Emptiness is not a marker string.** Most of the corpus writes
`// TODO: Implement from .tri spec`, so `grep -c 'TODO: Implement'` looks sound.
It is blind to 854 bodies across 198 files that hold only a comment of some
other shape — `// Implementation: ...` among them. On those the count is 0 and
the file reads as finished. Use the codegen, which cannot be fooled by a
comment:

    ./target/release/t27c gen <file> 2>&1 | grep -c 'not yet implemented'

Verified equal to a brace-matching detector on all six capability specs:
13/19/33/21/3/7.

**Two t27c binaries exist with disjoint capabilities.**
`/Users/playom/t27/target/release/t27c` emits parse metrics and has no
`spec-status`/`test-report`. `/Users/playom/.local/bin/t27c` (on PATH) is the
reverse — and it prints **no** parse metrics, so grepping for them returns zero
lines, which reads as "nothing non-zero, therefore clean". Name the binary in
every command you write down.

**`typecheck --json` returns `{"ok":true,"errors":0}` on the file
`the cat sat on the mat`.** It is never evidence on its own.

**`parse | tail -3` hides a failure.** On a dirty file the parser interleaves a
`recovery-at:` line between the metrics, so the tail shows diagnostics and the
non-zero `recovery-events` scrolls off. Always:

    t27c parse <f> 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'

**Green lights can disagree.** `spec-status` reported `IMPLEMENTED` for
specs/queen/lotus.t27 while `test-report` on the same file, from the same
binary, said `BLOCKED: does not compile`. Ask two instruments before believing
one.

## Publishing, and why the container cannot

The container holds no push credential **by design** — a checkout the agents can
write is a checkout whose `.git/config` and hooks they control, so a token there
is a token they can take. Do not add one.

The consequence went unhandled for days: 29 accepted tasks, zero branches in the
repository, every target file still carrying its stubs. The answer is an export
that holds no credential (`GET /queen/export/:issue` returns a git bundle) and a
publisher that runs outside (`~/queen-patches/queen publish`). The publisher
refuses a branch that wrote outside its declared boundary — that check caught 14
of 48, all carrying the same neighbour's commit from a reused worktree.

## Reading the telemetry without fooling yourself

- `incompleteSpec` is a **note, not a refusal** — there is no `continue` after
  it. An issue marked so can still be chosen.
- `notFirst` means queend already picked one this tick. It picks **one per
  tick**, so the swarm fills lanes at the tick rate, not all at once.
- `healthy_idle` and "the queue is exhausted" look identical from outside.
  Before calling idleness a fault, subtract `fileConflict` and `claimed` from
  what looks reachable — the first version of the health check reported "8 idle
  lanes beside 22 available issues" when all 17 of them were the same 17 blocked
  by held files.
- `t27c="absent"` on a review with `specs=0` is normal: the witness is not
  called when no `.t27` changed.

## Things that were silently wrong

- **A constant that exists twice.** `maximumConcurrentWorkers` lives in
  `trios/rings/SR-00/QueenDelegation.swift` and in
  `queen-core/Sources/QueenPolicy/QueenDelegation.swift`. The image builds the
  second. Raising it in the first, rebuilding and deploying produced a swarm
  reporting `capacity: 12` while every tick refused with *"4 workers already
  running (limit 4)"*. CI job `policy-copies-agree` now refuses a PR where they
  differ.
- **`railway up` must run from the repository root**, not from
  `trios/agent-server`: the service sets `rootDirectory` and descends itself.
  Deploying from inside fails with one log line — "scheduling build" — and
  nothing else.
- **The chat starved behind its own bees.** `queen-proxy` shared a z.ai
  credential with the swarm; at full load the Queen answered HTTP 502 with code
  1302. One credential is now reserved for her. z.ai carries exactly two
  concurrent requests per key — measured, four fired at once gave two 200s and
  two instant 1302s.
- **Deployed code that exists nowhere else.** Work was deployed from a tree and
  never committed; a later session had no way to know what was running. Commit
  before deploying, not after.

## The criteria trap — read this before writing an issue

Mechanical acceptance criteria measure whether a spec is SHAPED like working
code. They cannot measure whether it works, and a campaign built only from them
fills the corpus with plausible wrong answers at the same rate as right ones.

Proven, in master:
- `specs/tri/crypto/sha256.t27` implements `Sigma0` as `(x>>2)|(x<<30)`. FIPS
  180-4 requires a three-term XOR; two of the four constants are not even the
  right first term. It satisfies **all six** acceptance criteria of its issue.
- `specs/ml/activation/gelu_activation.t27` divides by `sqrt_2` under a comment
  saying the denominator is √(2π) — wrong by 1.772x, and it generates cleanly,
  so no witness can see it.
- `specs/tri/collections/array.t27` returns `undefined` behind the comment
  "TODO: Handle empty case", worded so it does not match a
  `grep -c 'TODO: Implement'` criterion.
- Tests can be written that re-type the arithmetic inline and never call the
  function they claim to test — they pass on an empty implementation and still
  count toward a test-count threshold.

So: a test-count criterion must require the test to NAME the function. A
correctness claim needs an oracle — a reference vector, a property, a
differential against a known-good implementation — and where none exists, say
in the issue that correctness is unverified rather than implying the criteria
cover it.

## The oracle, and what it cost to not have one

For most of this project's life "done" meant a bee asserted it was done and the
Queen agreed. Nothing executed. `queen food` counted empty function bodies and
called the number progress — but a body full of wrong code scores better on that
metric than an empty one, so the metric rewarded exactly the failure mode nobody
was checking for.

The oracle is `tools/oracle/run.sh`, or `queen oracle`:

    spec -> t27c gen -> mirrored .zig tree -> zig test

Two things make it work, and both are non-obvious:

- **Mirror the tree.** Generated files import each other by relative path
  (`../../base/types.zig`). Generate into a flat directory and every one of them
  fails with "import of file outside module path", which reads like a compiler
  problem and is not.
- **Shim at the tree root.** Zig takes the module root from the root source
  file's directory, and there is no flag for it in 0.15/0.16 (`--main-mod-path`
  and `--mod-path` are both gone). Write `test { _ = @import("a/b/c.zig"); }`
  into a file at the tree root and the whole tree becomes the module.

First full run: **329 of 906 pass, 575 do not compile, 2 fail their own tests.**
That number had been unknown and unknowable for the entire project.

### Attribute the failure to the file it is IN, not the file under test

343 of the 361 `@intCast` failures were not in the file being tested. They were
inherited from `specs/base/types.t27`, which all of them import. A per-file
error count said "361 broken specs"; the truth was "one broken spec and 343
victims". The oracle reports `[in <file>]` for exactly this reason — without it
you will go fix 361 things that are already correct.

I got this wrong first: I grepped the specs for untyped `@intCast`, found 2
where the oracle found 361, and briefly concluded the codegen was inventing
them. The grep was right. The import graph was the missing term.

### The metric it replaced was measuring the wrong thing

Keep `queen food` for a sense of volume, but never report it as quality. The
pair to report is:

    queen oracle     # how much runs      <- quality
    queen food       # how much is left   <- volume

### Ratchet, do not gate

`tools/oracle/baseline.tsv` records the passing set. The oracle exits non-zero
only if a spec that used to pass stops passing. It does not demand that new work
pass, because 575 specs currently do not and a gate would block everything. A
ratchet lets the number climb without ever slipping.

## Editing the witness script — four ways it bites

`WITNESS_SCRIPT` in `queen-dispatch.ts` is the only code the Queen runs against
a bee's actual commit. It is also a `String.raw` template literal inside
TypeScript, executed by `sh` in a container, against a checkout you cannot see.
Every one of these cost a deploy to learn:

1. **No `${...}` anywhere.** A dollar-brace is a TypeScript interpolation. The
   file stops compiling and the error points at the shell, not at the template.
   Use `sed` for trimming and `printenv VAR` for defaults — `set -u` makes a
   bare `$VAR` fatal for an unset variable, so a braced default is exactly what
   you reach for and exactly what you cannot have.
2. **`t27c` writes imports relative to where the SPEC sits.** The parse checks
   copy the spec to a flat `$tmp/specs/one/<name>.t27`. Generate from there and
   every import is off by one directory:
   `specs/one/quick_sort.t27` emits `../base/types.zig`;
   `specs/tri/sort/quick_sort.t27` emits `../../base/types.zig`.
   Anything needing imports must generate from a MIRRORED path.
3. **The witness is killed at 120s and a killed witness reports `parses:
   false`.** That is a bee blamed for a defect it does not have. Nothing
   expensive belongs in the script — bake it into the image instead.
4. **The image's `specs/` is not the corpus.** The agent server ships sixty-odd
   specs of its own; the 946 the oracle measures come from gHashTag/t27 through
   the t27c stage's sparse checkout, which lists `bootstrap gen` and had to be
   told about `specs`. The first baked tree said "60 generated" and looked fine.

Test it without deploying: `queen witness specs/agents/a.t27`. It extracts the
template, runs `sh -n` on it first, and executes it against the rescue
checkout with `TRIOS_ORACLE_TREE` pointed at a local tree.

## Make a new verdict a ratchet, not a gate

The oracle criterion is emitted three ways, and the two silent ones matter more
than the loud one:

- `oracle === null` — not measured (no zig, no tree). **No line at all.** An
  unmeasured file must never read as a passing one, and a `boolean` defaulting
  to false would have read as failing.
- `oraclePreBroken` — the base ref did not compile either. **No line at all.**
  384 of 946 specs are in this state; emitting it unmet would fail every bee
  that touched one and stop the queue inside a tick.
- measured, base was fine, branch is not — **the one case the bee owns.**

This shape is why the existing witness tests still pass untouched: their
fixtures carry no oracle line, so the field stays null and no line appears.

## Where this sits against the field (checked 2026-09-16)

Spec-driven development with agents is a crowded 2026 topic — Microsoft's SDD,
OpenSpec, SpecD, amux, and the SDAD paper (arXiv:2608.20341), which formalises
"independent multi-agent verification under human sign-off" and argues for
"separation between synthesis and release authority". AProver goes further and
pairs an LLM with a bounded model checker.

All of them write the spec in English or markdown and then **translate**
acceptance criteria into tests. t27's difference is that the spec is the source
language and compiles, so the criteria do not need translating — the compiler
is the judge. That advantage was theoretical until this cycle, because nothing
executed the output. It is the reason to keep the oracle binding rather than
advisory.

Their idea worth stealing: SDAD's separation of *synthesis* from *release
authority*. The swarm has it by accident (bees cannot push; publishing is
external) — it should be deliberate.

## Ship the telemetry in the same deploy as the mechanism

The oracle shipped verified: tested end to end locally against the real corpus,
three semantics confirmed, and reviews reaching the compiler jumped from 4 to
27. Every one of those statements was true and the feature was doing nothing at
all in production.

What said so was a one-line log added in the same session, because the criterion
text is not logged and not on any endpoint:

    oracle="no witness" x59   oracle="not measured" x4   (no verdicts at all)

`cp -al` does not link across mount points. The tree lives in an image layer and
`$tmp` is under `/tmp`, so the copy test simply returned false, the block was
skipped, and nothing was logged. `cp -a` as a fallback turned that into 19
`oracle="fail"` in the first window — including a bee that invented a Zig
builtin, `@ptrFromNull`, on an issue whose every stated criterion passed.

Two rules from this:

- **A gate nobody can observe is indistinguishable from one that is not
  running.** Add the log line in the same commit as the check, and keep "not
  measured" a distinct value from "pass".
- **"It worked locally and the adjacent numbers improved" is not evidence.**
  The same cycle had already recorded that lesson about the issue-paging fix,
  and I repeated it four hours later.

Also: the shell lives in a `String.raw` template, and a backtick inside a
comment closes it. The comment explaining that rule was itself written with
backticks and stopped the file compiling.

## Do not make a check required until it is green on the work you produce

Five checks were made binding on `t27-master-protection` in one move:
`validate`, `check-linked-issue`, `emit-bitexact`, `spec-guards`,
`Corpus ratchet`. They are the right checks. Bee PRs fail four of them, so the
swarm ran at full capacity producing work that could not land.

Before promoting a check to required, run it against a representative PR of the
kind the swarm actually opens — not against master, where the PR-triggered
checks do not run at all.

## The work was never leaving the container

219 bee branches of finished, Queen-accepted work sat in the container with no
way out, while 401 of 434 reviews reported `specs=0` and waited forever on
branches that do not exist on the remote. The container holds no push
credential by design, and that rule was kept while its consequence went
unhandled for months.

`queen publish --push` is the way out: the container EXPORTS a bundle holding no
credential, and the publisher pushes from a machine the bees cannot write to.
Run it. It is not optional plumbing — it is the only path from a finished bee to
a pull request.

## Judge before publishing, not after

`queen publish` now runs the oracle between fetching the bundle and pushing it.
On the first gated batch of 25: **23 pushed, 2 held** — `expected '}', found
':'` and `expected expression, found ';'`. Both would have become pull requests
and both had every stated criterion satisfied.

Only `fail` is held. `pre-broken` publishes, because the same failure is already
on the base and refusing those holds back 384 specs for a defect they inherited.

## THREE-dot diff, always

`git diff master branch` also shows everything master gained since the branch
forked. The bee branches run ~100 commits behind, so that comparison reported 38
specs per PR and made it look like every bee was carrying its neighbours' work.
It is `git diff master...branch` that says what a PR proposes. I raised a false
alarm on this before checking the merge-base.

## What the field measures, and what it means here (checked 2026-09-16)

Three 2026 studies covering 153,000+ agentic PRs:

- **27.67% textual merge-conflict rate** across 142,000 PRs (Ogenrwot and
  Businge, AIware 2026), attributed to the base moving while the agent worked,
  not to the agent. Our bee branches are far staler than that sample, so
  `queen publish` now prints how far behind the base each one sits.
- **Only 35.7% of rejections are genuine agent failure** (Peralta et al., MSR
  2026 Mining Challenge): 31.2% are workflow constraints and 33.1% have no
  recorded rationale. That is this swarm exactly — closed issues re-reviewed
  forever, a hardcoded wrong repository in a PR body, no publisher running.
  **Before blaming the model, check the plumbing.**
- **Reviewer-bot comment volume correlates with worse outcomes** (Fatima et
  al.): more comments, longer resolution, lower quality. Keep the oracle's
  feedback to one compiler error with the file it is in.

Their conclusion and ours agree: the lever is upstream specification and
workflow, not model capability. The difference is that a t27 spec compiles, so
"did it work" is a compiler question rather than a review opinion.

## Calibrate the instrument before you trust what it measured

The oracle reported `unable to load X.zig: FileNotFound` on 20+ branches and I
filed an issue describing a corpus-wide naming convention that nothing follows,
with a worked FPGA example. **All of it was my own tool.**

t27c resolves a `use` declaration against the FILESYSTEM. A use of
fpga::fifo::Fifo compiles to an import of ../fifo.zig when specs/fpga/fifo.t27
sits beside it, and falls back to an import of ../fifo/Fifo.zig when it does
not. The witness was mirroring ONE spec into an otherwise empty tree, so every
spec using a neighbour generated an import to a path nothing produces.

Two things would have caught it before the issue was filed:

- **Run the instrument on something known-good.** The same spec generated from
  the real corpus emitted the correct import. One command, and the false
  finding evaporates.
- **Ask whether the defect has the shape of your tool's blind spot.** "Every
  file in a family has a wrong import" is not how corpora decay; it is how
  harnesses fail.

The 28 unresolved modules in that issue were real -- they came from a tree
generated out of the full corpus -- but the narrative was withdrawn publicly
rather than quietly edited.

Sengupta et al., arXiv:2605.25665, call this a **verification-boundary issue**
and report the same arc: their deployment case study revealed one, and fixing
the harness was the finding. They pair it with "outer-loop calibration" --
treating the harness itself as a thing under continuous test, not a fixed
oracle.

## `zig test` cannot tell a build failure from a test failure

Both exit non-zero. 21 specs here compile and then panic on an unimplemented
body, so their base always "failed" -- and a bee could break compilation on any
of them and be told the breakage was inherited. Exactly that happened on
fifo_tb.

`--test-no-exec` builds the test binary without running it. The rule that works:

    base compiles && branch does not            -> the bee owns it
    both compile && base's tests passed && branch's do not -> the bee owns it
    otherwise                                   -> pre-broken

And say which one happened. The verdict was reporting the first line of the run
output -- "test command terminated with signal ABRT" -- on a branch whose real
problem was a syntax error, and naming the file by an absolute path inside a
mktemp directory. A verdict that blocks work has to say what to fix.

## Guard the template mechanically

`queen witness` now refuses a WITNESS_SCRIPT containing a backtick or a
dollar-brace before running anything. Both stop queen-dispatch.ts compiling, and
the comment warning about backticks has been written WITH backticks three times.
A rule you keep breaking belongs in a check, not in a comment.

## Pin the compiler, or every number you produce is about a different program

Production reported `oracle="fail"` on branches a local run of the *same witness
script* called `pre-broken`. Chasing it down took three wrong hypotheses:

1. "They judge different commits." No — the container's branch tips matched
   `origin/<branch>` exactly, checked through the export endpoint.
2. "The base differs." Partly. The local checkout sat on a feature branch
   carrying nine `types.t27` annotations that are NOT on master, so local runs
   measured a corpus with an unmerged fix applied. Re-judging from a detached
   worktree at `origin/master` changed 2 verdicts from `fail` to `pre-broken`.
3. The real one: **the compiler binaries were different programs.**
   `~/t27/target/release/t27c` was built on 8 September from the operator's
   working tree, which carries uncommitted files under `bootstrap/`. It does not
   even accept `--version`. Production builds t27c fresh from
   `gHashTag/t27@master` in the image. Different codegen, different Zig,
   different verdicts — production says `fifo_tb` fails on
   "local variable shadows declaration of 'wr_en'"; the eight-day-old compiler
   emits code that compiles and panics instead.

Everything the local oracle measured this session went through that binary: the
corpus counts, `tools/oracle/baseline.tsv`, every `queen witness` and
`queen ready` verdict, and the unresolved-import table.

**Rules that follow:**

- Build `t27c` from the ref you are judging against, into a scratch tree. Never
  reuse a binary from the operator's checkout — it is a different program that
  happens to share a name.
- Record the compiler's provenance beside every number. A pass count without a
  compiler sha is not a measurement, it is an anecdote.
- When production and a local run disagree, **production is the authority**: it
  is the thing that actually gates work, and it builds its own toolchain from a
  known ref.
- Log the REASON, not just the verdict. One word (`fail`) could not settle this;
  `oracleDetail` naming the file and the error settled it in one read.

## The base can be wrong in three separate ways, and it was wrong in all of them

A regression gate is only as good as what it compares against. Every one of
these produced the same visible symptom -- `oracle="fail"` on branches whose
defect is on master, and `pre-broken` never appearing once in several hundred
reviews:

1. **Two files meant different things by the variable.** `queen-export.ts` read
   `TRIOS_REPO_REF` as a BRANCH NAME and prefixed `origin/`; `queen-dispatch.ts`
   read it as a full REF, defaulting to `origin/dev`. Production sets `master`,
   so export resolved `origin/master` while every review compared against the
   container's LOCAL `master`. Fixed with one shared `baseRef()`: a bare name is
   qualified, a qualified ref is left alone.
2. **I "fixed" it in the variable and broke publishing.** Setting
   `TRIOS_REPO_REF=origin/master` made export emit `origin/origin/master`, and
   `/queen/export` began answering 409 on every issue. A variable is the wrong
   place to patch a disagreement between two readers of that variable.
3. **The ref was right and stale.** The workspace fetches when a worktree is
   PREPARED; a dispatch row then sits in review for hours while master moves.
   `git show origin/master:<file>` returns the last fetch. Now the review
   fetches once per issue before judging.

**The tell was in the distribution, and I missed it twice.** 384 specs do not
compile, so most reviews should INHERIT a failure. A verdict set with hundreds
of `fail` and zero `pre-broken` is not a swarm writing bad code; it is a broken
comparison. When one bucket of a verdict never fires, suspect the gate before
the work.

## Re-verify what you merged when the instrument changes

Three PRs were merged this session as "oracle-verified". Re-checked with the
compiler built from master:

- `fpga/testbench/cts_tb`, `fpga/testbench/router_tb` — compile, tests pass.
  Genuinely verified.
- `tri/utils/color` — does NOT compile. The stale compiler generated different,
  compiling code. The merge introduced no regression (it did not compile before
  either), so no harm, but the claim was wrong.

Say which is which. "Verified" with the wrong instrument is a claim, not a
measurement.

## The compiler is sealed, and that seal is the one place to stop

`bootstrap/src/compiler.rs` carries a FROZEN_HASH. Edit it and `cargo build`
refuses:

    t27c FROZEN HASH violation: bootstrap/src/compiler.rs has changed without a
    seal update.

FROZEN.md §5 step 2: the PR must be marked `[GOLD-RING]` with a milestone, **or
be an Architect-approved hotfix**. That is explicit human authority, and an
autonomous cycle does not have it. Write the patch, verify it in a scratch
worktree, revert it, and file it with the diff and the measured blast radius.
A finding a human can apply in one command is worth more than a change they
did not sanction.

## A fix that measures zero is a finding, not a failure

Merging the `types.t27` annotations moved the corpus by exactly zero specs:
279 PASS before, 279 after, none gained, none lost. The defect those
annotations addressed was emitted only by the stale compiler. The real blocker
in that same file is `pub fn packed_to_trit(packed: u8)` -- the codegen quotes
`packed` as `@"packed"` in bodies but not in parameter lists.

Measure before AND after, on the same pinned compiler, and report the delta even
when it is zero. Especially when it is zero: three cycles of my reporting rested
on a number that never existed.

## The freeze ceremony, step by step

`bootstrap/src/compiler.rs` is sealed. Edit it and `cargo build` refuses with the
live hash and the expected one. FROZEN.md §5 step 2 requires `[GOLD-RING]` with a
milestone **or an Architect-approved hotfix** — explicit human authority, which an
autonomous cycle does not have. With that authority, the ceremony runs like this:

1. **Baseline FIRST, on the clean tree.** `cargo test --release` before touching
   anything — 114 binaries, 2715 tests here. Without it you cannot tell your
   breakage from the repo's.
2. Apply the patch.
3. **The seal comes from the EXISTING binary**, which is the way out of the
   chicken-and-egg: `./target/release/t27c frozen-digest bootstrap/src/compiler.rs`
   takes a path argument. Its output is exactly the "Live hash" build.rs printed.
   Redirect it into `bootstrap/stage0/FROZEN_HASH`.
4. M1: `cargo build --release` — now green.
5. M3: `cargo test --release` again and compare totals to the baseline. Identical
   or it is not gold.
6. M4: there is no `tests/run_all.sh` in this repo. The PR's CI stands in.
7. Commit explaining WHY the seal moved, with both hashes in the message.

**Before merging a compiler change, prove it adds no generation failure.** Generate
the whole corpus with a t27c built from the branch and one built from master, and
diff the sets of files that generated at all:

    specs that generate, before: 945    after: 945
    newly failing to generate: 0        newly generating: 0

`emit-bitexact` will be red anyway — master carries ~10 specs that do not parse —
and that evidence is what separates "red because of me" from "red already".

## Report a zero honestly, twice if it happens twice

The empty-array fix removed the defect from **227 generated files, leaving 1**, and
cut the "expected type expression" class from 262 to 107. The corpus pass count
moved by **zero**: 279 before, 279 after. The same files have a second defect
behind the first — `pub const TOOLS: [1]str = [tri/gen];`, a single-element literal
holding an unquoted path.

That is the second correct fix in a row to measure zero. A defect class removed is
worth landing; a pass count is what you may not claim moved.

## Find the disambiguator, do not guess at the ambiguity

`[4]` and `["tri/gen"]` both look like a bracket holding one thing, and
`parse_bare_array_literal` refused both because the first is an array DIMENSION.
That refusal is sound for an integer or a name and simply does not apply to a
string: **no type is written `["tri/gen"]`**. One type test settles 151 of the
corpus's single-element literals and leaves the genuinely ambiguous ones alone.

The signal was already in the tree: the expression parser stamps
`extra_kind: "string"` on every string literal. Look for the property that makes
the case decidable before widening a heuristic — a rule that cannot be wrong
beats a rule that is usually right.

## Two ceremonies, and what they actually bought

    empty literal   #3962   `= [];` in 227 files -> 1     PASS 279 -> 279  (+0)
    single string   #3973   "expected type expression"
                            107 -> 6                      PASS 279 -> 280  (+1)

Across both, that error class went 262 -> 6 and is finished. The pass count moved
by one spec.

That is not failure, and it is not a headline either. **The corpus fails in
layers.** Clearing a parse-level defect lets a file reach name resolution, where
it fails again: `use of undeclared identifier` rose 296 -> 420 across these two
fixes, which is files getting FURTHER, not getting worse. Report the class you
removed and the count you moved, separately, and never let the first stand in
for the second.

## Decompose the class before believing it is many problems

"420 use of undeclared identifier" sounds like 420 separate name-resolution
questions. Counting the identifiers took one command and said otherwise:

    285 'str'      18 'T'      7 'goldenfloat_family'      6 'default_input'

**285 of 420 were one identifier.** The largest class is usually one thing
wearing a crowd's clothes. Cluster by the actual token before planning work.

Then probe every position the type can appear in, because the gap is never where
you assume:

    str       -> []const u8    correct
    ?str      -> ?[]const u8   correct
    []str     -> [][]const u8  correct
    [str; N]  -> [N][]const u8 correct
    [0]str    -> [0]str        WRONG

The mapper handled t27's own spellings and Zig's slice, and missed the
Zig-shaped SIZED array, which is the form the specs actually write. One case,
recursing into the element and leaving the dimension alone.

## The three ceremonies, and the shape of real progress

    #3962  empty literal      class 227 -> 1      PASS 279 -> 279   (+0)
    #3973  single string      class 107 -> 6      PASS 279 -> 280   (+1)
    #4114  sized-array elem   class 285 -> 0      PASS 280 -> 553   (+273)

The first two looked like failures by their pass count and were reported that
way. They were not: each cleared a parse-level wall so files could reach name
resolution, where the third fix was waiting. **273 specs started passing and
none stopped.**

A layer is only worth its headline once the layer behind it is also gone. Report
(class removed, count moved) as a pair every time, and keep going.

## Do not flood CI when you need a pull request through

A batch publish opened ~40 PRs and GitHub Actions saturated: 28 runs queued and
not draining, with the compiler PR stuck behind work that could have waited.
Land the change that matters first, then publish.

## PASS means "compiles" unless the file has a test in it

`zig test` exits 0 for a file that contains no test. The oracle's PASS column
therefore mixed two different claims, and the headline read the stronger one:

    generated Zig compiles                         553
    ...and carries at least one test, all passing  206
    ...but carries no test at all                  347

The sized-array type fix took compilation 280 -> 553 (+273) — true, and it was a
compile error, so that is exactly what it fixed. Tested correctness moved
151 -> 206 (+55). I first reported "553 compile and pass their tests" in a PR
body and corrected it publicly. Always split PASS before quoting it; the check
is one grep for `^test "` in the generated file.

347 specs that compile with nothing to run is also the largest single gap left
in what the oracle can verify — and it is mechanical work for bees.

## Before building something recurring, check who is already building it

This cycle opened with the operator's rule in memory — recurring swarm work runs
in the cloud, not on a laptop that sleeps — and two local scheduled tasks still
doing it. A running session was already moving the feeder to GitHub Actions
(PR #3978). `list_sessions` and `list_scheduled_tasks` first; then take the part
nobody holds.

Two cloud-native moves that needed no poller at all:

- **Native auto-merge** (`gh pr merge --auto --squash`) on a PR stuck in a
  saturated queue. GitHub lands it the moment its required checks pass. No cron,
  no laptop, no babysitting — which is what the memory rule asks for by name.
- **A nightly job, not a per-PR one,** for anything heavy. The per-PR queue was
  measured at 60 runs and not draining; adding a tens-of-minutes oracle to every
  PR would have made the thing blocking everyone worse. Nightly still catches a
  regression the day it lands.

`auto-merge-ready-prs.yml` on master looks like cloud merging and is not: it is
`workflow_dispatch` only, a button with no schedule.

## An `&&` that gates only the counter

`tools/oracle/run.sh` generated each spec with

    "$T27C" gen "$f" > "$dst" 2>/dev/null && [ -s "$dst" ] && gen=$((gen+1))

The redirection writes the file before the exit code is known, and the `&&` only
decides whether the counter moves. t27c emits whatever it managed before failing,
so a TRUNCATED module stayed in the tree, and when the fragment happened to be
valid Zig it ran under `zig test` and scored PASS. 89 of 278 "passing" specs had a
gen that exited non-zero; `ternary_logic.t27` has 33 tests and its fragment had
none. Every corpus number that script produced was inflated this way.

The review witness wrote `if gen ...; then zig test` and the image tree wrote
`gen || rm -f`, so bee verdicts were sound. Only the counter lied. When a pipeline
feeds a number, check that each stage's failure REMOVES its output, not merely
fails to count it — and record the failure as its own verdict (NOGEN) instead of
dropping it, or the denominator shrinks and the rate flatters.

## A timeout on `zig test` has to kill the process group

`zig test` builds a test binary and runs it as a CHILD. When a test never returns,
it is that grandchild that hangs. `alarm` or GNU `timeout` kills zig; the orphan
keeps the output pipe open; `$(...)` waits on the pipe forever. A 20-second limit
held open for seven minutes before this was understood. Fork, `setpgrp(0,0)` in
the child, and on the alarm `kill -TERM -$pid` — the whole group. Verified: the
hung spec resolves as TIMEOUT in 22s with no orphaned binary.

And classify it. A hang is `TIMEOUT`, not `NOCOMPILE`: it compiled.

## A spec with no base has nothing to have inherited

`pre-broken` means the base was already broken. The witness decided that by
trying the base, and a missing base left the same flags as a broken one — so a
NEW spec that failed was filed pre-broken, the one verdict that lets a failure
through unowned. The porting feeder produces nothing but new specs. Record
`base_exists`; no base means the failure is the bee's. Verified with a controlled
branch, and against the three cases that must not change.

## Before deploying, fast-forward to what is already deployed

The local BrowserOS checkout was 10 commits behind `feat/queen-supervisor`;
another session had pushed `119d9be9`. Deploying from the stale tree would have
reverted their work. `git merge-base --is-ancestor pub origin/<branch>`, then
`--ff-only`, then build on top. `railway up` uploads a directory, not a ref.

## Verify every claim an issue makes before a bee reads it — and verify the instrument

**This section used to hold up the 11 "add tests" issues (#4236-#4246) as the
model: every quoted number re-ran with zero mismatches.** Ten of the eleven were
false. The picker counted tests with `grep -cE '^[[:space:]]*test [A-Za-z_]'`,
which misses the quoted spelling `test "name" {` — the files carried 1-9 tests
each, and the generated Zig ran all of them. The numbers reproduced because the
same wrong grep produced them both times. Closed as not planned with the evidence.

Re-running a command proves the NUMBER, not that the command measures the thing
the title says. Cross-check with a second instrument that does not share the
first one's assumptions: tests in the `.t27` (quote-aware grep) against
`t27c gen X | grep -cE '^test "'` in the output; a regex against `t27c
spec-status`. When they disagree, the issue is not written.

What is mechanical now:

- `queen_feed.py` runs every command the body quotes, as the exact string, with
  `t27c` on PATH resolving to the provenance-checked judge (`work/bee-bin/t27c`),
  and skips the file unless each prints the "today" value. It refuses to run at
  all (exit 3) if `queen judge` fails and one `queen rebuild-judge` does not fix it.
- `queen verify-issue N... | --open` does the same for issues already published:
  `holds`, `STALE-CLAIM`, `UNCHECKED` (a command not on the safe list),
  `+BEE-CANNOT-RUN` (names `/Users/playom/t27/target/release/t27c`). Commands run
  only if every program is in {t27c, grep, wc, head, tail, sort, uniq, cut, tr,
  cat} and redirects go to `/tmp/t27-*`: issue bodies are text anyone with write
  access can edit, and this executes them on the operator's laptop.
- `(on arrival: N)` and `# ... not yours yet` are the state an EARLIER part
  leaves. Part 2 of 2 saying 2 while master prints 10 is waiting, not stale.

## A count downstream of a failed command is not a measurement

`t27c gen X 2>&1 | grep -c 'not yet implemented'` prints `0` when gen FAILS. A
bee that breaks the spec satisfies "prints 0"; verified on a copy with a syntax
error appended: old form `0`, status `NOPARSE`. The first version of `queen
verify-issue` reported two parse-broken specs as "goal already reached" for the
same reason. Write `t27c gen X > /tmp/t27-gen.zig && grep -c ... /tmp/t27-gen.zig`
— a failed generation prints nothing, which matches no number.

Measured 2026-09-17 on the open backlog, judged by the master compiler: 137 open
issues name `/Users/playom/t27` paths; of the 85 naming the stale binary, 84
carry a claim that does not reproduce in the container (31 behind a failing
`t27c`, most of the rest a `recovery-events:` parse metric only the uncommitted
binary printed). 47 of the 85 already have an open bee PR. Not rewritten yet:
every issue edit fires `notebook-sync`, and the Actions queue was saturated.

## The feeder's "52 issues" was the stale binary again

Same master commit, same 196 covered files: the old binary counted 483 empty
bodies in 139 files (47 uncovered); the judge counts 245 in 70 (11 uncovered).
Of the files that dropped out, most are IMPLEMENTED under master's compiler or
NOPARSE there (10 of 23 are bodiless `fn f(...) -> T;` declarations the stale
binary parsed). The bee's container runs master's compiler, so 14 was right.
The Zig backend emits `@compileError("not yet implemented")` only for a
FnDecl with no children, so gen stubs and `spec-status` empties agree by
construction — checked: all 70 files, stdout-only count == merged count.

## "PASS without tests" is mostly files with nothing to test

Of the 39 passing specs whose generated Zig has no `test "`, 37 declare no
function at all (`spec-status` = NOFN — 12-line placeholders and constant
tables). Two have code: `specs/ternary/clocked_counter.t27` (#4246) and
`specs/numeric/formats_catalog.t27` (83 functions). Report "PASS, no function"
and "PASS, functions but no test" separately.

## The Actions queue is a swarm bottleneck: group every gate

2026-09-17 ~11:00 UTC: 2267 runs queued, 19 running; ~459 created per hour and
~171 completing. 577 were post-merge master runs, and they were 61-62 per gate
for the ten gates WITHOUT a `concurrency:` group against exactly 1 per workflow
WITH one. 84 of 85 bee PRs sat in WAITING on queued required checks.

Relief: cancelled 552 superseded queued master runs of those ten, keeping the
newest per workflow (list in `queen-patches/backup/cancelled-superseded-*.tsv`);
queue 2267 -> 1656. Fix: #4249 / PR #4250 adds
`group: <file>-${{ github.ref }}` + `cancel-in-progress: true`. Deploy and
image-build workflows are deliberately excluded. Measure the queue with
`gh api 'repos/gHashTag/t27/actions/runs?status=queued&event=push&branch=master&per_page=100'`
grouped by `.path` — the list endpoint caps at 1000 results, so filter first.

## Local scheduled tasks die on a model name, silently

From 07:39 UTC every run of `queen-feed-empty-bodies` and
`queen-batch-merge-bee-prs` failed within 3 seconds: "issue with the selected
model (glm-5.2[1m])". `~/.claude/settings.json` points Claude Code at z.ai with
`glm-5.2`; the scheduler asks for the 1M-context variant, which z.ai rejects.
Nothing in `feed.log` shows it — the script never started. Check
`list_task_runs`, not the log. The provider setting is the operator's, not ours
to change.

## A rule broken five times belongs in a check: `queen judge`

The local judging compiler was wrong five separate times in two days — built from
uncommitted source, from a checkout carrying an unmerged fix, and last from a
worktree left on a measurement branch, so `queen ready` was silently judging with
master PLUS an unmerged compiler change. Each time the lesson was a comment.

Now `queen witness`, `queen ready` and `queen oracle` REFUSE unless the judge
proves where it came from. `queen judge` reports; `queen rebuild-judge` resets the
worktree to origin/master, builds, records provenance, regenerates the dependency
tree with failed generations left out, and verifies itself.

**The first version of the guard was blind to the original failure.** It compared
`HEAD:bootstrap` and passed the contaminated judge, because that change had been
applied with `git checkout <ref> -- file` — which edits the file and not HEAD.
The tree hashes matched; the compiled source did not. It now checks what is ON
DISK: `git diff --quiet origin/master -- bootstrap` and the `git hash-object` of
compiler.rs against origin/master's blob. Always test a guard against the exact
bad state it exists to catch, not a convenient stand-in.

Key on the compiler source, not the commit. master takes ~23 merges an hour,
nearly all specs; a commit key would force a rebuild every few minutes.

This is SLSA's build provenance in miniature: record the resolved source the
build actually consumed and who built it, because a commit reference alone does
not describe a build from a dirty worktree. SLSA's higher levels require a
hosted builder for the same reason — which is also why the image's t27c, built
from master in an isolated Docker stage, is the reference and the local judge
only a convenience that must prove itself against it.

## Editing a PR body resets its required checks

`check-linked-issue` runs on `pull_request_target: [opened, edited, reopened,
synchronize]`. Each correction to a PR description fired `edited`, queued a fresh
run, and GitHub reports the NEWEST run of a check — so a green result from an hour
earlier was replaced by `pending`, and the PR sat BLOCKED behind a saturated
queue. Three PRs on auto-merge stayed unmerged while bee PRs created later landed.

Correct the record anyway; a wrong public number is worse than a slow merge. But
batch the corrections into one edit, and expect the merge to wait for the queue.

## "Nothing is running" was a snapshot, not a stall

A read of 72 queued / 0 in progress looked like dead CI. Twenty seconds later: 40
successful runs and 23 PRs merged into master in the hour. The queue turns over
fast enough that any instant can show zero in progress. Measure throughput —
completions and merges over a window — before calling a pipeline stuck.

## Zero is not a result, and a script travels with its baseline

Two failures from the same afternoon, both "success" that measured nothing true:

- **An oracle run over zero specs exited 0.** A copy of run.sh placed at the repo
  root resolved its root as `../..`, climbed out, found no specs, and printed
  `0 / 0 pass`. The ratchet saw no regression; the caller saw green. run.sh now
  exits 2 when nothing generates. (Verify an exit code with `$?` on its own line —
  zsh has no `${PIPESTATUS}`, and piping to `tail` hides the real code.)
- **The fixed script against the stale baseline reported ~350 phantom
  regressions.** master still had `pass 541`; the fixed script measured 189 and
  listed every gap as "no longer passing". Those specs never passed. A later cycle
  reading that would chase a regression that does not exist. The script and its
  baseline are a matched pair: borrow both, or neither — and restore what you
  swapped, then check git says it is untouched.

## The NOGEN class is fragmented, and most of it is the specs, not the compiler

89 specs fail `t27c gen` itself. Unlike `use of undeclared identifier`, where 285 of
420 were one token, this class does not collapse: the largest cluster is 15, then
10, 6, 5, 5, 4. Before reaching for a compiler change — and the seal that comes
with it — ask whose defect each cluster is:

- **A bodyless `fn sig;`** (15 specs). Not defined in docs/ or CANON.md, and 0 of
  the 15 specs using it generate. So it is not valid t27; the empty body
  `fn sig { }` is, and means the same thing ("declared, not implemented").
  Converting fixed 3 of 10 unclaimed files outright; the other 7 moved to a
  deeper, unrelated parse error.
- **Markdown prose in the file body** (`# Title`, `## Specification`, paragraphs).
  `#` is a comment in t27; bare prose is not. Commenting the leading prose moved
  0 of 21 to generating — every one had another defect behind it — and a heuristic
  that comments dozens of lines cannot promise it never touched real code. Reverted.
  A change that measures zero and carries risk is not committed.

**Neither was a compiler bug.** The compiler was right to reject both. No ceremony.

## "Impossible" issues that bees quietly solved

Five issues filed from queen_feed.py asked bees to implement empty bodies in files
that did not generate at all, because their functions were bodyless `fn sig;`. They
looked unachievable. They were not: four already had bee PRs, and in every one the
bee had replaced each declaration with a real body — which is exactly what removes
the `;` — and the file generated. The work was the fix. Check the branches before
calling an issue blocked.

Those same issues tell bees to use `/Users/playom/t27/target/release/t27c`, the
stale laptop binary that does not exist in the container. The template came from
queen_feed.py; the other session's feeder has the same line. Both need the
container's t27c.

## zsh does not split unquoted variables

`for f in $LIST` loops ONCE over the whole string in zsh, unlike bash. A
verification loop built that way checked a single glued path, and a stray `mkdir`
from it later left a directory where a file was expected. Use an explicit word
list, or `${=LIST}`, and never trust a check whose loop body ran once.

Also: `path` is tied to `PATH` in zsh. `while IFS=$'\t' read id path c sha` turned
every later command into "command not found" — 558 cancel calls failed at once.
Never name a variable `path` (or `status`, which is read-only).

## Run `t27c --help` before building an instrument

The compiler ships measurement commands marked "THE SERVICE", and I spent two days
rebuilding a worse copy of one:

- `t27c corpus` — "the only corpus metric that does not lie": per backend, does it
  generate, does it compile, every step timed out, and it REFUSES the run when a
  tool is absent or I/O fails. My hand-built `tools/oracle/run.sh` acquired those
  properties one embarrassing defect at a time.
- `t27c classify` — is this file source at all? 27 of 946 `.t27` files are not
  (14 Markdown documents, 8 `spec X {`, 4 unclassified, 1 mixed). **919 is the
  honest denominator**, not 946. It already knew the "Markdown prose" cluster I
  tried to fix by commenting lines out — those files are documents, and belong
  outside the count, not inside it repaired.
- `t27c spec-status` — one word: IMPLEMENTED / PARTIAL / UNWRITTEN / NOPARSE / NOFN.
  A far better acceptance criterion than grepping parse metrics that the master
  build does not even print.

What the hand-built oracle still adds: it runs the tests (`corpus` compiles), and
it splits PASS by whether a test exists.

## Zig is lazy, and the review inherited that

`zig test` type-checks only what a test reaches. A private function no test calls
is never analysed — an unreferenced `const x: u8 = "string"` exits 0.

Three things that look like fixes and are not:
- `std.testing.refAllDecls` — reflection sees only `pub` declarations, and 82% of
  generated functions are private (5742 vs 1260).
- `std.testing.refAllDeclsRecursive` — does not exist in zig 0.16. Its "exit 1"
  was "no member named refAllDeclsRecursive", not a caught error. Read the message.
- `t27c corpus`'s "Zig ANALYSES it" — also `zig test --test-no-exec`, same blind
  spot. The limitation is shared, not a flaw unique to one tool.

What works: append `comptime { _ = &f; }` for each non-generic top-level function
(skip generics — taking their address is itself an error; read the file whole so
a multi-line signature is seen). On the 189 passing specs: 181 clean, **8 hid body
errors**. Now in the review witness: a bee's untested private function with a type
error went from `pass` to `fail`. Two independent implementations of the transform
agreed exactly (181/8, 0 disagreements) before it shipped.

## Declare a runtime dependency; do not assume it

The body-forcing transform needs perl and falls back silently without it. Debian
marks perl-base Essential, so it is almost certainly present — and "almost
certainly present" is exactly how `cp -al` once left the oracle inert. It is now in
the Dockerfile's apt-get line, and the build log is checked for it.

## The review column is a display, not a queue

`/queen/public-board` showed 180 cards "in review" while 30 of 32 lanes idled.
Measured: the kanban puts EVERY verdict other than `accept` there, with no
open-issue filter and no valve (`queen-kanban.ts` dispatchColumn), so closed
issues whose PR merged, send-backs at the ceiling, escalations and waits the
Queen had already released all count. `queen review-queue` splits it:

    review column: 177 card(s)
      66  closed issue, work merged
      14  closed issue, nothing merged
       8  open, PR open
      89  open, no PR

Only the last two groups can still change. The Queen re-reads just the `wait`
rows (`review_state IS NULL OR = 'wait'`), which was 41 of 177.

The verdict itself came from the bee grading ITSELF: `parseVerdictBlock` over
the bee's own transcript. With no verdict block and no commit, queend answers
`wait` (verdicts.count < totalCriteria is checked BEFORE committedFiles), the
row is re-judged every 60s forever, and the 6h frozen-wait valve releases it
into another identical attempt. Silence spends no retry budget, so the loop has
no end. That is what "review is overflowing" was.

## What the swarm did to master, measured

`t27c spec-status` over all 948 specs, by date: 68 NOPARSE on 2026-09-14, 68 on
09-16, 90 on 09-17, 79 later that day. IMPLEMENTED went 346 -> 499 in the same
window. So the swarm adds implementations AND breaks files, and nothing gates
it: the required checks on master are `validate` and `check-linked-issue`, and
neither runs the compiler over a changed spec. `Corpus Ratchet` would catch it
and is not required (it was itself red because a file IMPROVED and the ledger
was not re-blessed - #4270/#4271).

14 of those unparseable files had a working version sitting in the bee branch
whose PR could not merge (the file had been overwritten on master meanwhile).
Taking it back: NOPARSE 79 -> 65, IMPLEMENTED 499 -> 513 (#4272). Check for
lost content before doing this: 4 more files were left alone because the
unparseable version declares helper functions the working one does not, and 58
`test` declarations are lost with the 14 - none of which runs today, because
the file they live in does not parse.

## A conflicted PR reports itself as waiting for CI

GitHub runs no required check on a branch that does not merge, so every
`mergeStateStatus: DIRTY` pull request shows its required checks as null.
`queen merge` read that as WAITING ("CI is slow") for all 33 open bee PRs when
the truth was "these need a rebase". Classify DIRTY first; it now prints
CONFLICTED.

## Auto-merge fires on the REQUIRED checks, and waits for nothing else

`gh pr merge --auto --squash` on #4272 landed it 90 seconds later, with
`Corpus ratchet`, `spec-guards`, `check`, `check-now-freshness`, `coverage`,
`cli-tri`, `emit-bitexact`, `gate-topology` and `Check L1 TRACEABILITY` red:
the ruleset requires only `validate` and `check-linked-issue`. The amend that
added the issue refs to the commit message and the `docs/now/` entry was
pushed after the merge and went nowhere - the PR page kept showing the old head
because the PR was already closed as merged, which reads exactly like a stale
API response.

So: get the conventions right BEFORE arming auto-merge, and when a PR "will not
update", check `state` first - `gh api repos/<r>/pulls/<n> -q .state` - because
`gh pr view --json headRefOid` on a merged PR returns the merged head forever.

## What a pull request to gHashTag/t27 must carry

Learned by failing them, on a spec-only change:
- an issue reference in the COMMIT MESSAGE, not only in the PR body (`Check L1
  TRACEABILITY` reads the commits)
- exactly one new `docs/now/<date>-<slug>.md` entry, with a `##` heading naming
  the issues and bullets that say something (`NOW Sync Gate` requires the file,
  `check-now-freshness` reads it). `./scripts/tri now add` writes it - and does
  not run here, because the `tri` binary is not built on this machine; the file
  is plain markdown, so write it by hand to the shape of the newest entry.
- `fix(<compiler scope>)` in the title must carry a source file.
Before blaming your own change, check the same workflow on master: Gate
Topology, NOW Sync Gate, OpenSSF Scorecard, Seal Coverage and cli-tri were
already red there on 2026-09-18.

## Four boots to unstick one checkout, and each one was a different lie

The container's root checkout sat on a two-day-old commit. In order, with the
next boot as the only reading that counted:
1. `fetch && checkout` joined by `&&` printed "fetch failed" - the fetch was
   fine, the checkout was refusing over local changes.
2. The retry trusted the stash's exit code and printed "stash failed" directly
   under git's own "Saved working directory". Read the TREE, not the code.
3. `git stash` writes a commit, and the entrypoint set `user.name` AFTER the
   fetch block, so the stash had no author: "Aborting".
4. `--include-untracked` made the stash reach `.worktrees/` - every bee's
   worktree - and git aborted again. Tracked files only; untracked paths never
   block `checkout -B`.
Each fix was correct and none of them worked until the last. When a boot step
fails, print what the tool printed, not what you concluded from it.

## The model a scheduled task runs on is the app's, not the task's

Both local swarm tasks failed every run for two days: "There's an issue with the
selected model (glm-5.2[1m])". Measured against the provider's own endpoint,
`glm-5.2` answers and `glm-5.2[1m]` does not exist ([1211] Unknown Model). The
id is the app's selected model mapped through `ANTHROPIC_DEFAULT_*` in
`~/.claude/settings.json`, with the 1M suffix kept. None of these moves it:
`ANTHROPIC_MODEL` in settings (the app passes its own), `create_scheduled_task`
(no model field; a task created from an Opus session still ran as glm-5.2[1m]),
the task record on disk (no model key). It is the model picker, which is the
operator's UI.

So the fix is not to pick a model: it is to stop needing one. The merge job is
now `Auto Merge Ready PRs` on a `*/20` schedule (#4279) and the feeder is
`Queen Feed (empty bodies)` at :17 and :47 (#4281), both in GitHub Actions,
both verified by a manual dry run: the CI feeder reports the same "29 files hold
154 empty bodies; 0 uncovered" the laptop reported for the same commit.

Diagnose a provider error by asking the provider, not the client: a `curl` to
`$ANTHROPIC_BASE_URL/v1/messages` with the configured token separates "the key
is dead" from "that model id does not exist" in one call. The CLI's own 401 in
this sandbox was the environment, not the key.

## A verdict that summarises other measurements needs its own test

`queen verify-issue` says "goal already reached" when an issue's targets are
already true. It was wrong three times, each time in a new way, and each time
found by checking its answers against the compiler by hand:
1. it counted only the checks it could RUN, so two specs that do not parse -
   whose gen commands fail - were called finished;
2. after that fix it still counted only targets that quote a "today" value, so
   an issue whose first criterion is `... prints 0` with no today clause was
   called finished with 8 unimplemented bodies in the file;
3. a `t27c` call on a file that does not exist yet is a port issue's whole
   point, and reporting it as a stale claim condemned every open port issue.
A summary verdict is code like any other: give it cases (including the ones it
must NOT claim), and confirm a sample independently before acting on it. 26 of
28 "reached" issues survived that confirmation, and closing the other two would
have thrown away real work.

## Rewriting an issue does not free the row that was judged against it

The dispatch row freezes the issue's criteria, which is right - a bee is judged
by what it was told - and it means an issue you REWRITE is still held by a row
answering the old question. Measured 2026-09-19: after 126 issues were rewritten,
72 of the 86 rows still holding an issue carried the old text, the tick refused
673 candidates with "nothing to choose", and 0-2 bees ran.

`releaseStaleContracts` compares `queen_issues.criteria` with
`queen_dispatch.criteria` and releases the disagreements with no floor. First
round: 74 rows released, review column 180 -> 14, claimed 84 -> 12, bees 16/16.
Guards that matter: never re-open `accept`, never release when the issue board
is empty (the sync has not run and every row would look stale), and zero
`send_backs`/`free_attempts` because they counted attempts against a contract
that no longer exists.

## An honest refusal can still stop the swarm

2026-09-20, the morning after the adversarial review went live: 71 issues
claimed, 0 of 8 lanes running, 673 candidates refused with "nothing to choose".
Nothing was broken. The review was refusing work the compiler cannot build -
`invalid builtin function: '@noop'`, `@intCast must have a known result type`,
`expected ')', found ','`, `use of undeclared identifier 'to_string'` - and each
refusal spent one of two attempts. At the ceiling the row was `rejected`, which
had no clock, so it held its issue until the seven-day window dropped it.

Two lessons. First: when you make a judge stricter, count what happens to the
work it refuses, in the same change. Second: "a person decides, not a timer" is
right the second time and wrong the first, because the person is asleep and the
swarm is not. A spent ceiling is now handed back ONCE, an hour later, with the
last review's findings in the brief; `ceiling_releases` bounds it.

## The pusher: the reading nobody was taking

Three stalls in four days, each visible in a number nobody read (feeder fired
twice in twelve hours; merge task failing in three seconds for two days with no
log; 71 issues parked). `tools/queen/pusher.py` + `.github/workflows/pusher.yml`
take the reading every 30 minutes, compare it with the previous one - kept as
JSON inside its own issue (#4284), so the memory is public and nothing is
committed back to master - and file what stopped. Every rule names the command
that would falsify it, and the self-test includes three shapes a moving system
must NOT fire. When a swarm question comes up, read that issue first: it is the
cheapest true answer available.

## What the bees actually write, read off their own commits

Measured 2026-09-20 over the corpus they produced (948 specs, 518 IMPLEMENTED,
61 NOPARSE):

- They overwrite working work. `specs/tri/utils/utf8.t27` on master came from
  e71afdfc3 "Implement UTF-8 encoding and decoding functions" and does not
  parse; the version it replaced did. Same shape in `bytes`, `trie`, `csv`,
  `hex`, `quick_sort`.
- They write other languages. In `.t27` files: `println!`, `format!`,
  `Result<T, E>`, `use a::b::*`, `trait`, `struct AccountID(str);`,
  `catch unreachable`, `for (0..n) |i|`, `@constCast`. The parser accepts eight
  top-level forms: `const`, `var`, `fn`, `enum`, `struct`, `test`, `invariant`,
  `bench` (compiler.rs `parse_top_level_decl`).
- They collide. 4e57f31a7 and f3efbd712, two minutes apart on 2026-09-15,
  declared the type `Regex` in two files; the types ratchet has counted it since.
- They edit and do not commit: 16 of 32 reviewed rows in one night needed the
  salvage commit to have anything to judge.
- Overnight verdicts once the review became real: 20 sendBack, 7 empty,
  4 escalate, **1 accept**, with oracle errors like `invalid builtin function:
  '@noop'` and `@intCast must have a known result type`.

Read that before calling their output "implementations". The honest description
is pattern-matched code in a language nobody checked against, for as long as
nothing measured it.

## The roster, and the two outcomes the system cannot express

`~/queen-patches/AGENT_ROSTER.md` holds the full table: producers
(`implementer`, `porter`, `repairer`, `oracle-fixer`, `test-writer`,
`deduplicator`, `documentarian`), judges (`adversary` and `criteria-runner` are
live; `reward-hacking-auditor` and `scope-auditor` are not), and meta
(`pusher` is live).

The largest gap is not another producer. It is that a bee cannot say **"this
cannot be done as specified"**: an impossible task looks exactly like a lazy
one - two refusals, the ceiling, a parked issue. 126 issues carried criteria
the container could never satisfy, and 51 specs ask for constructs the language
does not have. The outcome needs a block the bee can write (`## IMPOSSIBLE`
with a reason), a review state that spends no budget and does not re-dispatch,
and a curator that clusters the reports into compiler work.

## Do not exhaust the host you are measuring from

2026-09-20: after several workflows (43, 28 and 23 agents) plus repeated
`railway logs` tails and `bun test` runs, the Mac ran out of process slots.
`fork()` failed for every subsequent command - Bash, the terminal panel and the
browser pane all returned EAGAIN - so the session could no longer measure,
test, commit or deploy anything, with work half-finished.

Rules that follow: cap workflow fan-out (ten agents is a lot on a laptop), never
leave a streaming log tail running, prefer one measurement pass whose output you
save to a file over ten live queries, and when a long agent run ends, check the
shell still answers before starting the next one.

## DRY, measured: 576 of 4021 bodies were written twice

A walk over `specs/` comparing function bodies with comments and whitespace
normalised away: **576 of 4021 (14.4%) are byte-identical copies**, in 165
groups. `magadd` is written 30 times, `magsub` 30, `sadd` 29, `magmul` 21.
Verified by hand: `specs/ternary/gft_axpy.t27:11` and
`gft_bitnet_neuron.t27:12` are the same 935 characters.

The cause is structural. A bee's brief named the file, the signatures and the
criteria and said nothing about the twenty-nine implementations that already
existed, and there was no command it could run to find out. So:

- `python3 tools/dupe_scan.py --name <fn>` - where it already lives (file:line).
- `python3 tools/dupe_scan.py --like <spec>` - what in this file is elsewhere.
- `t27c dupes --bodies` / `--name` - the same questions from the compiler,
  comparing parsed children rather than text (686 functions in 189 groups over
  the 887 specs that parse; the text walk covers all 948, including the 61 that
  do not parse - state both scopes rather than reconciling them).
- `Duplicate Body Ratchet` fails a pull request that adds a new copy; the ledger
  `tools/duplicate_bodies_baseline.txt` moves DOWN only. gf4..gf64 share a
  `validate_format` by design, so a gate demanding zero would be switched off
  rather than obeyed.
- The feeder's preamble now carries those two commands in every issue it writes.

Two mistakes worth keeping: hashing `format!("{:?}", node)` includes `line`, so
identical bodies at different lines hashed differently (276 reported against a
text walk's 555); and a `HashMap`'s order made two runs of the same command list
groups differently, which no reader can diff. Sort, and leave position out of
any identity.

## Keep the instruments somewhere the OS will not empty

The pinned judge, its provenance record, the oracle tree and every measurement
worktree lived under the session scratchpad in `/private/tmp`. On 2026-09-20 the
system cleaned it mid-cycle: `t27c` gone, `queen judge` unable to answer, the
verify worktree orphaned from a parent that no longer existed. The clone under
`~/queen-patches/work/t27-master` survived and is now where `queen` points.

Nothing about a judged verdict may live in a directory the OS treats as
disposable. If a rebuild is needed: `cargo build --release -p t27c` in that
clone, then `ln -sf .../target/release/t27c ~/queen-patches/work/bee-bin/t27c`.

## What none of this proves

There is no t27 interpreter. A `test` block runs only as generated Zig, under
the oracle or the review witness. The compiler
can say a spec parses, lowers, and has no empty bodies; it cannot say the
algorithm is right. When reporting swarm output, say "165 bodies closed" and not
"165 correct implementations" — the second is a claim nobody here can check.

## The lane count is derived from what is connected (since 2026-09-20)

`docker-entrypoint.sh` computes it at every boot and exports it, so the Swift
policy and the TypeScript that reports capacity read one number:

    lanes = distinct worker credentials x TRIOS_QUEEN_WORKER_LANES_PER_KEY  (1..4)
    bees  = min(lanes, container memory / TRIOS_QUEEN_BEE_MEMORY_MB)

A bare `TRIOS_QUEEN_MAX_WORKERS` is IGNORED and logged as ignored; only
`TRIOS_QUEEN_MAX_WORKERS_CEILING` caps the derivation. Before that change it was
the whole cap, defaulting to **four** when unset, and it read 8 on 2026-09-20
while twenty lanes were connected — a number that had been 4, then 16, then 8,
with no commit in either repository to show for any of it.

**Something rewrites that variable to exactly `9`, within about forty-five
seconds, every time it is set.** Measured four times on 2026-09-20. Not the peer
Claude session (asked), not a shared variable, not `railway.json`, not a
workflow in either repository, not launchd or cron on this Mac, and the
agent-server holds no Railway token. Every write also triggers a redeploy, which
SUPERSEDES an in-flight `railway up` — so deploy while leaving the variable
alone, and let the derivation ignore whatever it says.

Ten credentials at two lanes each is twenty bees, measured live: capacity 20 and
18 running while the variable said 9. Read all three before believing any:

```
railway variables --service trios-agent-server --kv | grep -c '^TRIOS_QUEEN_WORKER_API_KEY'
railway variables --service trios-agent-server --kv | grep '^TRIOS_QUEEN_WORKER_LANES_PER_KEY'
curl -s $HOST/queen/status | python3 -c "import json,sys;print(json.load(sys.stdin)['workers'])"
```

Count credentials by DIGEST, never by printing them: two variables holding one
key are one credential, and the deployment had ten distinct values in ten
variables.

**The service IS connected to GitHub - to `fix/queen-worker-provider-and-prompt-size`.**
The earlier note here said it was not, on the evidence that it has no
`RAILWAY_GIT_*` variables; that evidence was wrong. `railway deployment list
--json` carries `meta.branch` and `meta.commitHash` for every deploy. Every
variable change rebuilds from that branch's tip, so merging into
`feat/queen-supervisor` changes nothing until the tracked branch moves too, and a
`railway up` upload survives only until the next variable write. Fast-forward the
tracked branch (`gh api --method PATCH .../git/refs/heads/<branch> -f sha=... -F
force=false`) after every merge you want in production. A deploy is `railway up`
from a checkout of that branch — clone it, `railway link --project
564d9ebd-7aa8-44fe-93ec-e0b03c87158d --environment production --service
trios-agent-server`, then `railway up --service trios-agent-server --detach`
from the repository root.

(The project is linked from `~/trinity`, not from `~/t27` or `~/queen-patches`.)

A variable change redeploys the service. The running bees are lost, `active`
drops to whatever reconnects, and the swarm refills over the next two ticks —
measured 2026-09-20: 2 of 10 at 17:16:03, 10 of 10 at 17:16:48.

## A watchman that only asks "has it stopped?" cannot see a half-empty swarm

Every rule in `tools/queen/pusher.py` began `workers_active == 0`. Two bees in
ten lanes — 80% idle, 221 issues waiting — fired nothing. Partial idleness is
the normal shape of a productivity collapse; total idleness is the rare one.

Four readings it now takes: `lanes-idle` (two or more empty lanes while work is
dispatchable), `fuel-runway` (fewer dispatchable issues than lanes, **while
still running**), `capacity-shrank` (the lane count fell between readings) and
`throughput-down` (finished-per-hour halved). Rates come from the server's own
`queue.observedAt`, never the runner's clock: a scheduled job that starts late
otherwise reads as a swarm that slowed down. A finished counter that went
BACKWARDS is a restart, not a rate.

## A tank is refilled while the engine runs

The first version of that loop had a hole in the middle: `fuel-runway` fires at
ten of ten lanes, dispatches the feeder, and the feeder answered `swarm busy -
nothing added` because `queue_idle()` asked only whether lanes were free. Ten of
ten is when the queue empties fastest. Both feeders now take `--runway N` and
top the queue up to N dispatchable issues whatever the lanes are doing.

When a watchdog triggers a repair, run the repair path end to end before
believing the loop closes. Both halves passed their own tests.

## What the swarm can actually be given, measured

- **empty function bodies** — drained. `28 files hold 146 empty bodies; 100 files
  covered by open issues; 0 uncovered` on 2026-09-20.
- **untested functions** — 887 specs answer `t27c coverage`, 278 have at least
  one untested function, **1427 functions have none**, and 236 of those specs
  have between one and eight, which is one issue each. 221 were uncovered.
- **vacuous tests** — NOT fuel. 2243 of 7198 look vacuous, and outside
  `specs/igla/race/**` there is exactly **one**. The pool is 28 generated
  batch-protocol files carrying 80 identical `{ /* verify baseline */ }` bodies,
  already baselined as debt.
- **unparseable specs** — 61, and the tool that files them still lives only in
  `~/queen-patches/queen_repair_fuel.py`.
- **duplicate bodies** — 576, blocked on #4298 until `use a::b;` emits an import.

Of 673 open issues only **119 carry a `## Boundary`**, and an issue without one
can never be dispatched. Of the 554 without: 66 name exactly one spec, 36 name
several, 452 name none — they are decisions and compiler defects, not spec work.

## Two commands in one binary can answer the same question differently

`t27c lint` warns `has no test or invariant` about functions whose tests call
them by name. Over 60 specs: **677 lint warnings against 190 untested functions
from `t27c coverage`, disagreeing on 59 of the 60.** `coverage` is the one that
is right (checked by hand on `specs/base/ternary_encoding.t27`, where lint warns
about `bit_to_trit_pair` and `test bit_to_trit_pair_zero` calls it on line 249).

Never write an acceptance criterion on an instrument you have not cross-checked
against a second one. A criterion on `lint` would ask a bee to write twelve
tests that already exist.

## Adding a workflow moves a census, and the census gate is not optional

`tri census pin --gate` fails when `tools/census/*.txt` disagrees with the tree.
Six workflows were added across #4277, #4279, #4281, #4283, #4291 and #4300 and
none re-blessed, so `cli-tri` was red on master from 2026-09-17. Any PR that
adds or removes a workflow must run `tri census pin --bless` **in the same
commit** and name every number that moved in the message.

## A worktree's `.git` is a file, and gates that exclude the directory still see it

`check_devhome_paths.py` passed `--exclude-dir=.git` to grep and failed in every
worktree on `gitdir: /Users/...` — a path no commit contains. The reviewer
measures in a detached worktree and every bee works in one, so a gate that is
wrong there is wrong where the swarm lives. `--exclude=.git` belongs beside it.

## macOS has no `timeout`

A measurement harness that wraps every probe in `timeout 60` returns `rc=127`
for all of them, which reads as "every command is broken" rather than "the
harness is". The bee container is Linux and has it; the machine you measure
from may not.

## A service that stops answering is not restarted by its restart policy

Railway's `restartPolicyMaxRetries` is a budget: past it the platform stops
restarting. It was 10; it is 10000 now. But on 2026-09-21 the service went down
again at twenty lanes and did NOT come back by itself even with that budget -
the process had stopped answering HTTP without exiting, and a restart policy only
fires on an exit. It answered `{"status":"error","code":502,"message":"Application
failed to respond"}` for nine hours the night before with nothing reading it.

`tools/queen/watchdog.py` (every five minutes in CI) asks the one question no
other instrument asks - does it answer at all - and is the only thing allowed to
redeploy. It needs `RAILWAY_TOKEN`, `RAILWAY_SERVICE_ID`, `RAILWAY_ENVIRONMENT_ID`
as repository secrets; the two IDs are set, the token has to come from the
Railway dashboard. Without it the watchdog alarms and says it restarted nothing.

## Finished bees leak worktrees until the disk is full

`removing 27 bee worktree(s) left by a previous container (25948 MiB free)` ->
`33692 MiB free now`, at boot, after a crash. The emergency sweep in
`reapWorktrees` (fires at `QUEEN_VOLUME_HIGH`, default 80%) kept every one of
them because each branch carried commits the base did not, and the container
has no push credential - "the only copy". True until the branch reaches origin.
The sweep now runs `git ls-remote origin refs/heads/<branch>` and removes a
worktree whose commit the remote already has.

## Twenty lanes kills the container; ten does not (unresolved)

Measured 2026-09-21: at ten lanes, stable for twenty minutes and more. At twenty,
down within about eight minutes, twice, with no OOM line in the log - the log
simply stops mid-stream. Twenty ran for an hour on 2026-09-20 before the worktree
leak was found, so memory alone is not the answer. Running at ten until the cause
is pinned; the ceiling is `TRIOS_QUEEN_MAX_WORKERS_CEILING`, which the derived
entrypoint honours, and `TRIOS_QUEEN_MAX_WORKERS` set to the same number covers
an older image.

## Every variable write redeploys, and a redeploy can resurrect an older image

Railway rebuilds nothing on a variable change; it redeploys. Twice on 2026-09-20
that brought back an image without the derived lane count, where
`TRIOS_QUEEN_MAX_WORKERS` - which something outside both repositories keeps
rewriting - governs again. A variable change during a `railway up` build also
SUPERSEDES the build (`REMOVED`). Deploy with `railway up` and leave variables
alone until it reports SUCCESS; check `[entrypoint] ... (derived)` in the log to
know which image is running.

## The publisher must never switch branches in place

It ran from a checkout carrying its own `publish.py`, `git checkout -B` switched
branches in place, and the file travelled into a bee's branch - which then could
not merge (#4338). It now commits in a worktree cut from the bee's own branch.
And it refuses a bee branch that adds a workflow: that moves the census and the
gate-topology ledger, which no bee can know about.

## Worker model is ranked, not typed (2026-09-21, BrowserOS #496)

- `TRIOS_QUEEN_WORKER_MODEL` is the primary; `TRIOS_QUEEN_WORKER_MODEL_CANDIDATES` (comma list) turns on ranking. Unset = old behaviour.
- Score = expected seconds per 500-token step = `(500 / tok/s) / successRate`. Inputs: probes every 5 min (speed, tool call, 404/410) + every live bee answer (retry wrapper `onOutcome`). Window 15 min, Laplace-smoothed.
- Eligible only after it called a tool in a probe; challenger must be 20% cheaper; gone models re-probed hourly.
- Read it: `curl -s .../queen/status | jq .models`; logs: `Queen worker models ranked`, `Queen worker model switched`.
- Measured: super-120b 66 tok/s at ~60% beats ultra-550b 10 tok/s at ~80%. Switching models does not fix 429 (per-key rate limit) - lanes per key does.
- NVIDIA retires models without notice (gpt-oss-120b, qwen3-coder-480b, kimi-k2, llama-3.3 -> 410 on 2026-09-21). List live ones: GET /v1/models.

## GitHub schedules are not a clock (measured 2026-09-21)

- queen-watchdog.yml (`*/5`) ran ONCE in 5.5h after it was added; publish/pusher/feeders fire 2-5h apart, while the Actions queue held 3 queued runs. GitHub drops scheduled runs silently.
- So: CI crons are best-effort backstops. Self-recovery must live in the container (liveness supervisor); anything time-critical gets `workflow_dispatch` from a running loop.
- queen-publish failed every run 2026-09-20 19:02 -> 2026-09-21 because `gh_json` parsed stderr+stdout and swallowed the cause (#4514/#4515). When a swarm workflow is red, read the log BEFORE trusting its one-line message; check `gh run list --workflow <wf>` for all queen-* workflows each loop.
- `tri swarm [--since N]` (#4513): lanes, finished/hour and the model ranking in one screen.

## Reading throughput (2026-09-21)

- `dispatches.total` counts ROWS, one per issue; a re-dispatch reuses the row. It stood at 955 for 40 min of real work. Use `dispatches.finishedLastHour` / `dispatchedLastHour` (BrowserOS #498) or `tri swarm`.
- Refusals by cause per model (`models.ranking[].refusals`): super-120b 75x429 in 15 min vs ultra-550b 89% of 221 calls answered at 63 tok/s. The ranking moved the swarm to ultra on its own. Model choice holds 3 min (anti-flap).
- The in-container liveness supervisor now prints /proc forensics (state, cpu, children) at miss 3, midway and last (#497): read `[liveness]` lines after any restart before guessing a cause.
- Local publish loop (while GH_AGENT_TOKEN is 401): `TRI_T27C=~/.local/bin/t27c python3 -u tools/queen/publish.py --limit 5` from a scratch t27 worktree; 4 of 4 bee PRs published 11:47Z merged within 15 min.

## Reviews need a lane the bees do not take (2026-09-21, BrowserOS #499)

- Reviews used the bees' lane arithmetic; bees fill every lane, so the sweep answered "no reviewer lane is free" every round and logged NOTHING (a skipped row is silent). 44 finished turns waited. Lowering LANES_PER_KEY to 1 made it total.
- Now: `TRIOS_QUEEN_REVIEW_EXTRA_LANES` (default 1 per key; 0 on z.ai, which answers 1302 to a 3rd concurrent request).
- Read the review queue from `/queen/status`: `dispatches.reviewable` (rows the sweep will take) vs `unreviewed` (all finished rows without a verdict, incl. reaped/closed) and `reviewedLastHour`. If reviewable > 0 and reviewedLastHour == 0, the sweep is starved - look for a lane/budget cause before blaming the model.
- Any lane change: check what ELSE shares those lanes (reviews, probes) before shipping it.

## Rate limits are per key AND model (2026-09-21, BrowserOS #502)

- 13:25Z: super-120b 826 x 429 / 15 min (27% ok) while ultra-550b on the SAME keys drew none. So a key refused for one model still has the other's quota.
- `ModelRanking.routeFor(keyTag)`: a (model, key) pair that drew a 429 is avoided for 60 s; that key's next request goes to the best measured model it wasn't refused for, after 250 ms instead of the backoff. 503 does not spill (model-wide).
- Consequence: more candidate models = more quota per key. Lanes per key can go up as long as there are models to spill to; judge by finishedLastHour, not by 429 counts alone.
- The 209 "waiting for review" rows at 13:23 were ALL on closed issues (`unreviewableClosed`); `reviewable` was 0. Read `reviewable`, not `unreviewed`.

## NVIDIA catalogue ceiling (measured 2026-09-21 13:50)

- Of 81 listed models only super-120b (71 tok/s) and ultra-550b (9-63 tok/s depending on load) are usable. Listed-but-404: mistral-large(-2), codestral, gemma-3, kimi-k2.6, deepseek-coder. Timeout: glm-5.3(-flash), kimi-k3, nemotron-3.5-lightning. gpt-oss-20b calls tools but at 6 tok/s.
- So per-key quota = super + ultra only. Candidates set to ultra only (plus primary). Beyond that, throughput needs either another provider's keys (a second pool: TRIOS_QUEEN_WORKER_POOL_<n>_*) or fewer LLM requests per bee turn.
- Probe requests now retry twice on 429/5xx (#503): one transient refusal used to leave ultra unscored and unable to take spills.

## Requests per bee turn (2026-09-21, BrowserOS #504)

- Each agent step = one model request; MAX_TURNS=100 steps per turn. On rate-limited keys requests are the ceiling.
- `Agent turn profile` log line per turn: steps, toolCalls, singleCallSteps, toolErrors, byTool. Read it before changing the brief or tools: `railway logs --lines 5000 | grep "Agent turn profile"`.
- ENOENT from filesystem tools now carries the nearest directory's listing + close matches (saves the follow-up ls).
- The brief has `## Your request budget` (batch independent tool calls, chain bash, no confirm-ls, stop when criteria answered).
- Next levers if single-call steps stay high: a multi-path read tool; trimming the browser-oriented system prompt for headless bees (tokens, not requests).

## Parallel lanes on the Queen actor runtime (2026-10-09/10, epics trios#1712 and #1729)

Seven lanes ran at once, each with a spec in t27, a card, the runtime in trios, tests and a benchmark. All seven landed in the integration branch `actors-next`. Every rule below was paid for in time.

- **Integration branch, one deploy.** A merge into trios `queen` is a production deploy with a 1800 s drain, and five merges in one morning meant five blackouts. Lanes PR into `actors-next`. `actors-next` goes into `queen` once, with the owner's OK, and every new behaviour sits behind a flag that is off by default.
- **`queen/<x>` branches cannot exist next to `queen`.** Git refuses a ref under an existing branch name. Every lane hit this. Name branches `queen-<x>`.
- **`Closes #N` only fires on the default branch.** trios' default is `main`, so lane issues stay open after merging into `actors-next`, and even after `queen`. Close them by hand with the merge link.
- **One dependency install, many worktrees.** trios agent-server deps are 2.2 GB. Use one shared clone, `git worktree add -b <branch> <dir> origin/actors-next` per lane, and symlink the 7 `node_modules` dirs. Never `bun install` per lane.
- **New rules go in new card modules.** `actors.wasm` is a binary: two lanes rebuilding it conflict, and git cannot merge them. Each lane added its own card (`telemetry`, `turn_stop`, `reviewer_sizing`, `waits`, `netlink`, `keyed`). The only conflicts left were PIN lines, and those are resolved by keeping both.
- **Card build trap.** Never pipe `zig cc` into `| head`: SIGPIPE kills zig on its warnings, and no wasm is written. Redirect stderr to a file. Build twice and compare hashes.
- **t27 hooks in an agent worktree.** The inherited `core.hooksPath` points at the main checkout's stale `.githooks`, which call `t27c hook-pre-commit`, a subcommand the current t27c lacks. Commit with `git -c core.hooksPath="$PWD/.githooks"` and `tri` on PATH. Never `--no-verify`.
- **u32 products do not wrap in t27.** `seed * 2654435769` in u32 makes a test FAIL instead of wrapping. Widen to u64 and take `% 4294967296`, as `actors.t27` `seed_hash` does. On hardware that is plain truncation.
- **Load makes timing tests lie.** At load 40 (7 lanes plus yosys plus zig), the 200 000-turn flood test and "a process takes its messages in order" fail 2 times in 15, on the base too. Compare suites against a baseline taken at the same load. Keep at most 4 agents at once. Take per-message cost only on a quiet host.
- **Benchmarks must model production's worst rows.** The first actors benchmark did not model unreadable rows. The go-live hot loop (184 visits per minute against 22 rows) appeared only in production. Each swap needs a production stage with the same counters as the simulation (telemetry, t27#8285).
- **Half the gain may not be actors.** Lane 7: the 60-minute turn bound gave the loop 349 against the actors' 352 on 16 lanes. Lane 3: adaptive concurrency loses with 3 lanes or small memory. Lane 2: stoppable turns cut overload throughput from 78 to 61 per hour, because the old code ran past its bound. Report the cases where the old mechanism wins.
- **A second backend for free.** The t27b lab signs a receipt for every master run (https://t27b-lab-production.up.railway.app/latest.json, `/runs/<sha>.receipt.json`). Check it with `t27c corpus-receipt compare R R` from a checkout that has `.trinity/keys`, and restore the keys dir afterwards. On 2026-10-10 every new queen spec passed on t27b with runtime asserts, with 0 reference disagreements.
- **Card constants: `gen-ts`, not `gen-js`.** A `*.gen.js` has no types. Every import of it is TS7016 in `@browseros/agent`'s typecheck, which reads server sources, and `export *` from it exports nothing to tsc (TS2305/2459). trios#1733 regenerated 20 cards with `t27c gen-ts`, with names and values identical. Typecheck went from 98 to 19 errors. Biome excludes `queen-*.gen.{js,ts}` by pattern, so a new card needs no edit to `biome.json`.

# Hot-Reload Steward

## Map

The steward monitors these seven SPECS for performance regressions against budget:
- `specs/tri/hot/hot.t27` - the `tri hot` command specification
- `specs/verified/digest_cache.t27` - digest cache performance
- `specs/verified/suite_jobs.t27` - suite job performance  
- `specs/verified/gen_hash_cache.t27` - hash generation cache performance
- `specs/verified/bitstream_reuse.t27` - bitstream reuse performance
- `specs/verified/watch.t27` - watch performance
- `specs/verified/bench_agent.t27` - benchmark agent performance

The steward implements the `tri hot` command budget:
- `tri hot` - the budget check: a warm frontier within 5000 ms, a bitstream reuse hit within 2000 ms
- `watch` - reruns, on save, the specs a save affects
- `frontier` - lists what can be reused instead of rebuilt
- `prebuild` - builds the bitstream ahead of the board run
- `suite` - the corpus suite, in parallel
- `caches` - each cache's size against its eviction limit

This work is driven by the hot-reload loop of 10-11 October (ledger #8737, epic #8095) and documented in the post at t27.ai/blog/from-a-minute-to-a-second/.

## One run, on master, in the container

1. Run `t27c test-report` on each of the seven SPECS:
   - `t27c test-report specs/tri/hot/hot.t27`
   - `t27c test-report specs/verified/digest_cache.t27`
   - `t27c test-report specs/verified/suite_jobs.t27`
   - `t27c test-report specs/verified/gen_hash_cache.t27`
   - `t27c test-report specs/verified/bitstream_reuse.t27`
   - `t27c test-report specs/verified/watch.t27`
   - `t27c test-report specs/verified/bench_agent.t27`

2. Run `t27c frontier` twice, timing the second run against the 5000 ms budget:
   - `t27c frontier` (warm-up)
   - `time t27c frontier` (measure against budget)

3. If `tri hot` is on PATH, use it instead:
   - `time tri hot`

4. For silicon reuse hit measurement, check for openXC7 toolchain (`T27_OPENXC7`). Where that is unset, report "silicon: not measured". Never write a pass for it.

5. Report the machine's load next to every time measurement.

## Report

1. One comment on #8737 per run, giving:
   - The current commit hash
   - The commands executed
   - Each time against its budget (5000 ms for warm frontier, 2000 ms for bitstream reuse)
   - Pass/fail counts for each spec

2. One new Queen-shaped issue per regression, with the template:
   ```
   ## Boundary
   [The file paths from the boundary section]

   ## User Scenarios
   [The user scenarios from the issue]

   ## Requirements
   [The requirements from the issue]

   ## Success Criteria
   [The success criteria from the issue]

   Refs #8737
   ```

## Never

- Never merge, approve or auto-merge.
- Never touch a board, a JTAG cable or flash, and never post `/bench approve`.
- Never edit `bootstrap/src/test_report.rs`, `bootstrap/src/cc_gate.rs` or `tools/policy/foreign-exceptions.txt`.
- Never hand-write code in any language other than t27.
- Never run builds on the owner's Mac.
- Never claim a speed-up without the load it was measured at.
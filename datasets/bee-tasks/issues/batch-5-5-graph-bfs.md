# Implement init, add_edge, traverse and deinit in specs/tri/graph/graph_bfs.t27

## Context

`specs/tri/graph/graph_bfs.t27` is the breadth-first traversal of the Trinity
graph library, sitting beside `graph.t27`, `graph_dfs.t27`, `dijkstra.t27`,
`bellman_ford.t27`, `prims_mst.t27`, `topological_sort.t27` and
`disjoint_set.t27` in `specs/tri/graph/`. It owns two types: a `Graph` holding
an adjacency list `adj : [][]usize` indexed by vertex id plus the allocator that
made it, and a `BFSResult` holding two parallel arrays — `order` (the visit
sequence) and `distance` (hops from the source) — plus its own allocator. The
module's six test blocks already spell out the properties the structures must
have: one adjacency row per vertex, an undirected edge written into both
endpoints' rows, the handshake lemma (degree sum equals twice the edge count),
an isolated vertex keeping a length-0 row, `order.len == distance.len` with the
source at distance 0, and BFS distances rising by at most 1 along the visit
order. All four functions are empty bodies, so none of it is produced by code
yet. Your job is to write those four bodies.

`.t27` is hand-authored source, compiled out to C/Rust/Verilog/Zig by `t27c`.
You are writing the input language.

## Exact current state

All commands run from `/Users/playom/t27`.

```
$ ./target/release/t27c parse specs/tri/graph/graph_bfs.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

```
$ ./target/release/t27c count specs/tri/graph/graph_bfs.t27 | head -1
graph_bfs.t27: 7 node types
```

```
$ grep -c 'TODO: Implement' specs/tri/graph/graph_bfs.t27
4
```

```
$ grep -cE '^[[:space:]]*test ' specs/tri/graph/graph_bfs.t27
6
```

The file parses clean today. Keep it clean, and keep all six existing test
blocks exactly as they are — they are the specification.

## What to write

Four functions. Signatures quoted verbatim — change only the bodies.

1. `fn init(allocator: std.mem.Allocator, vertex_count: usize) -> Graph`
   Allocate `adj` with exactly `vertex_count` rows, each an empty slice, and
   store the allocator in the struct. One row per vertex even when a vertex has
   no neighbours — that is `test adjacency_has_one_row_per_vertex` and
   `test an_isolated_vertex_has_an_empty_row`.

2. `fn add_edge(graph: *Graph, from: usize, to: usize) -> void`
   Undirected: append `to` to `graph.adj[from]` **and** `from` to
   `graph.adj[to]`, growing both rows through `graph.allocator`. Writing only
   one direction breaks both
   `test an_undirected_edge_appears_in_both_endpoints_rows` and
   `test the_degree_sum_is_twice_the_edge_count`.

3. `fn traverse(graph: *Graph, start: usize, allocator: std.mem.Allocator) -> BFSResult`
   Standard BFS with an explicit FIFO queue and a visited marker. Allocate
   `order` and `distance` from the `allocator` argument (not the graph's), set
   `distance[start] = 0`, and push neighbours in adjacency-row order so the
   result is deterministic. Return a `BFSResult` whose `order` and `distance`
   are the same length and whose `allocator` field is the one passed in — that
   pair of facts is `test bfs_result_holds_one_distance_per_vertex_and_starts_at_zero`.
   Because BFS explores by layer, the distance must rise by at most 1 between
   consecutive entries of `order`, which is
   `test bfs_distances_never_jump_by_more_than_one`.

4. `fn deinit(graph: *Graph) -> void`
   Free every adjacency row and then the row array itself, through
   `graph.allocator`. This is the file's own header comment: "Free graph
   memory". `BFSResult` carries its own allocator for the caller to free
   separately; note that in a comment.

### The tests are the specification

One of the six, quoted verbatim, to show the style the file uses:

```
    test the_degree_sum_is_twice_the_edge_count
        // Verify: the handshake lemma on the path 0 - 1 - 2 — two edges, so the
        // row lengths add up to four
        given row0 = [_]usize{ 1 }
        and row1 = [_]usize{ 0, 2 }
        and row2 = [_]usize{ 1 }
        and rows = [_][]usize{ @constCast(&row0), @constCast(&row1), @constCast(&row2) }
        when graph = Graph{ .adj = @constCast(&rows), .allocator = std.testing.allocator }
        and degree_sum = graph.adj[0].len + graph.adj[1].len + graph.adj[2].len
        then degree_sum == 4
```

Every existing test builds its graph by hand because no constructor worked. Add
at least one new `test` block per implemented function that goes through the
functions instead: `init(alloc, 3).adj.len == 3`; the path 0-1-2 built by two
`add_edge` calls reproducing the same both-rows shape; `traverse` on that path
from vertex 0 giving `order.len == distance.len` and `distance[0] == 0`; and a
`deinit` block stating the ownership rule. Do not delete or rewrite the six
existing blocks.

**There is no t27 interpreter.** Test blocks are parsed and counted, never
executed. Do not claim a test passed.

## Acceptance criteria

**Every criterion below is a command with an expected output, and every one of them
fails on the file as it stands today.** The `now` column was produced by running the
command in this repository on 2026-09-09. Re-run them before you start; you will get
the same numbers, and that is the point — a criterion that already passes before any
work is done proves nothing.

- 1. all 4 named functions still exist, every function body is non-empty, and `t27c parse` reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0
- 2. `grep -c 'TODO: Implement' specs/tri/graph/graph_bfs.t27` prints 0
- 3. the real-code node count is at least 8
- 4. `t27c parse specs/tri/graph/graph_bfs.t27 | grep -c 'kind: TestBlock'` prints at least 10
- 5. `t27c count specs/tri/graph/graph_bfs.t27 | head -1` reports at least 11 node types

| # | check | now (before you start) | required when you are done |
|---|-------|------------------------|----------------------------|
| 1 | signatures + non-empty bodies + parse | `4 names, fns=4 nonempty=0, clean` | `4 names, fns=n nonempty=n (n>=4), clean` |
| 2 | stub marker | `4` | `0` |
| 3 | real-code nodes | `0` | `>= 8` |
| 4 | test blocks | `6` | `>= 10` |
| 5 | node types | `7` | `>= 11` |

### The exact commands (run from `/Users/playom/t27`)

**Criterion 1** is three commands that must all hold at once. It is one criterion
because each half is worthless alone: the signature grep alone passes today, and a
file with every function deleted would pass the body scan.

*1a — the 4 signatures survive.* Deleting or renaming a function fails here.

```
$ grep -cE '^[[:space:]]*(pub[[:space:]]+)?fn (init|add_edge|traverse|deinit)\(' specs/tri/graph/graph_bfs.t27
```

Must print `4`. It prints `4` today — this half is a guard against regression, not
a task.

*1b — every body is real.* This counts declared functions and how many have a body
that is more than comments. Deleting a `TODO` comment and leaving the braces empty
does not move it.

```
$ python3 - specs/tri/graph/graph_bfs.t27 <<'EOF'
import re,sys
s=open(sys.argv[1],encoding='utf-8').read(); n=e=0
for m in re.finditer(r'^[ \t]*(?:pub[ \t]+)?fn[ \t]+\w+',s,re.M):
    i=s.find('{',m.end())
    if i<0: continue
    d=0
    for j in range(i,len(s)):
        if s[j]=='{': d+=1
        elif s[j]=='}':
            d-=1
            if d==0: break
    n+=1
    if re.sub(r'//[^\n]*','',s[i+1:j]).strip(): e+=1
print('fns=%d nonempty=%d'%(n,e))
EOF
```

Today it prints `fns=4 nonempty=0`. The two numbers must be **equal**, and the first
must be **at least 4**. A private helper function you add is welcome — it raises both
numbers together and 1a still pins the 4 public names.

*1c — the file still parses clean.*

```
$ ./target/release/t27c parse specs/tri/graph/graph_bfs.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
```

All three must read `0`. Use this `grep -E` form and never `| tail -3`: on a file with
recovery events the parser prints `discarded:` diagnostic lines *after* the three
metrics, so `tail -3` shows diagnostics and makes a broken file look clean.

**Criterion 2 — the stub marker is gone.**

```
$ grep -c 'TODO: Implement' specs/tri/graph/graph_bfs.t27
```

Today it prints `4`. It must print `0`.

**Criterion 3 — real code, not prose.** Comment-only bodies produce no statement or
expression nodes at all. This counts the node kinds only a real body can produce:

```
$ ./target/release/t27c parse specs/tri/graph/graph_bfs.t27 2>&1 | grep -cE 'kind: (StmtLocal|ExprReturn|ExprCall|ExprBinary|ExprIndex|ExprFieldAccess)'
```

Today it prints `0`. It must print at least `8`.

The bar is two nodes per stubbed function (4 x 2 = 8 on top of the 0 already
present). For scale: 88.0% of the stub-free, parse-clean files in `specs/` carry at
least 2 such nodes per function and the median is 8.8, so this is a floor, not a
target.

**Criterion 4 — a test block for each function you implement.**

```
$ ./target/release/t27c parse specs/tri/graph/graph_bfs.t27 2>&1 | grep -c 'kind: TestBlock'
```

Today it prints `6`. It must print at least `10` — the 6 blocks already in the file
plus one new block for each of the 4 functions you implement. Counting through
`t27c parse` rather than `grep 'test '` over the source is deliberate: only blocks the
parser actually accepted are counted, so a malformed test block scores nothing.

**Criterion 5 — the file became structurally richer.**

```
$ ./target/release/t27c count specs/tri/graph/graph_bfs.t27 | head -1
```

Today it prints `graph_bfs.t27: 7 node types`. It must report at least `11`. For scale, 77.7% of
the stub-free, parse-clean files in `specs/` are at or above that number, and the
median across them is 17.5.

`| head -1` is not optional. The rest of the `t27c count` table is **not reproducible**:
rows with equal counts print in a different order on every run (measured: 3 distinct
orderings in 8 consecutive runs over the same unchanged file), and the table stops
after 15 rows with `... and N more`. 14 files in `specs/` contain test blocks that
`t27c count | grep TestBlock` cannot see for exactly that reason — and they are the
richly-implemented ones, which is the state this task is trying to reach. Never read
a number off that table; the first line is the only stable part of the output.

### What is not evidence

- **`t27c typecheck` is not a gate.** It returns `{"ok":true,"errors":0}` on 500 words
  of English prose and on a Python file. Do not cite it for anything.
- **There is no t27 interpreter.** Your test blocks will be parsed and counted, never
  executed. Write them — a `test` block beside a function is how this corpus states
  what a function must do — but never claim you ran one or that one passed.

### Your VERDICT

The first thing in your final message must be the block below, with each criterion in
the issue's own words on one line, ending in exactly `met`, `unmet` or
`could-not-check`:

```
## VERDICT
- 1. all 4 named functions still exist, every function body is non-empty, and t27c parse reports recovery-events 0, declarations-swallowed 0, lexer-discarded-chars 0: met
- 2. grep -c 'TODO: Implement' specs/tri/graph/graph_bfs.t27 prints 0: met
- 3. the real-code node count is at least 8: met
- 4. t27c parse specs/tri/graph/graph_bfs.t27 | grep -c 'kind: TestBlock' prints at least 10: met
- 5. t27c count specs/tri/graph/graph_bfs.t27 | head -1 reports at least 11 node types: met
```

## Scope

That file and nothing else. Every other issue in this set owns a different file, and
two agents editing one file is the exact failure the worktree isolation exists to
prevent. Do not edit any other `.t27` file, do not touch `compiler/`, do not add new
files, and do not create any `.sh` or `.bash` file.

## Boundary

specs/tri/graph/graph_bfs.t27

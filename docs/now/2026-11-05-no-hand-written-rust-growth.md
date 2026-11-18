# Lesson: a compiler capability cannot land as hand-written Rust seed growth

Date: 2026-11-05. Trigger: three attempts to land t27b/compiler work all hit
the same wall (#8236), because each grew hand-written Rust instead of porting
logic to `.t27` and generating it.

## The three failures (same root cause)

| Attempt | Net foreign | Why blocked |
|---|---|---|
| t27b activation (mount + retry loop + call-site) | +19 | hand-written glue |
| cache-format port (escape/encode/decode) | ~0 | enum↔bytes glue eats the savings |
| gen-rust Milestone 1a (collect_type_params free-fn `<T>`) | +24 | grows the L0 seed `bootstrap/src/compiler.rs` |

All three are technically correct (probe green, tests green, seal updated for
1a) but none is landable under `#8236`: "no hand-written code in any language
but t27 may grow; port it to .t27 and generate it; no exception and no label
lifts this".

## The legal pattern (and its illegal inverse)

```
LEGAL:
.t27 implementation -> generated Rust -> remove equivalent hand-written Rust

ILLEGAL:
add feature in Rust -> delete unrelated Rust -> call it neutral
```

Deleting 24 arbitrary lines of `compiler.rs` to "offset" a +24 feature is
compensation accounting, not a port. Every deleted line must be the result of
ported-and-generated functionality.

## The route

A compiler capability lands as:
1. represent the capability in `.t27` (parser/typecheck/lowering rules),
2. generate the Rust,
3. remove the equivalent hand-written Rust,
4. preserve or improve net foreign budget.

For compiler backend work this is `#5980` (t27core self-compilation): the
L0 seed `bootstrap/src/compiler.rs` only shrinks as its logic moves into
`specs/compiler/core/t27core.t27` and is generated.

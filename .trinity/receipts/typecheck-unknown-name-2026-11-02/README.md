# Receipt: `t27c typecheck` accepts unknown names (2026-11-02)

Four minimal specs, one command each. Three use a name that is declared
nowhere; `control.t27` uses a declared one (negative control).

Binaries (the main clone's release builds, not built from this branch):

| binary | version | sha256 (first 16) | mtime |
|---|---|---|---|
| `~/t27/target/release/t27c` | t27c 0.5.2 | 041fb69cd5fe45b2 | 2026-11-01T17:40 |
| `~/t27/target/release/t27b` | t27b 0.1.0 | ee89c7d27724dc11 | 2026-11-01T20:19 |

Commands, run from this directory:

```
~/t27/target/release/t27c typecheck <file>.t27
~/t27/target/release/t27b asm <file>.t27
```

| file | sha256 (first 16) | unknown form | `t27c typecheck` | `t27b asm` |
|---|---|---|---|---|
| `control.t27` | 77bba15b0955a80c | none (declared `KNOWN`) | `Typecheck OK (0 errors, 0 warnings)` | rc 0 |
| `bare.t27` | cc128f6328d2288f | bare identifier `NO_SUCH_NAME` | `Typecheck OK (0 errors, 0 warnings)` | rc 2, `ExprIdentifier(undeclared) at line 4 (unknown name NO_SUCH_NAME)` |
| `call.t27` | bb49a955f3682fab | call `no_such_fn(1)` | `Typecheck OK (0 errors, 0 warnings)` | rc 2, `ExprCall(undeclared fn) at line 4 (call to no_such_fn)` |
| `field_base.t27` | 325381b7ecd845ac | field on unknown base `NO_SUCH_NAME.runtime_ms` | `Typecheck OK (0 errors, 0 warnings)` | rc 2, `ExprFieldAccess at line 4 (NO_SUCH_NAME.runtime_ms)` |

What this shows, and only this: in a function body's return expression,
`t27c typecheck` does not resolve identifiers, callees or field bases, so its
"OK" says nothing about name resolution. `t27b asm` does resolve them and
refuses each unknown name.

Not covered: module-level `const` initialisers, test bodies, `use` of a
missing module, and whether `t27c test-report` (the zig path) catches these at
build time. Each needs its own one-file repro before it is claimed.

Consequence for review: a `t27c typecheck` OK is not evidence that the names in
a spec exist. `t27b asm` rc 0 is (for the forms above).

Refs #12877.

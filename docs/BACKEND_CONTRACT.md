# Backend contract — Zig, C, Verilog

**Status:** Normative skeleton (refine per ADR and ring)  
**Goal:** State what **must be preserved** when projecting `.t27` to each backend.

---

## 1. Shared obligations

Each backend **must**:

- Emit only **generated** output (no hand-edited golden files in `gen/`).  
- Preserve **observable behavior** defined by the spec for the **declared fragment** (as `LANGUAGE_SPEC.md` will delimit).  
- Include a **header** marking auto-generation (validated by `tests/validate_gen_headers.sh`).

---

## 2. Zig

- **Module layout:** Mirror spec paths under `gen/zig/`.  
- **Build:** `compile-project` may emit `build.zig` for coherent projects.  
- **Allowed deviation:** None for **stable** specs once round-trip CI is enabled.

---

## 3. C

- **Linkage:** Headers and sources paired predictably.  
- **Numeric behavior:** Must match GoldenFloat / integer models **as specified** for the fragment; document any platform assumption.
- **Slices (#11183):** a `[]T` parameter, field or local lowers to `T*` and the length is not carried, so `x.len` on a slice is not C. Whether a slice becomes a (pointer, length) pair or the C back end refuses slices is an open decision (D4 of #11183); this section changes when it is taken.
- **Range slice:** `a[lo..hi]`, `a[lo..]` and `a[lo:hi]` are slices, not elements: the address of element `lo`, written `(a + lo)`, the pointer a `[]T` parameter takes. The upper bound is dropped where D4 drops every length. A slice local with a declared type is `T* x`.
- **Untyped array-literal local or module constant:** `var a = [x, y, z]` declares `T a[N]`, where `T` is the literal's own type, the type a list of numbers or of calls to one function already gets, or the type of the first element that is not a number (`__typeof__`, so a number beside it coerces to it, as in Zig). C has no inference for a brace list, so `__auto_type a = { .. }` is never written for these. Not yet lowered: a list of one name or number, `[v] ** n` without a type, a nested list, an empty list, `_ = [..]`, `return [..]`.

---

## 4. Verilog

- **Synthesis subset:** Document what is synthesizable vs simulation-only.  
- **Deviations:** Timing annotations may differ; **logical** behavior per spec tests.
- **Pointers and slices (#11183, D7, open):** RTL has no address space, so a field read or written through a `*Struct` or `[]T` parameter (`v.count`, `v.len`) has no hardware form. `gen-verilog` still writes it as the name `v_count`, which nothing declares, and `iverilog` cannot elaborate the module. Refusing the spec would move 608 tracked specs from passing to failing `gen-verilog` (see #11218); skipping the function with an `// ERROR:` comment, as is done for an unresolved array parameter, would change the output of the same 608.

---

## 5. Equivalence (roadmap)

**Ring 39 target:** same conformance corpus, **bit-exact or tolerance-documented** outputs across backends — dashboard TBD.

---

## 6. Violations

Breaking this contract without ADR + ring tag **`[GOLD-RING]`** is **not allowed** for stable specs.

---

*Backends are projections; specs are truth.*

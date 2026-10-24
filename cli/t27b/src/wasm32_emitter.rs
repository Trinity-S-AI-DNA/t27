//! The wasm32 emitter (epic #11840, milestone M-A): a program of constant leaf
//! functions becomes a WebAssembly module a standard runtime instantiates, with
//! every function exported by its t27 name.
//!
//! Hand-written Rust by owner approval (`owner-approved-foreign`, session
//! 2026-10-23). The spec that will replace this file is
//! `specs/tri/t27b/wasm32.t27` (to be written); port epic gHashTag/t27#6198.
//!
//! Module shape emitted (sections in id order, the minimum valid set):
//!   magic `\0asm`, version 1
//!   1  type:     one func type per exported fn, `() -> T`
//!   3  function: type index i for fn i
//!   7  export:   fn i exported under its t27 name, kind func (0)
//!   10 code:     locals 0, `<T>.const <v>`, `end`
//!
//! Supported shape: `pub fn name() -> T { return <const>; }` with T a scalar:
//! bool, i8/i16/i32, u8/u16/u32, i64/u64, f32/f64, and the odd widths uN/iN.
//! Everything else in a pub fn is refused by name, never silently dropped.

use crate::ir::{ExprKind, Program, Stmt, Ty};

/// The byte law (specs/tri/t27b/wasm.t27, floors 1-4): valtypes, varints,
/// section framing, constant instructions. Landed on master by #11964; since
/// then this emitter CONSUMES the law instead of owning a second copy of it.
#[path = "../../../gen/rust/tri/t27b/wasm.rs"]
#[allow(dead_code)]
mod wasm_law;

/// Why a pub fn (or the module) is not emitable, shaped like
/// `codegen::CodegenError` so the CLI prints it the same way and `corpus`
/// parses the construct out of it.
#[derive(Debug)]
pub struct EmitError {
    pub construct: String,
    pub func: String,
    pub line: u32,
    pub detail: String,
}

impl EmitError {
    /// The `unsupported construct` line (the exact phrasing `corpus` parses).
    pub fn message(&self) -> String {
        format!(
            "unsupported construct {} at line {} (fn {}: {})",
            self.construct, self.line, self.func, self.detail
        )
    }

    fn new(func: &str, line: u32, construct: &str, detail: String) -> Self {
        EmitError {
            construct: construct.to_string(),
            func: func.to_string(),
            line,
            detail,
        }
    }
}

/// The wasm value type of an IR scalar, or None for what this emitter does not
/// carry (addresses, i128). The law pins the four wasm-native value types
/// (`valtype_byte`, 0 back for a name it does not know); the emitter-level
/// rule on top is that bool and every integer of at most 32 bits rides the
/// i32 carrier (u64 rides the i64 carrier beside i64), the same bytes the
/// reference emitted all along.
fn valtype(ty: Ty) -> Option<u8> {
    let b = wasm_law::valtype_byte(ty.name().as_bytes());
    if b != 0 {
        return Some(b);
    }
    match ty {
        Ty::U64 | Ty::I64 => Some(wasm_law::VT_I64),
        Ty::Ptr | Ty::I128 => None,
        _ => Some(wasm_law::VT_I32),
    }
}

/// True when this emitter carries the type across the wasm boundary: the M-A
/// scalar set (`bool`, the <=32-bit ints, `i64`/`u64`, `f32`/`f64`). `t27b
/// run` (#11824) accepts exactly these for its entry, so the native value and
/// the wasm value of a battery fn are comparable (tests/wasm32_equality.rs).
pub fn carries(ty: Ty) -> bool {
    valtype(ty).is_some()
}

/// Unsigned LEB128 (section sizes, vector counts, indices) -- the law's
/// index-style writer adapted to the emitter's push-style buffer.
fn uleb(out: &mut Vec<u8>, v: u64) {
    let at = out.len();
    out.resize(at + wasm_law::uleb_len(v), 0);
    wasm_law::uleb(out, at, v);
}

/// Signed LEB128 (`i32.const` / `i64.const` operands) -- the law's writer.
/// The carried scalar set fits i64 (a u64 constant crosses as the same bit
/// pattern read signed), and the varint encoding of those bytes is unique, so
/// the narrowing preserves the encoding the law pins.
fn sleb(out: &mut Vec<u8>, v: i128) {
    let at = out.len();
    out.resize(at + 10, 0);
    let end = wasm_law::sleb(out, at, v as i64);
    out.truncate(end);
}

/// One section: id byte, payload size, payload -- the law's framing.
fn section(out: &mut Vec<u8>, id: u8, payload: &[u8]) {
    let at = out.len();
    let cap = at + 1 + wasm_law::uleb_len(payload.len() as u64) + payload.len();
    out.resize(cap, 0);
    let end = wasm_law::section(out, at, id, payload);
    out.truncate(end);
}

/// The constant instruction for `<ty>.const <v>` (`v` an IR constant: an int
/// in range, or a float bit pattern). An unsigned constant at or above 2^31
/// (i32 carrier) or 2^63 (i64 carrier) crosses as the same bit pattern read
/// signed: wasm's integer carriers are signed, the varint has no other valid
/// encoding of those bytes.
fn const_insn(ty: Ty, v: i128) -> Vec<u8> {
    debug_assert!(ty.fits(v), "IR constant {} outside {}", v, ty.name());
    // opcode + worst-case payload: an i64 varint is 10 bytes, an f64 is 8.
    let mut b = [0u8; 11];
    let end = match ty {
        Ty::F32 => wasm_law::f32_const(&mut b, 0, crate::ir::f32_of(v).to_bits()),
        Ty::F64 => wasm_law::f64_const(&mut b, 0, crate::ir::f64_of(v).to_bits()),
        Ty::U64 | Ty::I64 => {
            let sv = if v > i64::MAX as i128 { v - (1i128 << 64) } else { v };
            wasm_law::i64_const(&mut b, 0, sv as i64)
        }
        _ => {
            let sv = if v > i32::MAX as i128 { v - (1i128 << 32) } else { v };
            wasm_law::i32_const(&mut b, 0, sv as i32)
        }
    };
    b[..end].to_vec()
}

/// The constant of the one-statement body `return <const>;`, or None for any
/// other shape.
fn const_return(f: &crate::ir::Func) -> Option<i128> {
    match f.body.as_slice() {
        [Stmt::Return(Some(e))] => match e.kind {
            ExprKind::Const(v) => Some(v),
            _ => None,
        },
        _ => None,
    }
}

/// Emit the whole module.
pub fn emit(prog: &Program) -> Result<Vec<u8>, EmitError> {
    // The export set: every pub fn with no parameters and a scalar return,
    // whose body is a single constant return. Tests and private fns are not
    // exports; a pub fn outside the supported shape is refused by name.
    let mut fns: Vec<(usize, Ty, i128)> = Vec::new();
    for (i, f) in prog.funcs.iter().enumerate() {
        if f.is_test || !f.is_pub {
            continue;
        }
        if f.nparams != 0 {
            return Err(EmitError::new(
                &f.name,
                f.line,
                "fn parameters",
                format!("the wasm32 emitter exports fns with no parameters, {} here", f.nparams),
            ));
        }
        let Some(ret) = f.ret else {
            return Err(EmitError::new(
                &f.name,
                f.line,
                "void pub fn",
                "the wasm32 emitter exports fns that return a scalar".into(),
            ));
        };
        if valtype(ret).is_none() {
            let what = if ret == Ty::Ptr { "ptr return" } else { "i128 return" };
            return Err(EmitError::new(
                &f.name,
                f.line,
                what,
                format!("the wasm32 emitter exports scalar returns, `{}` here", ret.name()),
            ));
        }
        let Some(v) = const_return(f) else {
            return Err(EmitError::new(
                &f.name,
                f.line,
                "fn body",
                "the wasm32 emitter lowers a single `return <const>;` body only".into(),
            ));
        };
        fns.push((i, ret, v));
    }
    if fns.is_empty() {
        return Err(EmitError::new(
            "(module)",
            0,
            "module",
            "no pub fn with no parameters whose body is a single `return <const>;`".into(),
        ));
    }
    let k = fns.len();
    let mut m = Vec::with_capacity(64);
    m.extend_from_slice(b"\0asm"); // the magic
    m.extend_from_slice(&1u32.to_le_bytes()); // version 1

    // 1 type: one `() -> T` per fn (no dedup; one type per fn keeps the walk trivial)
    let mut p = Vec::new();
    uleb(&mut p, k as u64);
    for (_, ret, _) in &fns {
        p.push(0x60); // functype
        uleb(&mut p, 0); // parameters
        uleb(&mut p, 1); // results
        p.push(valtype(*ret).expect("scalar, checked above"));
    }
    section(&mut m, 1, &p);

    // 3 function: fn i has type i
    let mut p = Vec::new();
    uleb(&mut p, k as u64);
    for i in 0..k {
        uleb(&mut p, i as u64);
    }
    section(&mut m, 3, &p);

    // 7 export: name, kind func (0), func index
    let mut p = Vec::new();
    uleb(&mut p, k as u64);
    for (n, (i, _, _)) in fns.iter().enumerate() {
        let name = &prog.funcs[*i].name;
        uleb(&mut p, name.len() as u64);
        p.extend_from_slice(name.as_bytes());
        p.push(0x00);
        uleb(&mut p, n as u64);
    }
    section(&mut m, 7, &p);

    // 10 code: local declaration count 0, the constant, end
    let mut p = Vec::new();
    uleb(&mut p, k as u64);
    for (_, ret, v) in &fns {
        let mut body = Vec::new();
        uleb(&mut body, 0); // local groups
        body.extend(const_insn(*ret, *v));
        body.push(0x0B); // end
        uleb(&mut p, body.len() as u64);
        p.extend_from_slice(&body);
    }
    section(&mut m, 10, &p);
    Ok(m)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{ArithOp, Expr, Func, OverflowMode, Program, Var};
    use std::path::Path;

    fn konst(ty: Ty, v: i128) -> Expr {
        Expr { ty, kind: ExprKind::Const(v) }
    }

    fn fn_const_ret(name: &str, ret: Ty, v: i128) -> Func {
        Func {
            name: name.into(),
            nparams: 0,
            ret: Some(ret),
            vars: vec![],
            body: vec![Stmt::Return(Some(konst(ret, v)))],
            line: 1,
            is_pub: true,
            is_test: false,
            is_invariant: false,
            noreturn_site: 0,
            slots: vec![],
        }
    }

    fn prog(funcs: Vec<Func>) -> Program {
        Program {
            module: String::new(),
            funcs,
            sites: vec![],
            mode: OverflowMode::Trap,
            unchecked: vec![],
            data: vec![],
            globals: vec![],
            internal_abi: vec![],
        }
    }

    fn read_uleb(b: &[u8]) -> (u64, usize) {
        let (mut v, mut n) = (0u64, 0usize);
        loop {
            let byte = b[n];
            v |= ((byte & 0x7f) as u64) << (7 * n);
            n += 1;
            if byte & 0x80 == 0 {
                return (v, n);
            }
        }
    }

    fn read_sleb(b: &[u8]) -> i128 {
        let (mut result, mut shift) = (0i128, 0u32);
        for &byte in b {
            result |= ((byte & 0x7f) as i128) << shift;
            shift += 7;
            if byte & 0x80 == 0 {
                if shift < 128 && byte & 0x40 != 0 {
                    result |= -1i128 << shift;
                }
                return result;
            }
        }
        result
    }

    #[test]
    fn leb128_unsigned_edges() {
        let mut b = Vec::new();
        uleb(&mut b, 0);
        assert_eq!(b, vec![0x00]);
        let mut b = Vec::new();
        uleb(&mut b, 127);
        assert_eq!(b, vec![0x7f]);
        let mut b = Vec::new();
        uleb(&mut b, 128);
        assert_eq!(b, vec![0x80, 0x01]);
        let mut b = Vec::new();
        uleb(&mut b, 624_485);
        assert_eq!(b, vec![0xe5, 0x8e, 0x26]);
        let (v, n) = read_uleb(&b);
        assert_eq!((v, n), (624_485, 3));
    }

    #[test]
    fn leb128_signed_edges() {
        for (v, want) in [
            (0i128, vec![0x00]),
            (1, vec![0x01]),
            (42, vec![0x2a]),
            (63, vec![0x3f]),
            (64, vec![0xc0, 0x00]),
            (-1, vec![0x7f]),
            (-64, vec![0x40]),
            (-65, vec![0xbf, 0x7f]),
            (-123_456, vec![0xc0, 0xbb, 0x78]),
        ] {
            let mut b = Vec::new();
            sleb(&mut b, v);
            assert_eq!(b, want, "sleb({})", v);
            assert_eq!(read_sleb(&b), v);
        }
        // the full i64 range survives the round trip
        for v in [i64::MIN as i128, i64::MAX as i128, -(1i128 << 63), (1i128 << 63) - 1] {
            let mut b = Vec::new();
            sleb(&mut b, v);
            assert_eq!(read_sleb(&b), v);
        }
    }

    #[test]
    fn one_i32_fn_exact_module_bytes() {
        // `pub fn the_answer() -> i32 { return 42; }`, byte for byte.
        let m = emit(&prog(vec![fn_const_ret("the_answer", Ty::I32, 42)])).expect("emit");
        let mut want = Vec::new();
        want.extend_from_slice(b"\0asm\x01\x00\x00\x00"); // magic, version
        want.extend_from_slice(&[0x01, 0x05]); // type section, 5 bytes
        want.extend_from_slice(&[0x01, 0x60, 0x00, 0x01, 0x7f]); // 1 type: () -> i32
        want.extend_from_slice(&[0x03, 0x02, 0x01, 0x00]); // function: 1, type 0
        want.extend_from_slice(&[0x07, 0x0e, 0x01]); // export, 14 bytes, 1 entry
        want.extend_from_slice(b"\x0athe_answer"); // name length 10, the name
        want.extend_from_slice(&[0x00, 0x00]); // kind func, index 0
        want.extend_from_slice(&[0x0a, 0x06, 0x01, 0x04, 0x00, 0x41, 0x2a, 0x0b]); // code: 1 body of 4: locals 0, i32.const 42, end
        assert_eq!(m, want);
        assert_eq!(m.len(), 43);
    }

    #[test]
    fn every_scalar_return_type_lowers_to_its_valtype() {
        let cases: Vec<(Ty, i128)> = vec![
            (Ty::I32, 42),
            (Ty::I64, -7),
            (Ty::U64, 1 << 40),
            (Ty::U32, 4_000_000_000),
            (Ty::Bool, 1),
            (Ty::U8, 255),
            (Ty::I8, -128),
            (Ty::UN(21), 2_000_000),
            (Ty::IN(21), -1_000_000),
            (Ty::F64, crate::ir::f64_bits(3.5)),
            (Ty::F32, crate::ir::f32_bits(-0.5)),
        ];
        let funcs = cases
            .iter()
            .enumerate()
            .map(|(i, (t, v))| fn_const_ret(&format!("f{}", i), *t, *v))
            .collect();
        let m = emit(&prog(funcs)).expect("emit");
        // The type section result byte of each fn is that type's valtype.
        // Layout: 8 header bytes; type section id at 8, its size at 9; the
        // vector count at 10; each entry is `60 00 01 XX`, 4 bytes, so the
        // result byte of entry i sits at 14 + 4*i.
        let want_t: Vec<u8> = cases.iter().map(|(t, _)| valtype(*t).unwrap()).collect();
        let vt: Vec<u8> = (0..cases.len()).map(|i| m[14 + 4 * i]).collect();
        assert_eq!(vt, want_t);
        // The exports are the fn names in order, and each body is `const, end`.
        let text = String::from_utf8_lossy(&m);
        for i in 0..cases.len() {
            assert!(text.contains(&format!("f{}", i)));
        }
        let last = *m.last().expect("nonempty");
        assert_eq!(last, 0x0B); // every body ends with `end`
    }

    #[test]
    fn i64_uses_the_i64_const_opcode_and_signed_leb() {
        let m = emit(&prog(vec![fn_const_ret("big", Ty::I64, -7)])).expect("emit");
        // code section body: locals 0, i64.const -7 (0x42 0x79), end
        assert!(m.ends_with(&[0x00, 0x42, 0x79, 0x0b]));
        // the type section says i64 (0x7e), not i32
        assert!(m.windows(4).any(|w| w == [0x60, 0x00, 0x01, 0x7e]));
    }

    #[test]
    fn unsigned_above_the_signed_carrier_wraps_to_the_same_bits() {
        // u32 4,000,000,000 is the 32-bit pattern wasm carries as -294,967,296:
        // the varint must be that signed image, or the runtime refuses the
        // module ("extra bits in varint").
        let insn = const_insn(Ty::U32, 4_000_000_000);
        assert_eq!(insn[0], 0x41);
        assert_eq!(read_sleb(&insn[1..]), -294_967_296);
        // u64 2^63 + 5 is the same 64 bits as the signed -2^63 + 5.
        let insn = const_insn(Ty::U64, (1i128 << 63) + 5);
        assert_eq!(insn[0], 0x42);
        assert_eq!(read_sleb(&insn[1..]), -(1i128 << 63) + 5);
        // everything below the boundary passes through unchanged
        let insn = const_insn(Ty::U32, 2_147_483_647);
        assert_eq!(read_sleb(&insn[1..]), 2_147_483_647);
        let insn = const_insn(Ty::I32, -2_147_483_648);
        assert_eq!(read_sleb(&insn[1..]), -2_147_483_648);
    }

    #[test]
    fn private_and_test_fns_are_skipped_not_refused() {
        let mut a = fn_const_ret("pubk", Ty::I32, 5);
        a.is_pub = true;
        let mut b = fn_const_ret("priv", Ty::I32, 6);
        b.is_pub = false;
        let mut c = fn_const_ret("atest", Ty::I32, 7);
        c.is_test = true;
        let m = emit(&prog(vec![b, c, a])).expect("emit");
        let text = String::from_utf8_lossy(&m);
        assert!(text.contains("pubk"));
        assert!(!text.contains("priv"));
        assert!(!text.contains("atest"));
    }

    #[test]
    fn refusals_name_the_construct() {
        // a pub fn with parameters
        let mut f = fn_const_ret("params", Ty::I32, 1);
        f.nparams = 2;
        f.vars = vec![Var { name: "x".into(), ty: Ty::I32 }, Var { name: "y".into(), ty: Ty::I32 }];
        let e = emit(&prog(vec![f])).expect_err("refused");
        assert_eq!(e.construct, "fn parameters");
        // a void pub fn
        let mut f = fn_const_ret("voidy", Ty::I32, 1);
        f.ret = None;
        f.body = vec![Stmt::Return(None)];
        let e = emit(&prog(vec![f])).expect_err("refused");
        assert_eq!(e.construct, "void pub fn");
        // a ptr return
        let f = fn_const_ret("ptrr", Ty::Ptr, 0);
        let e = emit(&prog(vec![f])).expect_err("refused");
        assert_eq!(e.construct, "ptr return");
        // a non-constant body
        let mut f = fn_const_ret("work", Ty::I32, 0);
        f.body = vec![Stmt::Eval(konst(Ty::I32, 1)), Stmt::Return(Some(konst(Ty::I32, 2)))];
        let e = emit(&prog(vec![f])).expect_err("refused");
        assert_eq!(e.construct, "fn body");
        // a non-constant return expression
        let mut f = fn_const_ret("work2", Ty::I32, 0);
        f.body = vec![Stmt::Return(Some(Expr { ty: Ty::I32, kind: ExprKind::Arith {
            op: ArithOp::Add, lhs: Box::new(konst(Ty::I32, 1)), rhs: Box::new(konst(Ty::I32, 2)), site: 0,
        } }))];
        let e = emit(&prog(vec![f])).expect_err("refused");
        assert_eq!(e.construct, "fn body");
        // the module-level refusal names itself
        let e = emit(&prog(vec![])).expect_err("refused");
        assert_eq!(e.construct, "module");
        assert_eq!(e.func, "(module)");
        // the message keeps the corpus-parsed phrasing
        let mut f = fn_const_ret("params", Ty::I32, 1);
        f.nparams = 2;
        assert_eq!(
            emit(&prog(vec![f])).expect_err("refused").message(),
            "unsupported construct fn parameters at line 1 (fn params: the wasm32 emitter exports fns with no parameters, 2 here)"
        );
    }

    #[test]
    fn end_to_end_from_source_through_the_front_end() {
        let src = "pub fn the_answer() -> i32 { return 42; }\n";
        let parsed = crate::front::parse(Path::new("/nonexistent/scratch.t27"), src).expect("parse");
        let lowered = crate::lower::lower_src(&parsed.ast, OverflowMode::Trap, Some(src)).expect("lower");
        let m = emit(&lowered).expect("emit");
        assert_eq!(&m[..8], b"\0asm\x01\x00\x00\x00");
        assert!(String::from_utf8_lossy(&m).contains("the_answer"));
        assert!(m.ends_with(&[0x41, 0x2a, 0x0b])); // i32.const 42, end
    }
}

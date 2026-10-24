//! The wasm32 emitter (epic #11840, milestones M-A + M-B): a program of
//! constant leaf functions and parameter-carrying scalar functions becomes a
//! WebAssembly module a standard runtime instantiates, with every function
//! exported by its t27 name.
//!
//! Hand-written Rust by owner approval (`owner-approved-foreign`, session
//! 2026-10-23). The spec that will replace this file is
//! `specs/tri/t27b/wasm32.t27` (to be written); port epic gHashTag/t27#6198.
//!
//! Module shape emitted (sections in id order, the minimum valid set):
//!   magic `\0asm`, version 1
//!   1  type:     one func type per exported fn, `(P... -> T` over the fn's
//!                own parameter count
//!   3  function: type index i for fn i
//!   7  export:   fn i exported under its t27 name, kind func (0)
//!   10 code:     locals 0, the body program, `end`
//!
//! Supported shapes (T a scalar: bool, i8/i16/i32, u8/u16/u32, i64/u64,
//! f32/f64, and the odd widths uN/iN):
//!   M-A: `pub fn name() -> T { return <const>; }` -- any carried T.
//!   M-B: `pub fn name(p0: P, ...) -> T { return <expr>; }` where the
//!     expression is a tree of `+ - * /` over parameters and i32-range
//!     constants, P and T on the i32 carrier (bool and the <=32-bit ints).
//!     `/` and integer `@divTrunc` lower to i32.div_u (truncated unsigned
//!     quotient). The value semantics cross; the native overflow trap of
//!     `+ - *` in Trap mode does not (wasm i32.add/sub/mul wrap) -- a floor-6
//!     concern. Everything else in a pub fn is refused by name, never
//!     silently dropped.
//!   M-C: comparisons join the tree (t27os#3's named gap, the law's floor
//!     11): `== != < <= > >=` over the same operands lower to 0x46/0x47 and
//!     the SIGNED 0x48/0x4A/0x4C/0x4E. Signed because t27's ordered
//!     comparisons on i32 are Zig's signed ones and the die's own law
//!     (wasm_board_ax7203.t27) numbers the same bytes signed -- `/` chose
//!     div_u to match the die where die and source disagreed; here they
//!     agree. `x == 0` folds to one eqz (0x45), `x != 0` to two.

use crate::ir::{CmpOp, ArithOp, Expr, ExprKind, Func, Program, Stmt, Ty};

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

/// The lowered body of one exported fn: the M-A constant leaf, or the M-B
/// stack program over the fn's parameters (instructions without the trailing
/// `end`).
enum Body {
    Const(Ty, i128),
    Stack(Vec<u8>),
}

/// Append `<expr>`'s postfix stack program to `out`: parameters via
/// `local.get`, i32-range constants via `i32.const`, `+ - * /` via the floor-5
/// opcodes (`Div` -- the IR of both `/` and integer `@divTrunc` -- via
/// i32.div_u), comparisons `== != < <= > >=` via floor 11 (0x46/0x47 and the
/// signed 0x48/0x4A/0x4C/0x4E; `x == 0` folds to one eqz, `x != 0` to two,
/// the law's eqz_fold rule). Anything else is refused by name.
fn lower_stack(e: &Expr, f: &Func, out: &mut Vec<u8>) -> Result<(), EmitError> {
    match &e.kind {
        ExprKind::Const(v) => {
            // The i32 carrier: a u32 constant at or above 2^31 crosses as the
            // same bit pattern read signed (const_insn's rule, i32 branch).
            let sv = if *v > i32::MAX as i128 { *v - (1i128 << 32) } else { *v };
            let mut b = [0u8; 11];
            let end = wasm_law::i32_const(&mut b, 0, sv as i32);
            out.extend_from_slice(&b[..end]);
            Ok(())
        }
        ExprKind::Var(id) => {
            if *id as usize >= f.nparams {
                return Err(EmitError::new(
                    &f.name,
                    f.line,
                    "local variable",
                    "the wasm32 emitter reads the fn's parameters only".into(),
                ));
            }
            let mut b = [0u8; 11];
            let end = wasm_law::local_get(&mut b, 0, *id);
            out.extend_from_slice(&b[..end]);
            Ok(())
        }
        ExprKind::Arith { op, lhs, rhs, .. } => {
            let opcode: u8 = match op {
                ArithOp::Add => wasm_law::OP_I32_ADD,
                ArithOp::Sub => wasm_law::OP_I32_SUB,
                ArithOp::Mul => wasm_law::OP_I32_MUL,
                ArithOp::Div => wasm_law::OP_I32_DIV_U,
                other => {
                    return Err(EmitError::new(
                        &f.name,
                        f.line,
                        "arithmetic op",
                        format!("the wasm32 emitter lowers + - * / on the i32 carrier, `{:?}` here", other),
                    ));
                }
            };
            lower_stack(lhs, f, out)?;
            lower_stack(rhs, f, out)?;
            out.push(opcode);
            Ok(())
        }
        ExprKind::Cmp { op, lhs, rhs } => {
            let is_zero = |e: &Expr| matches!(e.kind, ExprKind::Const(0));
            let folded = wasm_law::eqz_fold(*op as u8, is_zero(lhs), is_zero(rhs));
            if folded > 0 {
                // `x == 0` / `0 == x` is one eqz over x; `x != 0` two. The
                // zero operand never lowers -- floor 11's rule.
                let x = if is_zero(lhs) { rhs } else { lhs };
                lower_stack(x, f, out)?;
                for _ in 0..folded {
                    out.push(wasm_law::OP_I32_EQZ);
                }
                return Ok(());
            }
            let opcode = wasm_law::cmp_opcode(*op as u8);
            debug_assert!(opcode != 0, "CmpOp has six variants, all on floor 11");
            lower_stack(lhs, f, out)?;
            lower_stack(rhs, f, out)?;
            out.push(opcode);
            Ok(())
        }
        _ => Err(EmitError::new(
            &f.name,
            f.line,
            "expression kind",
            "the wasm32 emitter lowers parameters, i32 constants, + - * / and == != < <= > >= only".into(),
        )),
    }
}

/// Emit the whole module.
pub fn emit(prog: &Program) -> Result<Vec<u8>, EmitError> {
    // The export set: every pub fn in a supported shape. M-A: no parameters,
    // a scalar return, a single constant return. M-B: parameters on the i32
    // carrier, an i32-carrier return, a single `return <expr>;` whose
    // expression is a tree of + - * / over parameters and i32 constants.
    // Tests and private fns are not exports; a pub fn outside the supported
    // shape is refused by name.
    let mut fns: Vec<(usize, Ty, usize, Body)> = Vec::new();
    for (i, f) in prog.funcs.iter().enumerate() {
        if f.is_test || !f.is_pub {
            continue;
        }
        let Some(ret) = f.ret else {
            return Err(EmitError::new(
                &f.name,
                f.line,
                "void pub fn",
                "the wasm32 emitter exports fns that return a scalar".into(),
            ));
        };
        if f.nparams == 0 {
            // The M-A path: a constant leaf of any carried scalar type.
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
            fns.push((i, ret, 0, Body::Const(ret, v)));
        } else {
            // The M-B path: a stack program over i32-carrier parameters.
            for p in &f.vars[..f.nparams] {
                if valtype(p.ty) != Some(wasm_law::VT_I32) {
                    return Err(EmitError::new(
                        &f.name,
                        f.line,
                        "fn parameters",
                        format!("the wasm32 emitter carries i32 parameters, `{}` here", p.ty.name()),
                    ));
                }
            }
            match valtype(ret) {
                None => {
                    let what = if ret == Ty::Ptr { "ptr return" } else { "i128 return" };
                    return Err(EmitError::new(
                        &f.name,
                        f.line,
                        what,
                        format!("the wasm32 emitter exports scalar returns, `{}` here", ret.name()),
                    ));
                }
                Some(wasm_law::VT_I32) => {}
                Some(_) => {
                    return Err(EmitError::new(
                        &f.name,
                        f.line,
                        "fn return type",
                        format!("the wasm32 emitter carries i32 results here, `{}` instead", ret.name()),
                    ));
                }
            }
            let mut stack = Vec::new();
            match f.body.as_slice() {
                [Stmt::Return(Some(e))] => {
                    lower_stack(e, f, &mut stack)?;
                }
                // The select pattern: `if (<param> == 0) { return <x>; }
                // return <y>;` -- eqz (0x45) is the law's comparison floor,
                // select (0x1B) its branch-free join; the operands are i32
                // constants, so the whole body is five instructions.
                [Stmt::If { cond, then: tb, els }, Stmt::Return(Some(fall))] => {
                    let refuse_body = || EmitError::new(
                        &f.name,
                        f.line,
                        "fn body",
                        "the wasm32 emitter lowers `return <expr>;` or `if (<param> == 0) { return <const>; } return <const>;` only".into(),
                    );
                    if !els.is_empty() {
                        return Err(refuse_body());
                    }
                    let [Stmt::Return(Some(te))] = tb.as_slice() else {
                        return Err(refuse_body());
                    };
                    let (ExprKind::Const(x), ExprKind::Const(y)) = (&te.kind, &fall.kind) else {
                        return Err(refuse_body());
                    };
                    let ExprKind::Cmp { op: CmpOp::Eq, lhs, rhs } = &cond.kind else {
                        return Err(EmitError::new(
                            &f.name,
                            f.line,
                            "if condition",
                            "the wasm32 emitter compares a parameter against 0 (eqz, the law's comparison floor) only".into(),
                        ));
                    };
                    // `<param> == 0` or `0 == <param>`: either spelling is eqz(param).
                    let pv = match (&lhs.kind, &rhs.kind) {
                        (ExprKind::Var(p), ExprKind::Const(0)) => *p,
                        (ExprKind::Const(0), ExprKind::Var(p)) => *p,
                        _ => {
                            return Err(EmitError::new(
                                &f.name,
                                f.line,
                                "if condition",
                                "the wasm32 emitter compares a parameter against 0 only (`== 0`, eqz's exact shape)".into(),
                            ));
                        }
                    };
                    if pv as usize >= f.nparams {
                        return Err(refuse_body());
                    }
                    // select's stack: the true value, the false value, the condition.
                    let sv = |v: &i128| if *v > i32::MAX as i128 { *v - (1i128 << 32) } else { *v };
                    let mut b = [0u8; 11];
                    let end = wasm_law::i32_const(&mut b, 0, sv(x) as i32);
                    stack.extend_from_slice(&b[..end]);
                    let end = wasm_law::i32_const(&mut b, 0, sv(y) as i32);
                    stack.extend_from_slice(&b[..end]);
                    let end = wasm_law::local_get(&mut b, 0, pv);
                    stack.extend_from_slice(&b[..end]);
                    stack.push(wasm_law::OP_I32_EQZ);
                    stack.push(wasm_law::OP_SELECT);
                }
                _ => {
                    return Err(EmitError::new(
                        &f.name,
                        f.line,
                        "fn body",
                        "the wasm32 emitter lowers `return <expr>;` or `if (<param> == 0) { return <const>; } return <const>;` only".into(),
                    ));
                }
            }
            fns.push((i, ret, f.nparams, Body::Stack(stack)));
        }
    }
    if fns.is_empty() {
        return Err(EmitError::new(
            "(module)",
            0,
            "module",
            "no pub fn whose body the wasm32 emitter lowers (a constant leaf, or an expression over i32 parameters and constants)".into(),
        ));
    }
    let k = fns.len();
    let mut m = Vec::with_capacity(64);
    m.extend_from_slice(b"\0asm"); // the magic
    m.extend_from_slice(&1u32.to_le_bytes()); // version 1

    // 1 type: one functype per fn, over its own parameter count (no dedup; one
    // type per fn keeps the walk trivial)
    let mut p = Vec::new();
    uleb(&mut p, k as u64);
    for (_, ret, nparams, _) in &fns {
        if *nparams == 0 {
            p.push(0x60); // functype
            uleb(&mut p, 0); // parameters
            uleb(&mut p, 1); // results
            p.push(valtype(*ret).expect("scalar, checked above"));
        } else {
            // `(...) -> i32` over the fn's own parameter list, via the law.
            let params = vec![wasm_law::VT_I32; *nparams];
            let results = [valtype(*ret).expect("i32 carrier, checked above")];
            let at = p.len();
            let cap = at + 1 + wasm_law::uleb_len(*nparams as u64) + *nparams + 1 + 1;
            p.resize(cap, 0);
            let end = wasm_law::functype_full(&mut p, at, &params, &results);
            p.truncate(end);
        }
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
    for (n, (i, _, _, _)) in fns.iter().enumerate() {
        let name = &prog.funcs[*i].name;
        uleb(&mut p, name.len() as u64);
        p.extend_from_slice(name.as_bytes());
        p.push(0x00);
        uleb(&mut p, n as u64);
    }
    section(&mut m, 7, &p);

    // 10 code: local declaration count 0, the body program, end
    let mut p = Vec::new();
    uleb(&mut p, k as u64);
    for (_, _, _, body) in &fns {
        let mut body_bytes = Vec::new();
        uleb(&mut body_bytes, 0); // local groups
        match body {
            Body::Const(ty, v) => body_bytes.extend(const_insn(*ty, *v)),
            Body::Stack(bytes) => body_bytes.extend_from_slice(bytes),
        }
        body_bytes.push(0x0B); // end
        uleb(&mut p, body_bytes.len() as u64);
        p.extend_from_slice(&body_bytes);
    }
    section(&mut m, 10, &p);
    Ok(m)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Expr, Func, OverflowMode, Program};

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
}


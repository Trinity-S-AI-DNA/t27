//! The wasm32 emitter (`t27b build --target wasm32`, epic #11840 M-A), from
//! the public surface: the bytes `emit` writes, the exports they describe, and
//! the refusals for everything a pub fn does beyond the supported shape.
//!
//! The private LEB128 helpers have their unit tests in the emitter itself
//! (`cargo test -p t27b --lib`); this file holds what the canonical gate runs.

use std::path::Path;
use t27b::ir::*;
use t27b::wasm32_emitter;
use t27b::{front, lower};

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

fn lower_src(src: &str) -> Program {
    let parsed = front::parse(Path::new("/nonexistent/t.t27"), src).expect("parse");
    lower::lower_src(&parsed.ast, OverflowMode::Trap, Some(src)).expect("lower")
}

#[test]
fn one_i32_fn_is_the_exact_module() {
    // `pub fn the_answer() -> i32 { return 42; }`, byte for byte: magic and
    // version, one func type `() -> i32`, the function section, the export,
    // and a code body of locals 0, i32.const 42, end.
    let m = wasm32_emitter::emit(&prog(vec![fn_const_ret("the_answer", Ty::I32, 42)])).expect("emit");
    let mut want = Vec::new();
    want.extend_from_slice(b"\0asm\x01\x00\x00\x00"); // magic, version 1
    want.extend_from_slice(&[0x01, 0x05, 0x01, 0x60, 0x00, 0x01, 0x7f]); // type: 1 x () -> i32
    want.extend_from_slice(&[0x03, 0x02, 0x01, 0x00]); // function: 1 fn of type 0
    want.extend_from_slice(&[0x07, 0x0e, 0x01]); // export: 1 entry
    want.extend_from_slice(b"\x0athe_answer\x00\x00"); // name, kind func, index 0
    want.extend_from_slice(&[0x0a, 0x06, 0x01, 0x04, 0x00, 0x41, 0x2a, 0x0b]); // code: 1 body of 4
    assert_eq!(m, want);
    assert_eq!(m.len(), 43);
}

#[test]
fn every_scalar_return_type_carries_its_valtype() {
    // bool and the <=32-bit ints ride i32 (0x7f), i64/u64 ride i64 (0x7e),
    // f32/f64 ride 0x7d/0x7c; the result byte of type i sits at 14 + 4*i.
    let cases: Vec<(Ty, i128, u8)> = vec![
        (Ty::I32, 42, 0x7f),
        (Ty::I64, -7, 0x7e),
        (Ty::U64, 1 << 40, 0x7e),
        (Ty::U32, 4_000_000_000, 0x7f),
        (Ty::Bool, 1, 0x7f),
        (Ty::U8, 255, 0x7f),
        (Ty::I8, -128, 0x7f),
        (Ty::UN(21), 2_000_000, 0x7f),
        (Ty::IN(21), -1_000_000, 0x7f),
        (Ty::F64, crate_f64_bits(3.5), 0x7c),
        (Ty::F32, crate_f32_bits(-0.5), 0x7d),
    ];
    let funcs = cases
        .iter()
        .enumerate()
        .map(|(i, (t, v, _))| fn_const_ret(&format!("f{}", i), *t, *v))
        .collect();
    let m = wasm32_emitter::emit(&prog(funcs)).expect("emit");
    for (i, (_, _, vt)) in cases.iter().enumerate() {
        assert_eq!(m[14 + 4 * i], *vt, "valtype of fn {}", i);
    }
    assert_eq!(*m.last().expect("nonempty"), 0x0b); // every body ends with `end`
}

fn crate_f64_bits(x: f64) -> i128 {
    x.to_bits() as i128
}

fn crate_f32_bits(x: f32) -> i128 {
    x.to_bits() as i128
}

#[test]
fn end_to_end_from_source_exports_the_answer() {
    let src = "pub fn the_answer() -> i32 { return 42; }\n";
    let m = wasm32_emitter::emit(&lower_src(src)).expect("emit");
    assert_eq!(&m[..8], b"\0asm\x01\x00\x00\x00");
    assert!(String::from_utf8_lossy(&m).contains("the_answer"));
    assert!(m.ends_with(&[0x41, 0x2a, 0x0b])); // i32.const 42, end
}

#[test]
fn private_and_test_fns_are_skipped_not_refused() {
    let mut b = fn_const_ret("priv", Ty::I32, 6);
    b.is_pub = false;
    let mut c = fn_const_ret("atest", Ty::I32, 7);
    c.is_test = true;
    let a = fn_const_ret("pubk", Ty::I32, 5);
    let m = wasm32_emitter::emit(&prog(vec![b, c, a])).expect("emit");
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
    let e = wasm32_emitter::emit(&prog(vec![f])).expect_err("refused");
    assert_eq!(e.construct, "fn parameters");
    // a void pub fn
    let mut f = fn_const_ret("voidy", Ty::I32, 1);
    f.ret = None;
    f.body = vec![Stmt::Return(None)];
    let e = wasm32_emitter::emit(&prog(vec![f])).expect_err("refused");
    assert_eq!(e.construct, "void pub fn");
    // a ptr return
    let f = fn_const_ret("ptrr", Ty::Ptr, 0);
    let e = wasm32_emitter::emit(&prog(vec![f])).expect_err("refused");
    assert_eq!(e.construct, "ptr return");
    // a body that does more than return one constant
    let mut f = fn_const_ret("work", Ty::I32, 0);
    f.body = vec![Stmt::Eval(konst(Ty::I32, 1)), Stmt::Return(Some(konst(Ty::I32, 2)))];
    let e = wasm32_emitter::emit(&prog(vec![f])).expect_err("refused");
    assert_eq!(e.construct, "fn body");
    // a non-constant return expression
    let mut f = fn_const_ret("work2", Ty::I32, 0);
    f.body = vec![Stmt::Return(Some(Expr {
        ty: Ty::I32,
        kind: ExprKind::Arith {
            op: ArithOp::Add,
            lhs: Box::new(konst(Ty::I32, 1)),
            rhs: Box::new(konst(Ty::I32, 2)),
            site: 0,
        },
    }))];
    let e = wasm32_emitter::emit(&prog(vec![f])).expect_err("refused");
    assert_eq!(e.construct, "fn body");
    // the module-level refusal names itself
    let e = wasm32_emitter::emit(&prog(vec![])).expect_err("refused");
    assert_eq!((e.construct.as_str(), e.func.as_str()), ("module", "(module)"));
    // the message keeps the phrasing `corpus` parses out of stderr
    let mut f = fn_const_ret("params", Ty::I32, 1);
    f.nparams = 2;
    assert_eq!(
        wasm32_emitter::emit(&prog(vec![f])).expect_err("refused").message(),
        "unsupported construct fn parameters at line 1 (fn params: the wasm32 emitter exports fns with no parameters, 2 here)"
    );
}

#[test]
fn the_front_end_refusal_reaches_the_wasm_path() {
    // A pub fn with parameters lowers fine in general; the wasm emitter is the
    // one refusing it. One source-level walk to the same construct.
    let src = "pub fn later(x: i32) -> i32 { return x; }\n";
    let prog = lower_src(src);
    let e = wasm32_emitter::emit(&prog).expect_err("refused");
    assert_eq!(e.construct, "fn parameters");
    assert_eq!(e.func, "later");
}

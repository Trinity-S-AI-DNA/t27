//! `pass_vacuous` (#6115): a pass having executed no runtime assert, per the
//! reference interpreter `t27b test --check` runs beside the JIT; #12751 adds
//! the CT-VERIFIED count, so a folded-only pass is no longer called vacuous.

use std::path::Path;

use t27b::eval::Interp;
use t27b::ir::OverflowMode;
use t27b::{front, lower};

/// (runtime asserts, compile-time-verified checks) over every test, summed.
fn parsed_ct(name: &str, src: &str) -> (u64, u64) {
    let parsed = front::parse(Path::new(name), src).expect("fixture parses");
    let prog = lower::lower(&parsed.ast, OverflowMode::Trap).expect("fixture lowers");
    let mut total = 0;
    let mut ct = 0;
    for (id, _) in prog.tests() {
        let mut it = Interp::new(&prog);
        it.call(id, &[]).expect("fixture test passes");
        total += it.asserts;
        ct += it.ct_verified;
    }
    (total, ct)
}

/// Every assert folds to a constant: nothing is checked at run time.
const CONSTANT_ONLY: &str = r#"module vacuous_fixture;
test folded {
    assert(1 + 1 == 2);
    assert_eq(3, 3);
    assert(true);
}
"#;

/// The negative control: the same shape, but each assert depends on a call.
const RUNTIME: &str = r#"module runtime_fixture;
fn inc(x: u32) -> u32 {
    return x + 1;
}
test checked {
    assert_eq(inc(1), 2);
    assert(inc(2) == 3);
}
"#;

/// An assert inside a loop counts once per execution, not once per site.
const LOOP: &str = r#"module loop_fixture;
fn id(x: u32) -> u32 {
    return x;
}
test loops {
    var i : u32 = 0;
    while i < 4 {
        assert(id(i) == i);
        i = i + 1;
    }
}
"#;

#[test]
fn constant_only_asserts_are_ct_verified_not_vacuous() {
    // #12751 parity: folded checks count as CT-VERIFIED, not vacuous.
    assert_eq!(parsed_ct("vacuous_fixture.t27", CONSTANT_ONLY), (0, 3));
}

#[test]
fn runtime_asserts_are_counted() {
    assert_eq!(parsed_ct("runtime_fixture.t27", RUNTIME), (2, 0));
}

#[test]
fn asserts_are_counted_per_execution() {
    assert_eq!(parsed_ct("loop_fixture.t27", LOOP), (4, 0));
}

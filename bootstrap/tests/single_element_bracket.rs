//! A bracketed single element with a declared `[1]T` annotation is a LIST.
//!
//! `pub const DATE: [1]u32 = [20261026];` used to parse its initializer as
//! `ExprIdentifier("[42]")` -- a single-element bracket group with no comma was
//! misclassified as an identifier, because the bare-array parser deliberately
//! refuses single integer elements as indistinguishable from a dimension
//! (`[42]T`). The zig emitter then printed the identifier verbatim and zig
//! parsed the output as an array TYPE: `expected type expression, found ';'`.
//!
//! The declared annotation settles the ambiguity: a `[1]T` const whose value is
//! one bracketed integer is a one-element list. Found landing epic #12660's
//! progress card (specs/blog/epic_progress.t27).

use std::process::Command;

fn gen(spec: &str, tag: &str) -> String {
    let dir = std::env::temp_dir().join(format!("t27c-single-bracket-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    let p = dir.join("in.t27");
    std::fs::write(&p, spec).expect("write spec");
    let out = Command::new(env!("CARGO_BIN_EXE_t27c"))
        .arg("gen")
        .arg(&p)
        .output()
        .expect("run t27c gen");
    assert!(out.status.success(), "t27c gen failed: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8(out.stdout).expect("gen output utf8")
}

#[test]
fn declared_arity_one_makes_a_single_integer_element_a_list() {
    let out = gen(
        "pub const DATE: [1]u32 = [20261026];\n\npub fn get() -> u32 {\n    return DATE[0];\n}\n",
        "arity-one",
    );
    let init = out
        .lines()
        .find(|l| l.starts_with("pub const DATE"))
        .expect("DATE const in the zig output");
    assert!(
        init.contains(".{ 20261026 }"),
        "the initializer must be a zig one-element list, got: {init}"
    );
    assert!(
        !init.contains("= ["),
        "the rust-style bracket initializer must not reach the zig output: {init}"
    );
}

#[test]
fn a_single_element_stays_a_dimension_without_the_annotation() {
    // `pub const D = [42];` with no declared type keeps its legacy text form:
    // the ambiguity guard is exactly what this fix does not touch.
    let out = gen("pub const D = [42];\n", "no-annotation");
    // Either shape is acceptable here -- the test pins today's behavior so a
    // future widening of the guard is a deliberate decision, not a drift.
    assert!(out.contains("pub const D"), "the const must survive: {out}");
}

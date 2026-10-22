//! #7176: a `use` that `t27c gen` cannot resolve is named on stderr.
//!
//! The splice skipped a target that was not a file without a word, `gen`
//! exited 0, and the first error came from zig about an identifier
//! (`use of undeclared identifier 'LIST_END'`), never about the `use` line.
//! Through the SHIPPED binary, as `unknown_type.rs` does: a unit test alone
//! would pass against a compiler that never called the check.

use std::path::{Path, PathBuf};
use std::process::Command;

fn scratch(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("t27c-7176-cli-{}-{}", std::process::id(), tag));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).expect("dir");
    d
}

/// (exit ok, stdout, stderr) of `t27c gen <p>`.
fn gen(p: &Path) -> (bool, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_t27c"))
        .arg("gen")
        .arg(p)
        .output()
        .expect("t27c");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
    )
}

const IMPORTER: &str = "module c;\nuse a::b;\n\npub fn k() -> u8 {\n    return K;\n}\n";

#[test]
fn a_use_the_splice_cannot_find_is_named_with_its_line() {
    let d = scratch("missing").join("specs/c");
    std::fs::create_dir_all(&d).expect("dir");
    let p = d.join("in.t27");
    std::fs::write(&p, "module c;\nuse a::b;\n\npub const N : u8 = 1;\n").expect("write");
    let (ok, _, err) = gen(&p);
    assert!(ok, "the exit code does not change: {err}");
    let want = format!("{}:2: use a::b resolves to no spec", p.display());
    assert!(err.contains(&want), "want {want:?} in:\n{err}");
    assert!(err.contains("nothing is spliced from it (#7176)"), "{err}");
}

#[test]
fn a_use_the_splice_finds_prints_no_warning() {
    let root = scratch("found").join("specs");
    std::fs::create_dir_all(root.join("a")).expect("dir");
    std::fs::create_dir_all(root.join("c")).expect("dir");
    std::fs::write(root.join("a/b.t27"), "module b;\npub const K : u8 = 3;\n").expect("write");
    let p = root.join("c/in.t27");
    std::fs::write(&p, IMPORTER).expect("write");
    let (ok, out, err) = gen(&p);
    assert!(ok, "{err}");
    assert!(!err.contains("#7176"), "{err}");
    assert!(out.contains("const K"), "the spliced constant is in the output:\n{out}");
}

/// #8980: the spec tree is found from where the file IS, not from how its path is spelled. Called
/// as a bare `in.t27` from its own directory (parent "") or as `../c/in.t27`, the walk never met
/// `specs/`, nothing was spliced, and `test-report` was BLOCKED on an undeclared `constants`.
#[test]
fn a_use_resolves_however_the_spec_path_is_spelled() {
    let root = scratch("spelled").join("specs");
    std::fs::create_dir_all(root.join("a")).expect("dir");
    std::fs::create_dir_all(root.join("c")).expect("dir");
    std::fs::write(root.join("a/b.t27"), "module b;\npub const K : u8 = 3;\n").expect("write");
    std::fs::write(root.join("c/in.t27"), IMPORTER).expect("write");
    for spelled in ["in.t27", "../c/in.t27"] {
        let out = Command::new(env!("CARGO_BIN_EXE_t27c"))
            .args(["gen", spelled])
            .current_dir(root.join("c"))
            .output()
            .expect("t27c");
        let (stdout, stderr) = (String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
        assert!(out.status.success(), "{spelled}: {stderr}");
        assert!(!stderr.contains("#7176"), "{spelled}: {stderr}");
        assert!(stdout.contains("const K"), "{spelled}: the spliced constant is in the output:\n{stdout}");
    }
}

/// #10556: write specs/r/<name>.t27 for each file and gen the last one.
fn gen_tree(tag: &str, files: &[(&str, &str)]) -> (bool, String, String) {
    let root = scratch(tag).join("specs/r");
    std::fs::create_dir_all(&root).expect("dir");
    for (n, t) in files { std::fs::write(root.join(format!("{n}.t27")), t).expect("write"); }
    gen(&root.join(format!("{}.t27", files.last().expect("files").0)))
}
const A: &str = "module a;\npub fn ok(n: u32) -> bool {\n    return n > 0;\n}\npub fn admit(n: u32) -> bool {\n    return ok(n);\n}\n";
const B: &str = "module b;\npub fn ok(k: u32, f: bool) -> bool {\n    return k > 0 or f;\n}\npub fn fine(k: u32) -> bool {\n    return ok(k, false);\n}\n";
fn imp(body: &str) -> String { format!("module imp;\nuse r::a;\nuse r::b;\n\npub fn s(n: u32) -> bool {{\n    return {body};\n}}\n") }

#[test]
fn a_helper_two_modules_declare_stays_with_the_module_that_means_it() {
    let (ok, out, err) = gen_tree("scope", &[("a", A), ("b", B), ("imp", &imp("admit(n) and b::fine(n)"))]);
    assert!(ok && !err.contains("UNRESOLVED") && !out.contains("UNRESOLVED"), "{err}");
    for want in ["fn r_a__ok(", "fn r_b__ok(", "return r_a__ok(n);", "return r_b__ok(k, false);"] { assert!(out.contains(want), "{want}:\n{out}"); }
}

#[test]
fn a_qualified_helper_takes_the_flat_name_of_the_module_it_names() {
    let (_, out, err) = gen_tree("qualified", &[("a", A), ("b", B), ("imp", &imp("b::ok(n, true)"))]);
    assert!(!err.contains("UNRESOLVED") && out.contains("return r_b__ok(n, true);") && !out.contains("r_a__ok"), "{err}\n{out}");
}

#[test]
fn a_name_the_importer_writes_that_two_modules_declare_is_still_refused() {
    let (ok, out, err) = gen_tree("refused", &[("a", A), ("b", B), ("imp", &imp("ok(n, true)"))]);
    assert!(ok && err.contains("UNRESOLVED ok: declared in r/a.t27 and r/b.t27"), "{err}");
    assert!(!out.contains("r_a__ok") && !out.contains("fn ok("), "{out}");
}

#[test]
fn a_path_in_a_pulled_declaration_loses_its_module() {
    let c = "module c;\nuse r::a;\npub fn f(n: u32) -> bool {\n    return a::admit(n);\n}\n";
    let (ok, out, err) = gen_tree("path", &[("a", A), ("c", c), ("imp", "module imp;\nuse r::c;\n\npub fn s(n: u32) -> bool {\n    return c::f(n);\n}\n")]);
    assert!(ok && out.contains("return admit(n);") && !out.contains("a.admit("), "{err}\n{out}");
}

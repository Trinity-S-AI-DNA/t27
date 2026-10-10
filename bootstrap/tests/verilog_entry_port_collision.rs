// ============================================================================
// #5649: an entry-point parameter named like a boilerplate port is declared once.
//
// `gen-verilog` opens every module with the 1-bit inputs `clk`, `rst_n` and
// `en`, then adds one `input` port per parameter of `fn on_comb` /
// `fn on_clock`. A parameter of the same name was declared a second time and
// iverilog refused the module ("'clk' has already been declared in this
// scope"). specs/compiler/verilog_entry_port_collision.t27 decides: a 1-bit
// unsigned parameter IS that port and the header declares it once; a wider or
// signed one refuses the entry point instead of being narrowed into it.
// ============================================================================

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn t27c() -> &'static str {
    env!("CARGO_BIN_EXE_t27c")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn scratch_dir(label: &str) -> PathBuf {
    let dir = env::temp_dir().join(format!("t27_5649_{}_{}", std::process::id(), label));
    if dir.exists() {
        let _ = fs::remove_dir_all(&dir);
    }
    fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

fn gen_verilog_source(label: &str, source: &str) -> Output {
    let dir = scratch_dir(label);
    let spec = dir.join(format!("{label}.t27"));
    fs::write(&spec, source).expect("write spec");
    let out = Command::new(t27c())
        .arg("gen-verilog")
        .arg(&spec)
        .output()
        .expect("run t27c gen-verilog");
    let _ = fs::remove_dir_all(&dir);
    out
}

fn gen_verilog_spec(rel: &str) -> Output {
    Command::new(t27c())
        .arg("gen-verilog")
        .arg(repo_root().join(rel))
        .output()
        .expect("run t27c gen-verilog")
}

fn stdout_of(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn stderr_of(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

/// The lines between `module NAME (` and `);`, trimmed.
fn header(verilog: &str) -> Vec<String> {
    let mut inside = false;
    let mut lines = Vec::new();
    for line in verilog.lines() {
        if line.starts_with("module ") {
            inside = true;
            continue;
        }
        if inside && line.starts_with(");") {
            break;
        }
        if inside {
            lines.push(line.trim().to_string());
        }
    }
    lines
}

/// The name each header port declares (the last word, comma dropped).
fn port_names(verilog: &str) -> Vec<String> {
    header(verilog)
        .iter()
        .filter(|l| l.starts_with("input") || l.starts_with("output"))
        .filter_map(|l| l.trim_end_matches(',').split_whitespace().last().map(str::to_string))
        .collect()
}

/// How many header ports declare `name`.
fn declarations_of(verilog: &str, name: &str) -> usize {
    port_names(verilog).iter().filter(|n| n.as_str() == name).count()
}

/// FR-001: every header port name, at most once.
fn assert_each_port_once(verilog: &str) {
    let names = port_names(verilog);
    for n in &names {
        assert_eq!(
            names.iter().filter(|m| *m == n).count(),
            1,
            "`{n}` is declared more than once in the header:\n{verilog}"
        );
    }
}

/// When Icarus is installed (CI installs it for this suite), the module must elaborate.
fn assert_iverilog_accepts(label: &str, verilog: &str) {
    if !Command::new("iverilog").arg("-V").output().map(|o| o.status.success()).unwrap_or(false) {
        eprintln!("iverilog not found: {label} checked for duplicate ports only");
        return;
    }
    let dir = scratch_dir(&format!("{label}_iverilog"));
    let vp = dir.join(format!("{label}.v"));
    fs::write(&vp, verilog).expect("write verilog");
    let iv = Command::new("iverilog")
        .args(["-g2012", "-o", "/dev/null"])
        .arg(&vp)
        .output()
        .expect("run iverilog");
    let _ = fs::remove_dir_all(&dir);
    assert!(
        iv.status.success(),
        "iverilog rejected {label}:\n{}\n{verilog}",
        String::from_utf8_lossy(&iv.stderr)
    );
}

#[test]
fn a_bool_clk_parameter_is_declared_once_and_still_drives_on_comb() {
    // d_bank13_test.t27's shape.
    let out = gen_verilog_source(
        "clk_param",
        "module clk_param;\n\nfn on_comb(clk: bool) -> bool {\n    return clk;\n}\n",
    );
    assert!(out.status.success(), "gen-verilog failed: {}", stderr_of(&out));
    let v = stdout_of(&out);
    assert_eq!(declarations_of(&v, "clk"), 1, "clk is not declared exactly once:\n{v}");
    assert!(
        header(&v).iter().any(|l| l == "input  wire        clk,"),
        "the 1-bit boilerplate clk port is missing:\n{v}"
    );
    assert_eq!(v.matches("assign result = on_comb(clk);").count(), 1, "{v}");
    assert!(!v.contains("ENTRY POINT REFUSED"), "a 1-bit clk parameter was refused:\n{v}");
    assert!(!stderr_of(&out).contains("ENTRY POINT REFUSED"));
    assert_each_port_once(&v);
    assert_iverilog_accepts("clk_param", &v);
}

#[test]
fn on_clock_parameters_named_rst_n_and_en_are_the_boilerplate_ports() {
    // dsp_probe.t27's shape.
    let out = gen_verilog_source(
        "probe_params",
        "module probe_params;\n\npub var beat : bool = false;\n\n\
         fn on_clock(cfgmclk: bool, rst_n: bool, en: bool) -> void {\n    beat = cfgmclk;\n}\n",
    );
    assert!(out.status.success(), "gen-verilog failed: {}", stderr_of(&out));
    let v = stdout_of(&out);
    for name in ["clk", "rst_n", "en", "cfgmclk"] {
        assert_eq!(declarations_of(&v, name), 1, "`{name}` is not declared exactly once:\n{v}");
    }
    assert!(!v.contains("ENTRY POINT REFUSED"), "1-bit parameters were refused:\n{v}");
    assert_each_port_once(&v);
    assert_iverilog_accepts("probe_params", &v);
}

#[test]
fn a_wide_parameter_named_clk_is_refused_not_narrowed() {
    // FR-003: an 8-bit `clk` cannot be the 1-bit clk port.
    let out = gen_verilog_source(
        "wide_clk",
        "module wide_clk;\n\nfn on_comb(clk: u8) -> u8 {\n    return clk;\n}\n",
    );
    assert!(out.status.success(), "gen-verilog failed: {}", stderr_of(&out));
    let v = stdout_of(&out);
    assert_eq!(declarations_of(&v, "clk"), 1, "clk is not declared exactly once:\n{v}");
    assert!(
        !header(&v).iter().any(|l| l.contains("[7:0] clk")),
        "the 8-bit parameter became a port:\n{v}"
    );
    assert!(!v.contains("assign result"), "a refused entry point still drives result:\n{v}");
    assert!(
        v.contains("// ENTRY POINT REFUSED -- a parameter is named like a boilerplate port"),
        "the refusal is not written in the generated source:\n{v}"
    );
    assert!(
        stderr_of(&out)
            .contains("t27c gen-verilog: ENTRY POINT REFUSED -- clk: u8 -- named like the 1-bit `clk` port"),
        "the refusal is not reported on stderr: {}",
        stderr_of(&out)
    );
    assert_each_port_once(&v);
    assert_iverilog_accepts("wide_clk", &v);
}

#[test]
fn a_parameter_that_collides_with_nothing_keeps_its_own_port() {
    // FR-004: no collision, no change -- `x` is its own 8-bit port.
    let out = gen_verilog_source(
        "plain_param",
        "module plain_param;\n\nfn on_comb(x: u8) -> u8 {\n    return x;\n}\n",
    );
    assert!(out.status.success(), "gen-verilog failed: {}", stderr_of(&out));
    let v = stdout_of(&out);
    assert_eq!(
        header(&v),
        vec![
            "input  wire        clk,",
            "input  wire        rst_n,",
            "input  wire        en,",
            "input  wire [7:0] x,",
            "output wire        ready,",
            "output wire [7:0] result",
        ],
        "the header of a non-colliding entry point changed:\n{v}"
    );
}

#[test]
fn the_corpus_specs_that_collided_declare_each_port_once() {
    // The five specs master declared a port twice in (measured on 295b2c5c).
    let cases: [(&str, &[&str]); 5] = [
        ("specs/port/trinity/fpga/openxc7-synth/d_bank13_test.t27", &["clk"]),
        ("specs/port/trinity/fpga/openxc7-synth/d_direct.t27", &["clk"]),
        ("specs/port/trinity/fpga/openxc7-synth/led_simple_test.t27", &["clk"]),
        ("specs/port/trinity/fpga/openxc7-synth/led_test_inv.t27", &["clk"]),
        ("specs/port/fpga/verilog/dsp_probe.t27", &["rst_n", "en", "cfgmclk"]),
    ];
    for (rel, names) in cases {
        let out = gen_verilog_spec(rel);
        assert!(out.status.success(), "gen-verilog {rel} failed: {}", stderr_of(&out));
        let v = stdout_of(&out);
        for name in names {
            assert_eq!(declarations_of(&v, name), 1, "{rel}: `{name}` is not declared once");
        }
        assert_each_port_once(&v);
        assert_iverilog_accepts(rel.rsplit('/').next().unwrap_or(rel).trim_end_matches(".t27"), &v);
    }
    let bank = stdout_of(&gen_verilog_spec("specs/port/trinity/fpga/openxc7-synth/d_bank13_test.t27"));
    assert_eq!(bank.matches("assign result = on_comb(clk);").count(), 1, "{bank}");
}

#[test]
fn ternary_node_keeps_its_eight_inputs() {
    let v = stdout_of(&gen_verilog_spec("specs/igla/race/ternary_node.t27"));
    let inputs = header(&v).iter().filter(|l| l.starts_with("input")).count();
    assert_eq!(inputs, 8, "ternary_node's header changed:\n{v}");
    assert_each_port_once(&v);
}

/// The checked-in copy is exactly what gen-rust writes from
/// specs/compiler/verilog_entry_port_collision.t27: the decision lives in the spec.
#[test]
fn the_collision_rule_copy_is_what_gen_rust_writes() {
    let root = repo_root();
    let fresh = Command::new(t27c())
        .args(["gen-rust", "specs/compiler/verilog_entry_port_collision.t27"])
        .current_dir(&root)
        .output()
        .expect("run t27c gen-rust");
    assert!(fresh.status.success(), "{}", stderr_of(&fresh));
    let copy = fs::read(root.join("bootstrap/gen/rust/compiler/verilog_entry_port_collision.rs"))
        .expect("read the generated copy");
    assert!(
        fresh.stdout == copy,
        "bootstrap/gen/rust/compiler/verilog_entry_port_collision.rs drifted from its spec: regenerate it"
    );
}

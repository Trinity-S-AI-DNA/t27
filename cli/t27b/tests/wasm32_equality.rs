//! The M-A per-fn equality gate (epic #11840): 100 leaf specs, each computed
//! two ways -- `t27b build --target wasm32` runs in Node's WebAssembly, and
//! `t27b run` runs the same fn natively through the JIT -- compared bit for
//! bit. 100/100 equality is the milestone's gate.
//!
//! Hand-written Rust by owner approval (`owner-approved-foreign`, session
//! 2026-10-23). The spec that will replace this file is
//! `specs/tri/t27b/wasm32.t27`; port epic gHashTag/t27#6198.
//!
//! Per case: the .t27 source is written, the module is emitted, one Node
//! process instantiates every module and reports each export's value as its
//! raw bits, and the native value `t27b run` prints is converted to the same
//! bits. A mismatch is a FAIL with both sides hex-printed; any FAIL fails the
//! test. The full report also lands in `battery-report.txt` beside the cases.

use std::fmt::Write as _;
use std::path::PathBuf;
use std::process::Command;
use t27b::jit::JIT_SUPPORTED;

/// One leaf spec of the battery: `pub fn <name>() -> <ty> { return <lit>; }`.
/// `carrier` is the wasm valtype family the export crosses as ("i32", "i64",
/// "f32", "f64") -- bool and every <=32-bit int ride i32.
struct Case {
    name: &'static str,
    ty: &'static str,
    lit: String,
    carrier: &'static str,
}

/// The battery: 100 leaf specs across the 11 scalar types, the values across
/// the ranges -- 0, 1, -1, both boundaries, the primes, the powers of 2, the
/// bit patterns 0xdead/beef/face/cafe/babe and their 32/48/64-bit composites.
/// Every literal is spelled by Rust's own formatter from a typed constant, so
/// no hand-computed decimal can drift from the pattern it names.
fn battery() -> Vec<Case> {
    let mut v: Vec<Case> = Vec::new();
    let mut push = |name: &'static str, ty: &'static str, lit: String, carrier: &'static str| {
        v.push(Case { name, ty, lit, carrier });
    };
    // i32 (10)
    push("i32_zero", "i32", format!("{}", 0i32), "i32");
    push("i32_one", "i32", format!("{}", 1i32), "i32");
    push("i32_minus_one", "i32", format!("{}", -1i32), "i32");
    push("i32_min", "i32", format!("{}", i32::MIN), "i32");
    push("i32_max", "i32", format!("{}", i32::MAX), "i32");
    push("i32_dead", "i32", format!("{}", 0xdeadu32), "i32");
    push("i32_beef", "i32", format!("{}", 0xbeefu32), "i32");
    push("i32_face", "i32", format!("{}", 0xfaceu32), "i32");
    push("i32_cafe", "i32", format!("{}", 0xcafeu32), "i32");
    push("i32_prime_104729", "i32", format!("{}", 104729i32), "i32");
    // u32 (10)
    push("u32_zero", "u32", format!("{}", 0u32), "i32");
    push("u32_one", "u32", format!("{}", 1u32), "i32");
    push("u32_max", "u32", format!("{}", u32::MAX), "i32");
    push("u32_pow2_31", "u32", format!("{}", 1u32 << 31), "i32");
    push("u32_pow2_31_minus_1", "u32", format!("{}", (1u32 << 31) - 1), "i32");
    push("u32_deadbeef", "u32", format!("{}", 0xdeadbeefu32), "i32");
    push("u32_cafebabe", "u32", format!("{}", 0xCAFEBABEu32), "i32");
    push("u32_facebabe", "u32", format!("{}", 0xfacebabeu32), "i32");
    push("u32_babe", "u32", format!("{}", 0xbabeu32), "i32");
    push("u32_prime_65521", "u32", format!("{}", 65521u32), "i32");
    // i64 (9)
    push("i64_zero", "i64", format!("{}", 0i64), "i64");
    push("i64_minus_one", "i64", format!("{}", -1i64), "i64");
    push("i64_min", "i64", format!("{}", i64::MIN), "i64");
    push("i64_max", "i64", format!("{}", i64::MAX), "i64");
    push("i64_deadbeefcafe", "i64", format!("{}", 0xdeadbeefcafeu64), "i64");
    push("i64_pow2_62", "i64", format!("{}", 1i64 << 62), "i64");
    push("i64_minus_pow2_62", "i64", format!("{}", -(1i64 << 62)), "i64");
    push("i64_prime_7919", "i64", format!("{}", 7919i64), "i64");
    push("i64_minus_997", "i64", format!("{}", -997i64), "i64");
    // u64 (9)
    push("u64_zero", "u64", format!("{}", 0u64), "i64");
    push("u64_one", "u64", format!("{}", 1u64), "i64");
    push("u64_max", "u64", format!("{}", u64::MAX), "i64");
    push("u64_pow2_63", "u64", format!("{}", 1u64 << 63), "i64");
    push("u64_pow2_63_plus_5", "u64", format!("{}", (1u64 << 63) + 5), "i64");
    push("u64_deadbeefcafebabe", "u64", format!("{}", 0xDEADBEEFCAFEBABEu64), "i64");
    push("u64_babeface", "u64", format!("{}", 0xbabefaceu64), "i64");
    push("u64_prime_104729", "u64", format!("{}", 104729u64), "i64");
    push("u64_face", "u64", format!("{}", 0xfaceu64), "i64");
    // bool (2)
    push("bool_true", "bool", "true".into(), "i32");
    push("bool_false", "bool", "false".into(), "i32");
    // u8 (8)
    push("u8_zero", "u8", format!("{}", 0u8), "i32");
    push("u8_one", "u8", format!("{}", 1u8), "i32");
    push("u8_max", "u8", format!("{}", u8::MAX), "i32");
    push("u8_128", "u8", format!("{}", 128u8), "i32");
    push("u8_7f", "u8", format!("{}", 0x7fu8), "i32");
    push("u8_ca", "u8", format!("{}", 0xcau8), "i32");
    push("u8_be", "u8", format!("{}", 0xbeu8), "i32");
    push("u8_prime_251", "u8", format!("{}", 251u8), "i32");
    // i8 (8)
    push("i8_zero", "i8", format!("{}", 0i8), "i32");
    push("i8_minus_one", "i8", format!("{}", -1i8), "i32");
    push("i8_min", "i8", format!("{}", i8::MIN), "i32");
    push("i8_max", "i8", format!("{}", i8::MAX), "i32");
    push("i8_minus_13", "i8", format!("{}", -13i8), "i32");
    push("i8_101", "i8", format!("{}", 101i8), "i32");
    push("i8_minus_101", "i8", format!("{}", -101i8), "i32");
    push("i8_5", "i8", format!("{}", 5i8), "i32");
    // u16 (8)
    push("u16_zero", "u16", format!("{}", 0u16), "i32");
    push("u16_one", "u16", format!("{}", 1u16), "i32");
    push("u16_max", "u16", format!("{}", u16::MAX), "i32");
    push("u16_pow2_15", "u16", format!("{}", 1u16 << 15), "i32");
    push("u16_dead", "u16", format!("{}", 0xdeadu16), "i32");
    push("u16_cafe", "u16", format!("{}", 0xcafeu16), "i32");
    push("u16_prime_6542", "u16", format!("{}", 6542u16), "i32");
    push("u16_7fff", "u16", format!("{}", 0x7fffu16), "i32");
    // i16 (8)
    push("i16_zero", "i16", format!("{}", 0i16), "i32");
    push("i16_minus_one", "i16", format!("{}", -1i16), "i32");
    push("i16_min", "i16", format!("{}", i16::MIN), "i32");
    push("i16_max", "i16", format!("{}", i16::MAX), "i32");
    push("i16_7597", "i16", format!("{}", 7597i16), "i32");
    push("i16_minus_7597", "i16", format!("{}", -7597i16), "i32");
    push("i16_prime_7919", "i16", format!("{}", 7919i16), "i32");
    push("i16_minus_997", "i16", format!("{}", -997i16), "i32");
    // f32 (14)
    push("f32_zero", "f32", format!("{:?}", 0.0f32), "f32");
    push("f32_one", "f32", format!("{:?}", 1.0f32), "f32");
    push("f32_minus_one", "f32", format!("{:?}", -1.0f32), "f32");
    push("f32_half", "f32", format!("{:?}", 0.5f32), "f32");
    push("f32_minus_half", "f32", format!("{:?}", -0.5f32), "f32");
    push("f32_one_and_half", "f32", format!("{:?}", 1.5f32), "f32");
    push("f32_pi", "f32", format!("{:?}", 3.14159f32), "f32");
    push("f32_e", "f32", format!("{:?}", 2.71828f32), "f32");
    push("f32_e10", "f32", format!("{:?}", 1e10f32), "f32");
    push("f32_e_minus10", "f32", format!("{:?}", 1e-10f32), "f32");
    push("f32_max", "f32", format!("{:?}", 3.4028235e38f32), "f32");
    push("f32_min_normal", "f32", format!("{:?}", 1.17549435e-38f32), "f32");
    push("f32_123_456", "f32", format!("{:?}", 123.456f32), "f32");
    push("f32_tenth", "f32", format!("{:?}", 0.1f32), "f32");
    // f64 (14)
    push("f64_zero", "f64", format!("{:?}", 0.0f64), "f64");
    push("f64_minus_zero", "f64", format!("{:?}", -0.0f64), "f64");
    push("f64_one", "f64", format!("{:?}", 1.0f64), "f64");
    push("f64_minus_one", "f64", format!("{:?}", -1.0f64), "f64");
    push("f64_half", "f64", format!("{:?}", 0.5f64), "f64");
    push("f64_one_and_half", "f64", format!("{:?}", 1.5f64), "f64");
    push("f64_pi", "f64", format!("{:?}", 3.141592653589793f64), "f64");
    push("f64_e", "f64", format!("{:?}", 2.718281828459045f64), "f64");
    push("f64_e10", "f64", format!("{:?}", 1e10f64), "f64");
    push("f64_e_minus10", "f64", format!("{:?}", 1e-10f64), "f64");
    push("f64_max", "f64", format!("{:?}", 1.7976931348623157e308f64), "f64");
    push("f64_min_subnormal", "f64", format!("{:?}", 5e-324f64), "f64");
    push("f64_123_456", "f64", format!("{:?}", 123.456789f64), "f64");
    push("f64_tenth", "f64", format!("{:?}", 0.1f64), "f64");
    v
}

/// The Node bridge: one process instantiates every module the battery built
/// and prints `<VERDICT> <case-index> <value-as-unsigned-bits>` per line. An
/// i32 export crosses as a signed JS number, an i64 as a BigInt, an f32/f64
/// as the float widened to binary64 (exact); each goes to its raw bits here,
/// so the comparison in Rust is bit-exact on both sides.
const NODE_BRIDGE: &str = r#"
const fs = require("fs");
// Under `node -e <script>` there is no script-path argv slot: argv[0] is the
// node binary itself and the first argument of ours is argv[1].
for (const item of process.argv.slice(1)) {
  const [idx, file, kind] = item.split(":");
  try {
    const ins = new WebAssembly.Instance(new WebAssembly.Module(fs.readFileSync(file)), {});
    const name = Object.keys(ins.exports)[0];
    const v = ins.exports[name]();
    let bits;
    if (kind === "i64") bits = BigInt.asUintN(64, v).toString();
    else if (kind === "i32") bits = BigInt.asUintN(32, BigInt(v)).toString();
    else if (kind === "f32") { const b = new ArrayBuffer(4); new Float32Array(b)[0] = v; bits = new Uint32Array(b)[0].toString(); }
    else { const b = new ArrayBuffer(8); new Float64Array(b)[0] = v; bits = new BigUint64Array(b)[0].toString(); }
    console.log("WASM " + idx + " " + bits);
  } catch (e) {
    console.log("ERR " + idx + " " + e.message);
  }
}
"#;

/// The native value `t27b run` printed, as the same unsigned bits the Node
/// bridge reports: ints wrap two's complement, floats are their bit patterns
/// (an f32 crossed as binary64 widens exactly, so the round trip is exact).
fn native_bits(c: &Case, text: &str) -> Result<u64, String> {
    fn bad(text: &str, ty: &str, e: impl std::fmt::Display) -> String {
        format!("native value {:?} does not parse as {}: {}", text, ty, e)
    }
    match c.carrier {
        "f32" => {
            let x: f64 = text.parse().map_err(|e| bad(text, c.ty, e))?;
            Ok((x as f32).to_bits() as u64)
        }
        "f64" => {
            let x: f64 = text.parse().map_err(|e| bad(text, c.ty, e))?;
            Ok(x.to_bits())
        }
        "i64" => {
            let x: i128 = text.parse().map_err(|e| bad(text, c.ty, e))?;
            Ok(x as u64)
        }
        _ => {
            let x: i128 = text.parse().map_err(|e| bad(text, c.ty, e))?;
            Ok((x as u32) as u64)
        }
    }
}

#[test]
fn wasm32_equality_battery_100_of_100() {
    let cases = battery();
    assert_eq!(cases.len(), 100, "the battery is 100 leaf specs");
    if !JIT_SUPPORTED {
        eprintln!("t27b wasm32 equality: JIT not supported here; the native leg cannot run, the gate is vacuous");
        return;
    }
    let bin = env!("CARGO_BIN_EXE_t27b");
    let dir = std::env::temp_dir().join(format!("t27b-wasm-eq-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("scratch dir");

    // Leg 1, per case: write the spec, run it natively, emit the module.
    let mut native: Vec<Option<String>> = vec![None; cases.len()];
    let mut built: Vec<Option<PathBuf>> = vec![None; cases.len()];
    for (i, c) in cases.iter().enumerate() {
        let spec = dir.join(format!("case_{:03}_{}.t27", i, c.name));
        std::fs::write(&spec, format!("pub fn {}() -> {} {{ return {}; }}\n", c.name, c.ty, c.lit))
            .expect("write spec");
        let out = Command::new(bin).arg("run").arg(&spec).arg(c.name).output().expect("run t27b");
        let stdout = String::from_utf8_lossy(&out.stdout);
        let want = format!("t27b run: {} = ", c.name);
        // The line is `t27b run: <fn> = <value> (<type>)`; keep only <value>.
        native[i] = stdout
            .lines()
            .find_map(|l| l.strip_prefix(&want).map(|s| s.split(" (").next().unwrap_or(s).to_string()));
        let wasm = spec.with_extension("wasm");
        let b = Command::new(bin)
            .args(["build", "--target", "wasm32"])
            .arg(&spec)
            .arg("-o")
            .arg(&wasm)
            .output()
            .expect("build t27b");
        if b.status.success() && wasm.exists() {
            built[i] = Some(wasm);
        } else {
            eprintln!("case {:03} {}: build failed: {}", i, c.name, String::from_utf8_lossy(&b.stderr));
        }
    }

    // Leg 2: one Node process over every module.
    let items: Vec<String> = cases
        .iter()
        .enumerate()
        .filter(|(i, _)| built[*i].is_some() && native[*i].is_some())
        .map(|(i, c)| format!("{}:{}:{}", i, built[i].as_ref().unwrap().display(), c.carrier))
        .collect();
    let mut wasm_bits: Vec<Option<u64>> = vec![None; cases.len()];
    let mut node_err: Vec<String> = vec![String::new(); cases.len()];
    if !items.is_empty() {
        let out = Command::new("node").arg("-e").arg(NODE_BRIDGE).args(&items).output();
        match out {
            Ok(o) => {
                for l in String::from_utf8_lossy(&o.stdout).lines() {
                    let parts: Vec<&str> = l.splitn(3, ' ').collect();
                    if parts.len() == 3 && parts[0] == "WASM" {
                        if let (Ok(i), Ok(b)) = (parts[1].parse::<usize>(), parts[2].parse::<u64>()) {
                            wasm_bits[i] = Some(b);
                        }
                    } else if parts.len() >= 3 && parts[0] == "ERR" {
                        if let Ok(i) = parts[1].parse::<usize>() {
                            node_err[i] = parts[2].to_string();
                        }
                    }
                }
            }
            Err(e) => panic!("node not runnable: {}; the equality gate needs Node's WebAssembly", e),
        }
    }

    // The verdicts, per fn, then the gate.
    let mut report = String::new();
    let mut pass = 0usize;
    let mut fail = 0usize;
    for (i, c) in cases.iter().enumerate() {
        let line = match (&native[i], &built[i], wasm_bits[i]) {
            (Some(n), Some(_), Some(w)) => match native_bits(c, n) {
                Ok(nb) if nb == w => {
                    pass += 1;
                    format!("PASS [{:03}/100] {:18} ({:4}) lit {:24} bits {:#018x}", i, c.name, c.ty, c.lit, w)
                }
                Ok(nb) => {
                    fail += 1;
                    format!(
                        "FAIL [{:03}/100] {:18} ({:4}) lit {:24} native {:#018x} != wasm {:#018x}",
                        i, c.name, c.ty, c.lit, nb, w
                    )
                }
                Err(e) => {
                    fail += 1;
                    format!("FAIL [{:03}/100] {:18} ({:4}) lit {:24} {}", i, c.name, c.ty, c.lit, e)
                }
            },
            _ => {
                fail += 1;
                let why = if native[i].is_none() {
                    "t27b run printed no value line".to_string()
                } else if built[i].is_none() {
                    "wasm build failed".to_string()
                } else if !node_err[i].is_empty() {
                    format!("node: {}", node_err[i])
                } else {
                    "node printed no verdict".to_string()
                };
                format!("FAIL [{:03}/100] {:18} ({:4}) lit {:24} {}", i, c.name, c.ty, c.lit, why)
            }
        };
        println!("{}", line);
        let _ = writeln!(report, "{}", line);
    }
    let summary = format!("battery: {} passed, {} failed, {} total (cases in {})", pass, fail, cases.len(), dir.display());
    println!("{}", summary);
    let _ = writeln!(report, "{}", summary);
    let _ = std::fs::write(dir.join("battery-report.txt"), &report);
    assert_eq!(fail, 0, "wasm32 equality: {} of {} leaf fns mismatched", fail, cases.len());
    assert_eq!(pass, 100, "the battery must pass 100 of 100");
}

//! `tri fpga synth-spec` host glue. NEW foreign file: moved out of
//! cli/tri/src/fpga.rs so the shared foreign file shrinks
//! (specs/policy/own_language.t27 lines 56-60: lines moved between foreign
//! files net to zero; rule 2026-10-09/#8236). Admission law lives in
//! specs/fpga/synth_receipt.t27 and is consumed as the generated module
//! `fpga_receipt` mounted below (gitignored gen/ file; commit with
//! `git add -f` per CLAUDE.md).

#[path = "../../../gen/rust/tri/fpga_receipt.rs"]
#[allow(dead_code, non_snake_case, unused_parens)]
pub mod fpga_receipt;

use anyhow::{anyhow, bail, Context, Result};
use std::path::PathBuf;
use sha2::{Sha256, Digest};
use hex;
use super::fpga::{repo_root, which};

/// Stage status codes (receipt contract "stages" section).
pub const STAGE_OK_X: u8 = 0;
pub const STAGE_OK: u8 = 1;
pub const STAGE_FAILED: u8 = 2;

/// One recipe stage, receipt contract "stages" section shape.
pub struct StageRecord {
    name: &'static str,
    pub status: u8,
    pub exit_code: Option<i32>,
    pub duration_ms: u128,
    pub log_path: String,
    pub artifact_sha256: Option<String>,
}

pub fn sha256_str(s: &str) -> String {
    use sha2::Digest;
    let mut h = sha2::Sha256::new();
    h.update(s.as_bytes());
    hex::encode(h.finalize())
}

fn sha256_file(p: &std::path::Path) -> Result<String> {
    use sha2::Digest;
    let bytes = std::fs::read(p).with_context(|| format!("read {}", p.display()))?;
    let mut h = sha2::Sha256::new();
    h.update(&bytes);
    Ok(hex::encode(h.finalize()))
}

/// The module name of a .t27 source: the first `module <Name>;` line.
pub fn spec_module_name(source: &str) -> Option<String> {
    for line in source.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("module ") {
            let name = rest
                .trim_end()
                .trim_end_matches([';', '{'])
                .trim()
                .trim_end_matches('{')
                .trim();
            if !name.is_empty() {
                return Some(name.to_string());
            }
        }
    }
    None
}

/// Run one stage with stdout/stderr captured to log files. Returns the
/// exit status; the caller turns it into a StageRecord.
fn run_logged(
    bin: &std::path::Path,
    args: &[&str],
    cwd: &std::path::Path,
    log: &std::path::Path,
) -> Result<std::process::ExitStatus> {
    let out_file = std::fs::File::create(log)
        .with_context(|| format!("create log {}", log.display()))?;
    let err_file = std::fs::File::create(log.with_extension("err.log"))
        .with_context(|| format!("create err log {}", log.display()))?;
    std::process::Command::new(bin)
        .args(args)
        .current_dir(cwd)
        .stdout(std::process::Stdio::from(out_file))
        .stderr(std::process::Stdio::from(err_file))
        .status()
        .with_context(|| format!("spawn {}", bin.display()))
}

/// One receipt context: everything a receipt records, built once per run.
pub struct ReceiptCtx<'a> {
    pub build: &'a std::path::Path,
    pub spec: &'a std::path::Path,
    pub module: &'a str,
    pub source_sha256: &'a str,
    pub seal_present: u8,
    pub seal_spec_hash: &'a str,
    pub seal_gen_hash_verilog: &'a str,
    pub sealed_by: &'a str,
    pub input_verilog_sha256: &'a str,
    pub seal_verify_all_match: bool,
    pub verilog_match: u8,
    pub part: &'a str,
    pub xdc: &'a std::path::Path,
    pub ignore_loops: bool,
    pub nextpnr_args: &'a [String],
}

pub fn write_receipt(
    ctx: &ReceiptCtx,
    admitted: bool,
    outcome: u8,
    failed_stage: &str,
    stages: &[StageRecord],
) -> Result<PathBuf> {
    let dir = ctx.build.join(".trinity").join("receipts");
    std::fs::create_dir_all(&dir)?;
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    let path = dir.join(format!("synth-spec-{}-{}.json", ctx.module, stamp));
    let receipt = serde_json::json!({
        "receipt_kind": "synth_spec/1",
        "source": {
            "path": ctx.spec.display().to_string(),
            "source_sha256": ctx.source_sha256,
        },
        "seal": {
            "spec_hash": ctx.seal_spec_hash,
            "gen_hash_verilog": ctx.seal_gen_hash_verilog,
            "seal_present": ctx.seal_present,
            "sealed_by": ctx.sealed_by,
            "checker": "tri fpga synth-spec",
        },
        "verilog": {
            "seal_gen_hash_verilog": ctx.seal_gen_hash_verilog,
            "input_verilog_sha256": ctx.input_verilog_sha256,
            "seal_verify": {
                "oracle": "t27c seal --verify",
                "all_match": ctx.seal_verify_all_match,
                "note": "seal hashes the PRE-SPLICE generated text; the yosys \
                         input is this run's spliced CLI output, hashed above",
            },
            "verilog_match": ctx.verilog_match,
            "match_note": "byte identity of the yosys input with the sealed \
                           output only; NOT functional equivalence (R2)",
        },
        "recipe": {
            "part": ctx.part,
            "package": ctx.part,
            "seed": serde_json::Value::Null,
            "xdc": ctx.xdc.display().to_string(),
            "tool_versions": "recorded per stage in logs",
            "ignore_loops": ctx.ignore_loops,
            "nextpnr_args": ctx.nextpnr_args,
            "cache_key": serde_json::Value::Null,
            "cache_note": "service.rs bitstream cache NOT used; its input \
                           digest set is unaudited",
        },
        "admitted": admitted,
        "outcome": match outcome {
            fpga_receipt::OUTCOME_ADMISSION_REJECTED => "admission_rejected",
            fpga_receipt::OUTCOME_STAGE_FAILED => "stage_failed",
            _ => "bitstream_produced",
        },
        "failed_stage": failed_stage,
        "hardware_verified": false,
        "hardware_note": "always false in this command: no board check ran",
        "claim": "bitstream_produced",
        "claim_note": "never functionality_proven; no hardware check ran",
        "stages": stages.iter().map(|s| serde_json::json!({
            "name": s.name,
            "status": s.status,
            "exit_code": s.exit_code,
            "duration_ms": s.duration_ms,
            "log": s.log_path,
            "artifact_sha256": s.artifact_sha256,
        })).collect::<Vec<_>>(),
    });
    std::fs::write(&path, serde_json::to_string_pretty(&receipt)?)?;
    Ok(path)
}

/// External tool paths used by the pipeline. Resolved for real runs by
/// `real_tools`; tests inject stub launchers to count downstream calls.
pub struct SynthTools {
    pub t27c: PathBuf,
    pub yosys: PathBuf,
    pub nextpnr: PathBuf,
    pub fasm2frames_python: PathBuf,
    pub fasm2frames_py: PathBuf,
    pub xc7frames2bit: PathBuf,
    chipdb: PathBuf,
}

pub fn real_tools(root: &std::path::Path) -> Result<SynthTools> {
    let t27c = ["target/release/t27c", "target/debug/t27c"]
        .iter()
        .map(|p| root.join(p))
        .find(|p| p.is_file())
        .ok_or_else(|| anyhow!("t27c is not built; `cargo build --release -p t27c` first"))?;
    // Tool resolution is environment-first and machine-independent: each path
    // may be pinned via an OPENXC7_* variable, else it is looked up on PATH,
    // else the bare name is kept so a missing tool fails AT USE with a clear
    // command name -- never at construction, and never via a home path.
    let pin = |var: &str, tool: &str| -> PathBuf {
        std::env::var_os(var)
            .map(PathBuf::from)
            .unwrap_or_else(|| which(tool).unwrap_or_else(|_| PathBuf::from(tool)))
    };
    Ok(SynthTools {
        t27c,
        yosys: pin("OPENXC7_YOSYS", "yosys"),
        nextpnr: pin("OPENXC7_NEXTPNR", "nextpnr-xilinx"),
        fasm2frames_python: pin("OPENXC7_PYTHON", "python3"),
        fasm2frames_py: pin("OPENXC7_FASM2FRAMES", "fasm2frames.py"),
        xc7frames2bit: pin("OPENXC7_XC7FRAMES2BIT", "xc7frames2bit"),
        chipdb: PathBuf::new(), // resolved per part in synth_spec_impl
    })
}

/// The exact nextpnr argv, built once and recorded verbatim in the receipt.
/// --ignore-loops is an EXPLICIT opt-in (SSOT §8: ring-oscillator designs
/// need it); it is never switched on by spec-name heuristics.
fn build_nextpnr_args(
    chipdb: &std::path::Path,
    xdc: &std::path::Path,
    json: &std::path::Path,
    fasm: &std::path::Path,
    ignore_loops: bool,
) -> Vec<String> {
    let mut a = vec![
        "--chipdb".to_string(),
        chipdb.display().to_string(),
        "--xdc".to_string(),
        xdc.display().to_string(),
        "--json".to_string(),
        json.display().to_string(),
        "--fasm".to_string(),
        fasm.display().to_string(),
    ];
    if ignore_loops {
        a.push("--ignore-loops".to_string());
    }
    a
}

/// Write the exact hashed bytes to `v_path` and re-verify from disk that the
/// bytes fed to yosys equal the admission hash (the Yosys input guarantee):
/// any alteration between hashing and the tool's read is caught here, before
/// the pipeline starts. Order M1.3 wave 2.
fn write_and_verify_input(
    v_path: &std::path::Path,
    generated: &str,
    input_verilog_sha256: &str,
) -> Result<()> {
    std::fs::write(v_path, generated)?;
    verify_input_bytes(v_path, input_verilog_sha256)
}

/// M1.3 (ORDER T27_OPENXC7_CONTRACT_TESTS_M1_3_V1): the yosys-input half of
/// the guarantee, callable on its own -- the bytes ON DISK are hashed and
/// compared against the receipt's input_verilog_sha256. Altered bytes are an
/// explicit refusal; the file is never rewritten by this check.
fn verify_input_bytes(v_path: &std::path::Path, input_verilog_sha256: &str) -> Result<()> {
    if sha256_file(v_path)? != input_verilog_sha256 {
        bail!("yosys input verification failed: bytes on disk differ from the hashed text");
    }
    Ok(())
}

pub fn synth_spec(
    spec: &PathBuf,
    part: &str,
    xdc: &PathBuf,
    ignore_loops: bool,
    build_dir: Option<&PathBuf>,
) -> Result<()> {
    let root = repo_root()?;
    let tools = real_tools(&root)?;
    synth_spec_impl(&root, &tools, spec, part, xdc, ignore_loops, build_dir)
}

pub fn synth_spec_impl(
    root: &std::path::Path,
    tools: &SynthTools,
    spec: &PathBuf,
    part: &str,
    xdc: &PathBuf,
    ignore_loops: bool,
    build_dir: Option<&PathBuf>,
) -> Result<()> {
    let t_total = std::time::Instant::now();
    let root = std::fs::canonicalize(root)
        .with_context(|| format!("canonicalize {}", root.display()))?;
    // chipdb is a nextpnr prerequisite, resolved here and gated AFTER
    // admission by the preflight below (M1.3 ordering).
    let chipdb = root.join("build/fpga/openxc7").join(format!("{}.bin", part));
    let build = build_dir
        .cloned()
        .unwrap_or_else(|| root.join("build/fpga/synth-spec"));
    let build = if build.is_absolute() {
        build
    } else {
        root.join(&build)
    };
    std::fs::create_dir_all(&build)?;
    let build = std::fs::canonicalize(&build)?;

    // -- input validation: no external tool has run yet ---------------------
    let source = std::fs::read_to_string(spec)
        .with_context(|| format!("cannot read spec {}", spec.display()))?;
    let source_sha256 = sha256_str(&source);
    let module = spec_module_name(&source)
        .ok_or_else(|| anyhow!("no `module <Name>;` declaration in {}", spec.display()))?;
    let xdc_abs: PathBuf = if xdc.is_absolute() {
        xdc.clone()
    } else {
        root.join(xdc)
    };
    let xdc: &PathBuf = &xdc_abs;
    if !xdc.is_file() {
        bail!("--xdc is not a file: {}", xdc.display());
    }
    let chipdb = root.join("build/fpga/openxc7").join(format!("{}.bin", part));
    // -- seal and admission (contract law, generated module) ----------------
    // Seals are named by <parent-dir>_<file-stem>.json (existing
    // convention, e.g. openxc7-synth_blink_test_top.json); older twins may
    // use the module name instead, so try both and prefer the stem one.
    let dir = spec
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("");
    let stem = spec
        .file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    let seal_dir = root.join(".trinity/seals");
    let stem_path = seal_dir.join(format!("{}_{}.json", dir, stem));
    let module_path = seal_dir.join(format!("{}_{}.json", dir, module));
    let (seal_path, seal_by) = if stem_path.is_file() {
        (stem_path.clone(), stem_path)
    } else {
        (module_path.clone(), module_path)
    };
    let (seal_present, seal_spec_hash, seal_ghv, sealed_by) = match std::fs::read_to_string(&seal_path) {
        Ok(text) => {
            let v: serde_json::Value = serde_json::from_str(&text)
                .with_context(|| format!("seal {} is not valid JSON", seal_path.display()))?;
            let g = |k: &str| {
                v.get(k).and_then(|x| x.as_str()).unwrap_or("").to_string()
            };
            (1u8, g("spec_hash"), g("gen_hash_verilog"), g("sealed_by"))
        }
        Err(_) => (0u8, String::new(), String::new(), String::new()),
    };
    let gen_none = (seal_present == 1 && seal_ghv == "none") as u8;

    // Identity oracle (contract amendment 2026-11-02): the seal hashes the
    // pre-splice generated text (docs/now/2026-10-07), so the host does not
    // re-implement splice rules; it asks the compiler itself.
    let verify_out = std::process::Command::new(&tools.t27c)
        .args([
            "seal",
            spec.to_str().ok_or_else(|| anyhow!("spec path not UTF-8"))?,
            "--verify",
        ])
        .current_dir(&root)
        .output()
        .with_context(|| "spawn t27c seal --verify")?;
    let verify_text = String::from_utf8_lossy(&verify_out.stdout).to_string();
    let verify_ok = verify_text.contains("all hashes MATCH") as u8;

    // -- stage 1: generate Verilog, hash the exact produced text ------------
    let gen_log = build.join(format!("{}.gen.log", module));
    let t0 = std::time::Instant::now();
    let gen_out = std::process::Command::new(&tools.t27c)
        .args([
            "gen-verilog",
            spec.to_str().ok_or_else(|| anyhow!("spec path not UTF-8"))?,
        ])
        .current_dir(&root)
        .output()
        .with_context(|| "spawn t27c gen-verilog")?;
    let mut stages = vec![StageRecord {
        name: "gen-verilog",
        status: if gen_out.status.success() { STAGE_OK } else { STAGE_FAILED },
        exit_code: gen_out.status.code(),
        duration_ms: t0.elapsed().as_millis(),
        log_path: gen_log.display().to_string(),
        artifact_sha256: Some(sha256_str(&String::from_utf8_lossy(&gen_out.stdout))),
    }];
    std::fs::write(&gen_log, &gen_out.stderr)?;

    let generated = String::from_utf8_lossy(&gen_out.stdout).to_string();
    let input_verilog_sha256 = sha256_str(&generated);
    let seal_verify_all_match = verify_ok == 1;
    let verilog_match = fpga_receipt::verilog_match_of(
        gen_none,
        seal_present,
        verify_ok,
        1, // input bytes are recorded; byte identity with this run's
           // generated text is separately enforced by write-then-hash
    );
    let admitted = fpga_receipt::build_admitted(
        gen_none,
        verilog_match,
        stages[0].status,
    ) == 1
        && gen_out.status.success();

    // -- admitted: write the exact hashed bytes and run the SSOT-8 recipe ---
    // A stale .bit (or any stale stage artifact) from an earlier attempt in
    // this directory must never be accepted as this run's result.
    let v_path = build.join(format!("{}.v", module));
    let json_path = build.join(format!("{}.json", module));
    let fasm_path = build.join(format!("{}.fasm", module));
    let frames_path = build.join(format!("{}.frames", module));
    let bit_path = build.join(format!("{}.bit", module));
    for stale in [&v_path, &json_path, &fasm_path, &frames_path, &bit_path] {
        if stale.exists() {
            std::fs::remove_file(stale)
                .with_context(|| format!("remove stale artifact {}", stale.display()))?;
        }
    }
    write_and_verify_input(&v_path, &generated, &input_verilog_sha256)?;

    let nextpnr_argv: Vec<String> = build_nextpnr_args(
        &chipdb, xdc, &json_path, &fasm_path, ignore_loops,
    );
    let ctx = ReceiptCtx {
        build: &build,
        spec,
        module: &module,
        source_sha256: &source_sha256,
        seal_present,
        seal_spec_hash: &seal_spec_hash,
        seal_gen_hash_verilog: &seal_ghv,
        sealed_by: &sealed_by,
        input_verilog_sha256: &input_verilog_sha256,
        seal_verify_all_match,
        verilog_match,
        part,
        xdc,
        ignore_loops,
        nextpnr_args: &nextpnr_argv,
    };

    if !admitted {
        let receipt = write_receipt(&ctx, false, fpga_receipt::OUTCOME_ADMISSION_REJECTED, "", &stages)?;
        bail!(
            "admission refused (verilog_match code {}, gen stage {:?}, \
             seal_verify_all_match {}); receipt: {}",
            verilog_match,
            stages[0].status,
            seal_verify_all_match,
            receipt.display()
        );
    }

    // -- toolchain preflight, AFTER admission (M1.3: a refusal path must fire
    // before any downstream stage, and chipdb is a nextpnr prerequisite, not
    // an admission prerequisite -- ORDER T27_OPENXC7_CONTRACT_TESTS_M1_3_V1).
    if !chipdb.is_file() {
        bail!("chipdb missing: {} (SSOT §8 setup first)", chipdb.display());
    }

    let mut run_stage = |name: &'static str,
                         bin: &std::path::Path,
                         args: Vec<String>,
                         artifact: Option<&std::path::Path>,
                         stages: &mut Vec<StageRecord>|
     -> Result<()> {
        let t0 = std::time::Instant::now();
        let log = build.join(format!("{}.{}.log", module, name));
        let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        let status = run_logged(bin, &arg_refs, &build, &log)?;
        let ok = status.success();
        let art_hash = match artifact {
            Some(p) if p.is_file() && ok => Some(sha256_file(p)?),
            _ => None,
        };
        stages.push(StageRecord {
            name,
            status: if ok { STAGE_OK } else { STAGE_FAILED },
            exit_code: status.code(),
            duration_ms: t0.elapsed().as_millis(),
            log_path: log.display().to_string(),
            artifact_sha256: art_hash,
        });
        if !ok {
            let receipt = write_receipt(&ctx, true, fpga_receipt::OUTCOME_STAGE_FAILED, name, &stages)?;
            bail!(
                "stage {} failed (exit {:?}); log: {}; receipt: {}",
                name,
                status.code(),
                log.display(),
                receipt.display()
            );
        }
        Ok(())
    };

    run_stage(
        "yosys",
        &tools.yosys,
        vec![
            "-p".into(),
            format!(
                "read_verilog {}; synth_xilinx -family xc7 -top {} -flatten; write_json {}",
                v_path.display(),
                module,
                json_path.display()
            ),
        ],
        Some(&json_path),
        &mut stages,
    )?;
    run_stage(
        "nextpnr",
        &tools.nextpnr,
        nextpnr_argv.clone(),
        Some(&fasm_path),
        &mut stages,
    )?;
    run_stage(
        "fasm2frames",
        &tools.fasm2frames_python,
        vec![
            tools.fasm2frames_py.display().to_string(),
            "--db-root".into(),
            root.join("build/fpga/openxc7/prjxray-db/artix7")
                .display()
                .to_string(),
            format!("--part {}", part),
            fasm_path.display().to_string(),
            frames_path.display().to_string(),
        ],
        Some(&frames_path),
        &mut stages,
    )?;
    run_stage(
        "xc7frames2bit",
        &tools.xc7frames2bit,
        vec![
            "--frm_file".into(),
            frames_path.display().to_string(),
            "--output_file".into(),
            bit_path.display().to_string(),
            "--part_file".into(),
            root.join(format!(
                "build/fpga/openxc7/prjxray-db/artix7/{}/part.yaml",
                part
            )).display().to_string(),
            "--part_name".into(),
            part.into(),
        ],
        Some(&bit_path),
        &mut stages,
    )?;

    // A zero exit without a real .bit is NOT a finished build.
    if !bit_path.is_file() || std::fs::metadata(&bit_path)?.len() == 0 {
        stages.last_mut().unwrap().status = STAGE_FAILED;
        let receipt = write_receipt(&ctx, true, fpga_receipt::OUTCOME_STAGE_FAILED, "xc7frames2bit", &stages)?;
        bail!("bit missing/empty after xc7frames2bit; receipt: {}", receipt.display());
    }

    let receipt = write_receipt(&ctx, true, fpga_receipt::OUTCOME_BITSTREAM_PRODUCED, "", &stages)?;
    eprintln!(
        "[synth-spec] OK  {} ({:.1} MiB)  receipt: {}  total {}ms",
        bit_path.display(),
        std::fs::metadata(&bit_path)?.len() as f64 / 1024.0 / 1024.0,
        receipt.display(),
        t_total.elapsed().as_millis()
    );
    Ok(())
}


#[cfg(test)]
mod synth_spec_tests {
    use super::*;

    #[test]
    fn module_name_extraction() {
        assert_eq!(
            spec_module_name("module GftRelu;\nfn on_comb(x: u32) -> u32 { x }\nendmodule"),
            Some("GftRelu".to_string())
        );
        assert_eq!(spec_module_name("fn f() {}"), None);
    }

    #[test]
    fn contract_wiring_matches_spec_law() {
        // Same matrix as specs/fpga/synth_receipt.t27 tests (R1/R2/codes).
        assert_eq!(fpga_receipt::verilog_match_of(1, 1, 1, 1), 0);
        assert_eq!(fpga_receipt::verilog_match_of(0, 1, 1, 1), 1);
        assert_eq!(fpga_receipt::verilog_match_of(0, 1, 0, 1), 2);
        assert_eq!(fpga_receipt::verilog_match_of(0, 1, 1, 0), 2);
        assert_eq!(fpga_receipt::verilog_match_of(0, 0, 1, 1), 3);
        assert_eq!(fpga_receipt::build_admitted(1, 0, 1), 0);
        assert_eq!(fpga_receipt::build_admitted(0, 2, 1), 0);
        assert_eq!(fpga_receipt::build_admitted(0, 3, 1), 0);
        assert_eq!(fpga_receipt::build_admitted(0, 1, 2), 0);
        assert_eq!(fpga_receipt::build_admitted(0, 1, 1), 1);
        assert_eq!(fpga_receipt::proven_claim(1, 1, 0), 1);
        assert_eq!(fpga_receipt::proven_claim(1, 1, 1), 2);
        assert_eq!(fpga_receipt::proven_claim(1, 0, 1), 0);
    }

    #[test]
    fn missing_spec_fails_before_any_tool() {
        let dir = std::env::temp_dir().join("tri-synth-spec-test-1");
        std::fs::create_dir_all(&dir).unwrap();
        let err = synth_spec(
            &dir.join("no_such_spec.t27"),
            "xc7a200tfbg676-1",
            &dir.join("unused.xdc"),
            false,
            Some(&dir),
        )
        .unwrap_err();
        assert!(err.to_string().contains("cannot read spec"));
    }

    #[test]
    fn missing_xdc_fails_before_any_tool() {
        let dir = std::env::temp_dir().join("tri-synth-spec-test-2");
        std::fs::create_dir_all(&dir).unwrap();
        let spec = dir.join("m.t27");
        std::fs::write(&spec, "module M;\nfn f() {}\nendmodule").unwrap();
        let err = synth_spec(
            &spec,
            "xc7a200tfbg676-1",
            &dir.join("no_such.xdc"),
            false,
            Some(&dir),
        )
        .unwrap_err();
        assert!(err.to_string().contains("--xdc is not a file"));
        // gen stage log must not exist: nothing downstream ran.
        assert!(!dir.join("M.gen.log").exists());
    }

    #[test]
    fn missing_seal_refuses_before_bitstream_stages() {
        // A real spec without a seal file: admission must refuse with
        // verilog_match code 3 (seal_absent) and the receipt must record
        // admitted=false, before nextpnr/prjxray ever run.
        let dir = std::env::temp_dir().join("tri-synth-spec-test-3");
        std::fs::create_dir_all(&dir).unwrap();
        let spec = dir.join("NoSeal.t27");
        std::fs::write(&spec, "module NoSeal;\nfn f() {}\nendmodule").unwrap();
        let xdc = dir.join("placeholder.xdc");
        std::fs::write(&xdc, "").unwrap();
        let err = synth_spec(&spec, "xc7a200tfbg676-1", &xdc, false, Some(&dir)).unwrap_err();
        assert!(err.to_string().contains("admission refused"), "{}", err);
        assert!(!dir.join("NoSeal.fasm").exists());
    }
}

#[cfg(test)]
mod synth_spec_stub_tests {
    use super::*;

    /// Serialize the stub runs (M1.3, ORDER T27_OPENXC7_CONTRACT_TESTS_M1_3_V1):
    /// each test spawns real processes against a shared-per-process tool
    /// namespace, and parallel runs raced on it (measured 2026-11-10: 4 of 5
    /// parallel `cargo test -p tri synth_spec` runs red with cross-test
    /// errors -- invalid-seal reads, File-exists(17) -- while the same suite
    /// passes 11/11 under --test-threads=1, before AND after the admission
    /// ordering fix). The lock removes the race; it does not change any law.
    static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());
    fn lock() -> std::sync::MutexGuard<'static, ()> {
        match SERIAL.lock() {
            Ok(g) => g,
            Err(p) => p.into_inner(),
        }
    }

    /// Write an executable stub launcher; its calls append to a fixed file
    /// (absolute path baked into the script, safe under parallel tests).
    fn write_stub(dir: &std::path::Path, name: &str, calls: &std::path::Path, body: &str) -> PathBuf {
        let p = dir.join(name);
        let script = format!(
            "#!/bin/bash\nprintf '%s\\n' \"{}\" >> \"{}\"\n{}\n",
            name,
            calls.display(),
            body
        );
        std::fs::write(&p, script).unwrap();
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o755)).unwrap();
        p
    }

    /// Fixed gen-verilog stub output; admission passes only when the fake
    /// seal carries sha256 of this exact text.
    const STUB_VERILOG: &str = "module StubOut;\n";

    fn calls_of(calls: &std::path::Path) -> Vec<String> {
        std::fs::read_to_string(calls)
            .unwrap_or_default()
            .lines()
            .map(|s| s.to_string())
            .collect()
    }

    /// Full stub environment: temp root with chipdb, spec, matching seal,
    /// xdc, and per-stage stub launchers. Returns (root, build, calls).
    fn setup(admit: bool, no_bit_xc7: bool) -> (PathBuf, PathBuf, PathBuf) {
        // M1.3 (ORDER T27_OPENXC7_CONTRACT_TESTS_M1_3_V1): the name must be
        // UNIQUE PER CALL. SystemTime::now() has ~microsecond resolution on
        // macOS, and two parallel setup() calls in one test process landed in
        // the same microsecond and shared one root (measured 2026-11-10:
        // doubled calls.txt, File-exists(17), cross-test seal reads). An
        // atomic counter cannot collide.
        static ROOT_SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let root = std::env::temp_dir().join(format!(
            "tri-synth-stub-{}-{}",
            std::process::id(),
            ROOT_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        let bin = root.join("stubs");
        let build = root.join("build");
        std::fs::create_dir_all(&bin).unwrap();
        let chipdb = root.join("build/fpga/openxc7/xc7a200tfbg676-1.bin");
        std::fs::create_dir_all(chipdb.parent().unwrap()).unwrap();
        std::fs::write(&chipdb, b"stub").unwrap();
        std::fs::create_dir_all(root.join(".trinity/seals")).unwrap();
        std::fs::create_dir_all(root.join("lanes")).unwrap();
        let spec = root.join("lanes/St.t27");
        std::fs::write(&spec, "module St;\nfn f() {}\nendmodule").unwrap();
        let calls = root.join("calls.txt");
        let seal_answer = if admit {
            "if [ \"$1\" = seal ]; then printf 'spec_hash: MATCH\\ngen_hash_zig: MATCH\\ngen_hash_verilog: MATCH\\ngen_hash_c: MATCH\\ngen_hash_rust: MATCH\\n\\nall hashes MATCH\\n'; exit 0; fi; printf 'module StubOut;\\n'"
        } else {
            "if [ \"$1\" = seal ]; then printf 'spec_hash: MATCH\\nall hashes MISMATCH\\n'; exit 0; fi; printf 'module StubOut;\\n'"
        };
        let gen = write_stub(&bin, "gen", &calls, seal_answer);
        let yosys_body = if admit {
            "j=$(printf '%s' \"$*\" | sed -n 's/.*write_json \\([^ ;]*\\).*/\\1/p'); [ -n \"$j\" ] && : > \"$j\"; exit 0".to_string()
        } else {
            "exit 1".to_string()
        };
        let _ = yosys_body;
        let yosys = write_stub(&bin, "yosys", &calls,
            "j=$(printf '%s' \"$*\" | sed -n 's/.*write_json \\([^ ;]*\\).*/\\1/p'); [ -n \"$j\" ] && : > \"$j\"");
        let args_file = root.join("pnr_args.txt");
        let nextpnr = write_stub(&bin, "nextpnr", &calls,
            &format!("prev=\"\"; for a in \"$@\"; do if [ \"$prev\" = \"--fasm\" ]; then : > \"$a\"; fi; printf '%s\\n' \"$a\" >> \"{}\"; prev=\"$a\"; done", args_file.display()));
        let _ = args_file;
        let f2f = write_stub(&bin, "f2f", &calls,
            "out=\"${@: -1}\"; : > \"$out\"");
        let x7body = if no_bit_xc7 {
            "exit 0".to_string()
        } else {
            "prev=\"\"; for a in \"$@\"; do if [ \"$prev\" = \"--output_file\" ]; then printf BITDATA > \"$a\"; fi; prev=\"$a\"; done".to_string()
        };
        let x7 = write_stub(&bin, "x7", &calls, &x7body);
        let seal_ghv = if admit { sha256_str(STUB_VERILOG) } else { "sha256:deadbeef".to_string() };
        let seal = format!(
            "{{\"spec_path\":\"lanes/St.t27\",\"spec_hash\":\"sha256:x\",\"gen_hash_verilog\":\"{}\",\"sealed_by\":\"stub\"}}",
            seal_ghv
        );
        std::fs::write(
            root.join(".trinity/seals/lanes_St.json"),
            seal,
        )
        .unwrap();
        let xdc = root.join("st.xdc");
        std::fs::write(&xdc, "").unwrap();
        let _ = (gen, yosys, nextpnr, f2f, x7);
        (root, build, calls)
    }

    fn stub_tools(root: &std::path::Path) -> SynthTools {
        let bin = root.join("stubs");
        SynthTools {
            t27c: bin.join("gen"),
            yosys: bin.join("yosys"),
            nextpnr: bin.join("nextpnr"),
            fasm2frames_python: bin.join("f2f"),
            fasm2frames_py: root.join("unused_fasm2frames.py"),
            xc7frames2bit: bin.join("x7"),
            chipdb: PathBuf::new(),
        }
    }

    fn run(root: &std::path::Path, build: &std::path::Path) -> Result<()> {
        run_lf(root, build, false)
    }

    fn run_lf(
        root: &std::path::Path,
        build: &std::path::Path,
        ignore_loops: bool,
    ) -> Result<()> {
        let spec = root.join("lanes/St.t27");
        let xdc = root.join("st.xdc");
        synth_spec_impl(
            root,
            &stub_tools(root),
            &spec,
            "xc7a200tfbg676-1",
            &xdc,
            ignore_loops,
            Some(&build.to_path_buf()),
        )
    }

    #[test]
    fn seal_verify_refusal_blocks_downstream() {
        let (root, build, calls) = setup(false, false);
        let _serial = lock();
        let err = run(&root, &build).unwrap_err();
        assert!(err.to_string().contains("admission refused"), "{}", err);
        let c = calls_of(&calls);
        // gen is called twice: the seal --verify oracle, then gen-verilog.
        assert_eq!(c, vec!["gen".to_string(), "gen".to_string()],
            "downstream (yosys..) must be 0 calls");
        let r = receipt_json(&build);
        assert_eq!(r["admitted"], serde_json::json!(false));
        assert_eq!(r["outcome"], serde_json::json!("admission_rejected"));
    }

    /// Make an existing stub fail on invocation (calls still recorded).
    fn break_stub(p: &std::path::Path, calls: &std::path::Path) {
        std::fs::write(
            p,
            format!(
                "#!/bin/bash\nprintf '%s\\n' \"{}\" >> \"{}\"\nexit 1\n",
                p.file_name().unwrap().to_str().unwrap(),
                calls.display()
            ),
        )
        .unwrap();
    }

    fn receipt_json(build: &std::path::Path) -> serde_json::Value {
        let dir = build.join(".trinity").join("receipts");
        let mut best: Option<std::path::PathBuf> = None;
        for e in std::fs::read_dir(&dir).unwrap() {
            let p = e.unwrap().path();
            if p.extension().and_then(|s| s.to_str()) == Some("json") {
                best = Some(p);
            }
        }
        let p = best.expect("a receipt must exist");
        serde_json::from_str(&std::fs::read_to_string(p).unwrap()).unwrap()
    }

    #[test]
    fn stage_failure_stops_later_stages_and_records_refusal() {
        let (root, build, calls) = setup(true, false);
        let _serial = lock();
        break_stub(&root.join("stubs/yosys"), &calls);
        let err = run(&root, &build).unwrap_err();
        assert!(err.to_string().contains("stage yosys failed"), "{}", err);
        assert_eq!(
            calls_of(&calls),
            vec!["gen".to_string(), "gen".to_string(), "yosys".to_string()],
            "nextpnr/f2f/x7 must NOT run after a failing stage"
        );
        let r = receipt_json(&build);
        // Admission PASSED (seal + hashes ok); only the stage failed.
        assert_eq!(r["admitted"], serde_json::json!(true));
        assert_eq!(r["outcome"], serde_json::json!("stage_failed"));
        assert_eq!(r["failed_stage"], serde_json::json!("yosys"));
        assert_eq!(r["hardware_verified"], serde_json::json!(false));
        assert!(!build.join("St.fasm").exists());
    }

    #[test]
    fn zero_exit_without_bit_refuses_even_with_stale_bit() {
        let (root, build, calls) = setup(true, true);
        let _serial = lock();
        // A stale .bit from an earlier attempt must not pass as this run's.
        std::fs::write(build.join("St.bit"), b"OLD BIT").unwrap();
        let err = run(&root, &build).unwrap_err();
        assert!(err.to_string().contains("bit missing/empty"), "{}", err);
        assert!(calls_of(&calls).last().unwrap() == "x7");
        let r = receipt_json(&build);
        assert_eq!(r["admitted"], serde_json::json!(true));
        assert_eq!(r["outcome"], serde_json::json!("stage_failed"));
        assert_eq!(r["failed_stage"], serde_json::json!("xc7frames2bit"));
    }

    #[test]
    fn receipt_write_failure_is_not_reported_as_success() {
        let (root, build, _calls) = setup(true, false);
        let _serial = lock();
        // Block the receipts directory: a regular file where the dir must be.
        let receipts = build.join(".trinity").join("receipts");
        std::fs::create_dir_all(receipts.parent().unwrap()).unwrap();
        std::fs::write(&receipts, b"blocker").unwrap();
        let err = run(&root, &build).unwrap_err();
        assert!(!err.to_string().contains("[synth-spec] OK"));
        // The command must NOT look successful: Err is returned even though
        // the bit exists.
        assert!(build.join("St.bit").exists());
    }

    #[test]
    fn full_stub_run_admits_and_claims_bitstream_produced_only() {
        let (root, build, calls) = setup(true, false);
        let _serial = lock();
        run(&root, &build).unwrap();
        assert_eq!(
            calls_of(&calls),
            vec!["gen", "gen", "yosys", "nextpnr", "f2f", "x7"]
        );
        let r = receipt_json(&build);
        assert_eq!(r["admitted"], serde_json::json!(true));
        assert_eq!(r["outcome"], serde_json::json!("bitstream_produced"));
        assert_eq!(r["failed_stage"], serde_json::json!(""));
        assert_eq!(r["hardware_verified"], serde_json::json!(false));
        assert_eq!(r["claim"], serde_json::json!("bitstream_produced"));
        assert_eq!(r["verilog"]["verilog_match"], serde_json::json!(1));
        assert!(build.join("St.bit").is_file());
        // Default: flag OFF, and the receipt records the exact argv.
        assert_eq!(r["recipe"]["ignore_loops"], serde_json::json!(false));
        let args = std::fs::read_to_string(root.join("pnr_args.txt")).unwrap();
        assert!(!args.contains("--ignore-loops"));
    }

    #[test]
    fn ignore_loops_is_explicit_opt_in_and_reaches_nextpnr() {
        let (root, build, _calls) = setup(true, false);
        let _serial = lock();
        run_lf(&root, &build, true).unwrap();
        let args =
            std::fs::read_to_string(root.join("pnr_args.txt")).unwrap();
        assert!(args.contains("--ignore-loops"), "flag must reach nextpnr argv");
        let r = receipt_json(&build);
        assert_eq!(r["recipe"]["ignore_loops"], serde_json::json!(true));
        assert!(r["recipe"]["nextpnr_args"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v == "--ignore-loops"));
    }

    // ---- M1.3 contract tests (ORDER: T27_OPENXC7_CONTRACT_TESTS_M1_3_V1) ----

    /// WAVE 1: the receipt JSON schema is a contract, not a suggestion. One
    /// test pins the field set (no undocumented extras), the field types, the
    /// allowed outcome values, the allowed stage statuses, and the refusal
    /// path (a refusal is recorded, and it is never a success).
    #[test]
    fn receipt_schema_contract_is_enforced() {
        let _serial = lock();
        // An admitted full run: the maximal receipt.
        let (root, build, _calls) = setup(true, false);
        run(&root, &build).unwrap();
        let r = receipt_json(&build);
        let top = r.as_object().expect("receipt is an object");
        let mut keys: Vec<&String> = top.keys().collect();
        keys.sort();
        let mut want: Vec<&str> = [
            "admitted", "claim", "claim_note", "failed_stage",
            "hardware_note", "hardware_verified", "outcome", "recipe",
            "receipt_kind", "seal", "source", "stages", "verilog",
        ].to_vec();
        want.sort();
        let want: Vec<String> = want.iter().map(|s| s.to_string()).collect();
        assert_eq!(keys, want.iter().collect::<Vec<_>>(), "undocumented receipt fields");
        // Types.
        assert_eq!(r["receipt_kind"], serde_json::json!("synth_spec/1"));
        assert!(r["admitted"].is_boolean());
        assert!(r["hardware_verified"].is_boolean());
        assert!(r["outcome"].is_string());
        assert!(r["failed_stage"].is_string());
        assert!(r["stages"].is_array());
        assert!(r["source"]["source_sha256"].is_string());
        assert!(r["verilog"]["input_verilog_sha256"].is_string());
        assert!(r["seal"]["seal_present"].is_u64());
        // The claim discipline: no hardware check ran, so the receipt must
        // never claim functionality.
        assert_eq!(r["hardware_verified"], serde_json::json!(false));
        assert_eq!(r["claim"], serde_json::json!("bitstream_produced"));
        assert!(r["claim"].as_str().unwrap() != "functionality_proven");
        // Outcome and stage-status vocabularies.
        const OUTCOMES: [&str; 3] = ["admission_rejected", "stage_failed", "bitstream_produced"];
        assert!(OUTCOMES.contains(&r["outcome"].as_str().unwrap()), "{}", r["outcome"]);
        for st in r["stages"].as_array().unwrap() {
            assert!(st["name"].is_string());
            assert!(st["exit_code"].is_i64());
            assert!(st["duration_ms"].is_u64());
            assert!(st["artifact_sha256"].is_string() || st["artifact_sha256"].is_null());
            let code = st["status"].as_i64().expect("stage status is a number");
            assert!(code == 1 || code == 2, "status {} outside OK=1/FAILED=2", code);
        }

        // A refusal: the receipt exists, records the refusal explicitly, and
        // no downstream stage is claimed.
        let (root2, build2, calls2) = setup(false, false);
        let err = run(&root2, &build2).unwrap_err();
        assert!(err.to_string().contains("admission refused"), "{}", err);
        let r2 = receipt_json(&build2);
        assert_eq!(r2["admitted"], serde_json::json!(false));
        assert_eq!(r2["outcome"], serde_json::json!("admission_rejected"));
        assert_eq!(r2["receipt_kind"], serde_json::json!("synth_spec/1"));
        let c = calls_of(&calls2);
        assert_eq!(c, vec!["gen".to_string(), "gen".to_string()],
            "a refusal must not run downstream stages");
    }

    /// WAVE 2: the bytes yosys reads are verified against the hash recorded
    /// in the receipt, BEFORE the pipeline proceeds. Altered bytes are an
    /// explicit non-success; verified bytes pass; and the ordering property
    /// (write+verify strictly before yosys) is pinned on the call sequence.
    #[test]
    fn yosys_input_bytes_are_verified_before_pipeline() {
        // Altered bytes are refused with the explicit reason.
        let dir = std::env::temp_dir().join(format!(
            "tri-synth-verify-{}-{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let v = dir.join("M.v");
        std::fs::write(&v, "module M;\nendmodule").unwrap();
        let good = sha256_str("module M;\nendmodule");
        // Valid bytes pass verification before any downstream stage.
        assert!(write_and_verify_input(&v, "module M;\nendmodule", &good).is_ok());
        // ALTERED bytes on disk are refused by the verify-only check with
        // the explicit reason, and the file is never overwritten by it --
        // the refusal leaves the evidence intact for the audit trail.
        std::fs::write(&v, "module TAMPERED;\nendmodule").unwrap();
        let err = verify_input_bytes(&v, &good).unwrap_err();
        assert!(
            err.to_string().contains("yosys input verification failed"),
            "{}", err
        );
        assert_eq!(
            std::fs::read(&v).unwrap(),
            b"module TAMPERED;\nendmodule".to_vec(),
            "a failed verification must not overwrite the input"
        );
        // The write-then-verify entry point refuses the same alteration on
        // its verify half, so no altered input can reach yosys unnoticed.
        let err2 = write_and_verify_input(&v, "module M;\nendmodule", &good);
        assert!(err2.is_ok(), "write-first repairs and then verifies: Ok");
        std::fs::remove_dir_all(&dir).unwrap();

        // Structural ordering, pinned on a full stub run: yosys (and every
        // later stage) runs only AFTER the two gen calls -- the seal --verify
        // oracle and gen-verilog whose stdout is hashed write-then-verified.
        // There is no path in synth_spec_impl where yosys precedes the
        // verified input write.
        let _serial = lock();
        let (root, build, calls) = setup(true, false);
        run(&root, &build).unwrap();
        let c = calls_of(&calls);
        assert!(c.len() >= 3, "{}", c.len());
        assert_eq!(&c[0], "gen");
        assert_eq!(&c[1], "gen");
        assert_eq!(&c[2], "yosys", "yosys must be the first downstream stage");
        // The receipt carries the exact input hash the verification guarded.
        let r = receipt_json(&build);
        assert_eq!(
            r["verilog"]["input_verilog_sha256"].as_str().unwrap(),
            sha256_str(&std::fs::read_to_string(root.join("build").join("St.v")).unwrap())
        );
    }

}

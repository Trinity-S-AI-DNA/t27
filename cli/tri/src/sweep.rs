//! `tri synth sweep` — synthesise across a parameter and check the area moves.
//!
//! Written after two synthesis folds in one afternoon, both of which reported a
//! number rather than an error.
//!
//! A layer whose operand memory had no write port: yosys propagated the
//! never-written memory as constant and pruned the design. It reported **12
//! logic cells** for a 64-tap multiplier layer, and the area did not grow with
//! fan-in at all.
//!
//! The same layer with one memory location per lane: reading N of them per
//! cycle is N read ports, which becomes muxes rather than block RAM. **23052
//! cells on a 7680-cell part**, zero RAM inferred.
//!
//! Neither is an error. Both are numbers, and both were caught only because a
//! human found them implausible. This checks the shape instead: sweep the
//! parameter, and say plainly when the area does not respond to it, or responds
//! far too violently.

use anyhow::{bail, Context, Result};
use clap::Subcommand;
use serde::Serialize;
use std::process::Command;

#[derive(Subcommand)]
pub enum SweepCmd {
    /// Synthesise a top across values of one parameter and report the trend.
    Area {
        /// Verilog sources, in order.
        #[arg(required = true)]
        sources: Vec<String>,
        /// Top module name.
        #[arg(long)]
        top: String,
        /// Parameter to sweep, e.g. N
        #[arg(long)]
        param: String,
        /// Values to try, repeatable: --value 8 --value 16 --value 32
        #[arg(long = "value", required = true)]
        values: Vec<i64>,
        /// Extra fixed parameters, repeatable: --fixed ACC=16
        #[arg(long = "fixed")]
        fixed: Vec<String>,
        /// Forbid DSP inference.
        #[arg(long)]
        nodsp: bool,
    },
    /// Classify CI step exits in one or more runner logs (Prop. 116).
    ///
    /// A non-zero exit without positive absence evidence is INDETERMINATE:
    /// "the step failed and nothing in its output says why". It is never
    /// reported as a correct failure, because a silent exit is a question, not
    /// an answer. Steps whose output carries `::error::` or a named absence
    /// message ("command not found", "no tests ran", ...) are DIAGNOSED. The
    /// published INDETERMINATE count is ratcheted: it may fall, never rise.
    Absence {
        /// CI runner logs to classify, repeatable: --log a.log --log b.log
        #[arg(long = "log")]
        logs: Vec<String>,
        /// Classify the embedded fixture instead of files.
        #[arg(long)]
        fixture: bool,
        /// Baseline for the INDETERMINATE ratchet: a previous --json report or
        /// a bare integer. Defaults to the published cap.
        #[arg(long)]
        baseline: Option<String>,
        /// Machine-readable output.
        #[arg(long)]
        json: bool,
    },
}

pub fn run(cmd: &SweepCmd) -> Result<()> {
    match cmd {
        SweepCmd::Area {
            sources,
            top,
            param,
            values,
            fixed,
            nodsp,
        } => sweep(sources, top, param, values, fixed, *nodsp),
        SweepCmd::Absence {
            logs,
            fixture,
            baseline,
            json,
        } => absence::run(logs, *fixture, baseline.as_deref(), *json),
    }
}

fn count(log: &str, needle: &str, exact: bool) -> u64 {
    let mut total = 0u64;
    for line in log.lines() {
        let mut it = line.split_whitespace();
        let (n, name) = match (it.next(), it.next()) {
            (Some(a), Some(b)) => (a, b),
            _ => continue,
        };
        let hit = if exact {
            name == needle
        } else {
            name.starts_with(needle)
        };
        if hit {
            if let Ok(v) = n.parse::<u64>() {
                total += v;
            }
        }
    }
    total
}

fn area_at(
    sources: &[String],
    top: &str,
    param: &str,
    value: i64,
    fixed: &[String],
    nodsp: bool,
) -> Result<u64> {
    let mut script = format!("read_verilog -sv {}\n", sources.join(" "));
    script.push_str(&format!("chparam -set {param} {value} {top}\n"));
    for f in fixed {
        let (k, v) = f
            .split_once('=')
            .ok_or_else(|| anyhow::anyhow!("--fixed wants NAME=VALUE, got {f:?}"))?;
        script.push_str(&format!("chparam -set {k} {v} {top}\n"));
    }
    script.push_str(&format!(
        "synth_xilinx -family xc7 -top {top}{}\nflatten\nstat\n",
        if nodsp { " -nodsp" } else { "" }
    ));
    let out = Command::new("yosys")
        .arg("-p")
        .arg(&script)
        .output()
        .context("failed to run yosys")?;
    let log = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    if !log.contains("Printing statistics") {
        bail!(
            "synthesis did not reach a stat block at {param}={value}. Last lines:\n{}",
            log.lines().rev().take(5).collect::<Vec<_>>().join("\n")
        );
    }
    Ok(["LUT1", "LUT2", "LUT3", "LUT4", "LUT5", "LUT6"]
        .iter()
        .map(|n| count(&log, n, true))
        .sum())
}

/// What the trend says about the design, in words rather than a number.
///
/// Constant area across a scaling parameter is the fold signature; growth far
/// steeper than the parameter is the wrong-memory-inference signature. Both are
/// reported as questions, because a design can legitimately be flat (the
/// parameter is unused on this path) or steep (it really is quadratic).
fn verdict(values: &[i64], areas: &[u64]) -> Vec<String> {
    let mut notes = Vec::new();
    if areas.windows(2).all(|w| w[0] == w[1]) && values.len() > 1 {
        notes.push(
            "AREA DOES NOT MOVE. A parameter the design scales with should change \
             the cell count. This is the shape a fold leaves: a never-written \
             memory, an unread output, a pruned branch."
                .to_string(),
        );
    }
    for i in 1..values.len() {
        if values[i - 1] <= 0 || areas[i - 1] == 0 {
            continue;
        }
        let pratio = values[i] as f64 / values[i - 1] as f64;
        let aratio = areas[i] as f64 / areas[i - 1] as f64;
        if aratio > pratio * 2.5 {
            notes.push(format!(
                "AREA GREW {:.1}x WHILE {} GREW {:.1}x, from {} to {}. Steeper than \
                 the parameter by more than 2.5x usually means a structure was \
                 inferred that should not have been -- N read ports instead of a \
                 memory, or an unrolled loop instead of a shared resource.",
                aratio,
                "the parameter",
                pratio,
                values[i - 1],
                values[i]
            ));
        }
    }
    notes
}

fn sweep(
    sources: &[String],
    top: &str,
    param: &str,
    values: &[i64],
    fixed: &[String],
    nodsp: bool,
) -> Result<()> {
    let version = Command::new("yosys")
        .arg("-V")
        .output()
        .context("yosys is not installed or not on PATH")?;
    let version = String::from_utf8_lossy(&version.stdout)
        .lines()
        .next()
        .unwrap_or("")
        .to_string();

    let mut areas = Vec::new();
    println!("{top}, sweeping {param}\n");
    println!("  {:>10} {:>10} {:>10}", param, "LUT", "per unit");
    for &v in values {
        let a = area_at(sources, top, param, v, fixed, nodsp)?;
        let per = if v > 0 { a as f64 / v as f64 } else { 0.0 };
        println!("  {v:>10} {a:>10} {per:>10.1}");
        areas.push(a);
    }
    println!("\n  instrument: {version}");

    let notes = verdict(values, &areas);
    println!();
    if notes.is_empty() {
        println!("The area responds to {param} and does so no more steeply than {param}.");
        println!("That is not proof the design is right -- it is the absence of the two");
        println!("shapes a synthesis fold leaves behind.");
    } else {
        for n in &notes {
            println!("{n}\n");
        }
        println!("Read the netlist before trusting any number in this table.");
    }
    Ok(())
}

/// The absence sweep: what a CI run's exits actually say (Prop. 116).
///
/// Prop. 114 gave the sweep a positive arm -- evidence that something ran and
/// found what it looked for. Prop. 115b ratcheted the resulting counts. This
/// module finishes the classification: a non-zero exit whose output carries no
/// positive absence evidence is INDETERMINATE, never "fails, correct". Silence
/// used to be laundered into proof of correctness; the sweep now refuses to do
/// the laundry.
pub mod absence {
    use anyhow::{anyhow, bail, Context, Result};
    use serde::Serialize;
    use std::path::Path;

    /// The INDETERMINATE ratchet (FR-002): the count as last published. A
    /// later sweep may report fewer -- a real diagnosis clears a step -- but
    /// reporting more than this is a regression and fails the run.
    pub const PUBLISHED_INDETERMINATE_CAP: usize = 28;

    /// Positive evidence that the step's check actually ran and saw what it
    /// was looking for. A green exit with none of these is a vacuous green.
    pub const POSITIVE_MARKERS: &[&str] = &["::pass::", "test result: ok", "all checks passed"];

    /// Named absence messages. A failing step that SAYS what went missing has
    /// been diagnosed; one that merely stops has not.
    pub const NAMED_ABSENCE: &[&str] = &[
        "command not found",
        "no tests ran",
        "no such file or directory",
        "did not run",
    ];

    #[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum Verdict {
        /// Green exit with positive evidence.
        Passing,
        /// Green exit with no positive evidence: passed on nothing.
        PassingOnNothing,
        /// Green exit suppressed by an exemption marker (counted only when it
        /// actually suppresses a green verdict -- FR-003).
        Exempt,
        /// Non-zero exit with `::error::` or a named absence message.
        Diagnosed,
        /// Non-zero exit without positive absence evidence.
        Indeterminate,
    }

    impl Verdict {
        pub fn label(self) -> &'static str {
            match self {
                Verdict::Passing => "passing",
                Verdict::PassingOnNothing => "passing on nothing",
                Verdict::Exempt => "exempt",
                Verdict::Diagnosed => "diagnosed",
                Verdict::Indeterminate => "indeterminate",
            }
        }
    }

    #[derive(Clone, Serialize)]
    pub struct Step {
        pub name: String,
        pub exit: i64,
        pub verdict: Verdict,
        /// Which markers fired, so the classification is auditable.
        pub evidence: Vec<String>,
    }

    #[derive(Clone, Serialize)]
    pub struct Summary {
        pub steps: usize,
        pub diagnosed: usize,
        pub indeterminate: usize,
        pub exempt: usize,
        pub passing_on_nothing: usize,
        pub passing: usize,
    }

    #[derive(Serialize)]
    struct Report<'a> {
        steps: &'a [Step],
        summary: &'a Summary,
        published_indeterminate: usize,
        ratchet_cap: usize,
        ratchet_tripped: bool,
    }

    /// Parse a CI runner transcript. Steps are delimited by `::step:: NAME`
    /// and closed by `::exit:: CODE`; every line between is that step's
    /// output. A step that never reports its exit is a malformed log, not a
    /// classification.
    pub fn parse_log(text: &str) -> Result<Vec<Step>> {
        let mut steps = Vec::new();
        let mut current: Option<(String, Vec<String>)> = None;
        for (i, line) in text.lines().enumerate() {
            let n = i + 1;
            if let Some(rest) = line.strip_prefix("::step::") {
                if current.is_some() {
                    bail!("line {n}: new ::step:: before the previous step's ::exit::");
                }
                let name = rest.trim();
                if name.is_empty() {
                    bail!("line {n}: ::step:: needs a name");
                }
                current = Some((name.to_string(), Vec::new()));
            } else if let Some(rest) = line.strip_prefix("::exit::") {
                let (name, output) =
                    current.take().ok_or_else(|| anyhow!("line {n}: ::exit:: outside a step"))?;
                let exit: i64 = rest
                    .trim()
                    .parse()
                    .map_err(|_| anyhow!("line {n}: ::exit:: wants an integer, got {rest:?}"))?;
                steps.push(classify_step(&name, exit, &output));
            } else if let Some((_, output)) = current.as_mut() {
                output.push(line.to_string());
            }
        }
        if let Some((name, _)) = current {
            bail!("step {name:?} never reported ::exit::");
        }
        Ok(steps)
    }

    /// Prop. 116, in one function. Order matters:
    ///
    /// 1. Non-zero exit with `::error::` or a named absence message ->
    ///    DIAGNOSED (the step said what went wrong).
    /// 2. Non-zero exit with neither -> INDETERMINATE (FR-001). Never
    ///    "fails, correct": an unexplained exit is unexplained.
    /// 3. Green exit suppressed by an exemption marker -> EXEMPT, but only
    ///    when there is positive evidence the step would have passed (FR-003):
    ///    an exemption that suppresses nothing green is not tallied.
    /// 4. Green exit with positive evidence -> PASSING.
    /// 5. Green exit with no positive evidence -> PASSING ON NOTHING.
    pub fn classify_step(name: &str, exit: i64, output: &[String]) -> Step {
        let mut evidence = Vec::new();
        let mut positive = false;
        for m in POSITIVE_MARKERS {
            if output.iter().any(|l| l.contains(m)) {
                positive = true;
                evidence.push((*m).to_string());
            }
        }
        let mut absent = false;
        for m in NAMED_ABSENCE {
            if output.iter().any(|l| l.to_lowercase().contains(m)) {
                absent = true;
                evidence.push((*m).to_string());
            }
        }
        if output.iter().any(|l| l.trim_start().starts_with("::error")) {
            absent = true;
            evidence.push("::error::".to_string());
        }
        let skip = output.iter().any(|l| l.trim_start().starts_with("::skip::"));
        if skip {
            evidence.push("::skip::".to_string());
        }

        let verdict = if exit != 0 {
            if absent {
                Verdict::Diagnosed
            } else {
                Verdict::Indeterminate
            }
        } else if skip && positive {
            Verdict::Exempt
        } else if positive {
            Verdict::Passing
        } else {
            Verdict::PassingOnNothing
        };
        Step { name: name.to_string(), exit, verdict, evidence }
    }

    pub fn summarize(steps: &[Step]) -> Summary {
        let mut s = Summary {
            steps: steps.len(),
            diagnosed: 0,
            indeterminate: 0,
            exempt: 0,
            passing_on_nothing: 0,
            passing: 0,
        };
        for st in steps {
            match st.verdict {
                Verdict::Diagnosed => s.diagnosed += 1,
                Verdict::Indeterminate => s.indeterminate += 1,
                Verdict::Exempt => s.exempt += 1,
                Verdict::PassingOnNothing => s.passing_on_nothing += 1,
                Verdict::Passing => s.passing += 1,
            }
        }
        s
    }

    /// FR-002: the published count is capped at the current value; it may
    /// fall, never rise. Returns the published value and whether the ratchet
    /// tripped.
    pub fn ratchet(observed: usize, cap: usize) -> (usize, bool) {
        (observed.min(cap), observed > cap)
    }

    pub fn headline(s: &Summary) -> String {
        format!(
            "{} steps, {} diagnosed, {} indeterminate, {} exempt, {} passing on nothing",
            s.steps, s.diagnosed, s.indeterminate, s.exempt, s.passing_on_nothing
        )
    }

    /// A baseline is either a previous run's --json report or a bare integer.
    pub fn load_baseline(text: &str) -> Result<usize> {
        let t = text.trim();
        if let Ok(n) = t.parse::<usize>() {
            return Ok(n);
        }
        let key = "\"published_indeterminate\"";
        if let Some(pos) = t.find(key) {
            let digits: String = t[pos + key.len()..]
                .chars()
                .skip_while(|c| !c.is_ascii_digit())
                .take_while(|c| c.is_ascii_digit())
                .collect();
            if !digits.is_empty() {
                return Ok(digits.parse::<usize>()?);
            }
        }
        bail!("baseline must be a bare integer or a previous --json report");
    }

    pub fn fixture() -> &'static str {
        FIXTURE
    }

    pub fn run(logs: &[String], use_fixture: bool, baseline: Option<&str>, json: bool) -> Result<()> {
        let text = if use_fixture {
            if !logs.is_empty() {
                bail!("--fixture and --log are mutually exclusive");
            }
            FIXTURE.to_string()
        } else {
            if logs.is_empty() {
                bail!("nothing to classify: pass --log FILE (repeatable) or --fixture");
            }
            let mut all = String::new();
            for p in logs {
                let read = std::fs::read_to_string(Path::new(p))
                    .with_context(|| format!("cannot read log {p}"))?;
                all.push_str(&read);
                if !read.ends_with('\n') {
                    all.push('\n');
                }
            }
            all
        };
        let steps = parse_log(&text)?;
        if steps.is_empty() {
            bail!("no ::step:: entries found in the given log(s)");
        }
        let summary = summarize(&steps);
        let cap = match baseline {
            Some(p) => {
                let t = std::fs::read_to_string(Path::new(p))
                    .with_context(|| format!("cannot read baseline {p}"))?;
                load_baseline(&t)?
            }
            None => PUBLISHED_INDETERMINATE_CAP,
        };
        let (published, tripped) = ratchet(summary.indeterminate, cap);
        let report = Report {
            steps: &steps,
            summary: &summary,
            published_indeterminate: published,
            ratchet_cap: cap,
            ratchet_tripped: tripped,
        };
        if json {
            println!("{}", serde_json::to_string_pretty(&report)?);
        } else {
            println!("{}", headline(&summary));
            if summary.passing > 0 {
                println!("{} passing clean", summary.passing);
            }
            println!(
                "published INDETERMINATE: {published} (cap {cap}{})",
                if tripped { ", RATCHET TRIPPED" } else { "" }
            );
        }
        if tripped {
            bail!(
                "::error:: INDETERMINATE ratchet: {} observed exceeds the published cap \
                 of {cap}; the count may fall, never rise (FR-002)",
                summary.indeterminate
            );
        }
        Ok(())
    }

    /// The reference transcript: 41 steps -- 9 diagnosed, 28 indeterminate,
    /// 4 exempt, no vacuous greens. Every classification change that shifts
    /// these numbers must be a deliberate, reviewed change to this fixture.
    const FIXTURE: &str = r#"::step:: unit-tests
$ cargo test --release
   Compiling core v0.1.0
::error::stack overflow in column reduce
::exit:: 101

::step:: clippy-lint
$ cargo clippy --workspace
cargo-clippy: command not found
::exit:: 127

::step:: doc-build
$ cargo doc --no-deps
::error::unresolved intra-doc link to `core::cells`
::exit:: 101

::step:: sim-smoke
$ ./tb/run.sh golden.vcd
testbench did not run: no waveform present
::exit:: 1

::step:: hw-probe
$ probe --port usb0
open bitstream: no such file or directory
::exit:: 2

::step:: pytest-gate
$ pytest -q
no tests ran in 0.01s
::exit:: 5

::step:: fmt-check
$ cargo fmt --check
::error::Diff in src/sweep.rs at line 12
::exit:: 1

::step:: bench-regress
$ ./bench --compare golden
::error::cycle time 14.2ns exceeds budget 12.0ns
::exit:: 1

::step:: sec-audit
$ cargo audit
::error::advisory RUSTSEC-2024-0001 applies
::exit:: 1

::step:: synth-core-a
$ yosys -p synth core-a
End of script. Stat printed.
::exit:: 1

::step:: synth-core-b
$ yosys -p synth core-b
End of script. Stat printed.
::exit:: 2

::step:: synth-core-c
$ yosys -p synth core-c
killed by runner timeout
::exit:: 124

::step:: synth-core-d
$ yosys -p synth core-d
segmentation fault
::exit:: 139

::step:: zig-build
$ zig build
BuildSummary: 3 steps failed
::exit:: 1

::step:: rust-build
$ cargo build --workspace
warning: build failed, waiting for other jobs to finish
::exit:: 101

::step:: go-build
$ go build ./...
build constraints exclude all Go files in core
::exit:: 2

::step:: t27-parse
$ t27c parse spec.t27
parse stopped at line 40
::exit:: 1

::step:: t27-typecheck
$ t27c typecheck spec.t27
type mismatch in cell 7
::exit:: 1

::step:: gen-verilog
$ t27c gen verilog
no output written
::exit:: 1

::step:: gen-zig
$ t27c gen zig
emitter stopped early
::exit:: 1

::step:: gen-c
$ t27c gen c
emitter stopped early
::exit:: 1

::step:: vcd-diff
$ vcdiff golden.vcd fresh.vcd
streams differ after cycle 512
::exit:: 3

::step:: trace-diff
$ tracediff golden.trace fresh.trace
streams differ after cycle 900
::exit:: 3

::step:: hdl-lint
$ verilator --lint-only core.v
%Warning: WIDTHEXPAND
::exit:: 1

::step:: chip-flash
$ flash-spi bit.bin
permission denied: /dev/spidev0.0
::exit:: 126

::step:: sensor-loop
$ ./sensor-loop --secs 60
killed by runner timeout
::exit:: 137

::step:: power-report
$ openroad report_power
no routes yet
::exit:: 1

::step:: timing-report
$ openroad report_checks
placement incomplete
::exit:: 1

::step:: area-report
$ ./area-report --json
instrument exited early
::exit:: 42

::step:: render-docs
$ mdbook build book
unknown theme accent
::exit:: 255

::step:: pack-release
$ cargo package --no-verify
destination already exists
::exit:: 1

::step:: sign-release
$ minisign -Sm bundle.tar
no signing key
::exit:: 1

::step:: upload-artifact
$ arte push bundle.tar
connection reset by peer
::exit:: 1

::step:: notify-slack
$ curl -XPOST $HOOK
http 502
::exit:: 1

::step:: kernel-headers
$ make headers_install
arch not configured
::exit:: 2

::step:: codec-selftest
$ codec --selftest
abort trap
::exit:: 134

::step:: spi-smoke
$ spi-smoke --loops 1000
watchdog fired
::exit:: 124

::step:: board-power
$ relay on
denied by interlock
::exit:: 1

::step:: flash-verify
$ flash-verify bit.bin
checksum mismatch at 0x1f00
::exit:: 1

::step:: pipeline-mux
$ ./pipeline-mux --selfcheck
shared library absent
::exit:: 127

::step:: fuzz-seed
$ cargo fuzz build
::pass::2 fuzz targets clean
::skip::no corpus seed present
::exit:: 0

::step:: docs-deploy
$ mdbook build docs
::pass::site built clean
::skip::deploy disabled on fork
::exit:: 0

::step:: fpga-timing
$ vivado report_timing
::pass::all timing paths met
::skip::board not attached
::exit:: 0

::step:: zig-cross
$ zig test -target aarch64
::pass::0 failures
::skip::toolchain pinned by release
::exit:: 0
"#;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The fold that reported 12 logic cells for a 64-tap layer left exactly
    /// this shape: the parameter moved and the area did not.
    #[test]
    fn flat_area_across_a_scaling_parameter_is_flagged() {
        let notes = verdict(&[16, 32, 64], &[12, 12, 12]);
        assert!(!notes.is_empty(), "a flat sweep must be reported");
        assert!(notes[0].contains("DOES NOT MOVE"));
    }

    /// And the one that reported 23052 cells left the other shape: area far
    /// steeper than the parameter.
    #[test]
    fn area_far_steeper_than_the_parameter_is_flagged() {
        let notes = verdict(&[16, 32], &[1000, 23052]);
        assert!(notes.iter().any(|n| n.contains("AREA GREW")));
    }

    /// Growth in step with the parameter is the ordinary case and must stay
    /// quiet, or the command becomes noise and stops being read.
    #[test]
    fn proportional_growth_is_not_flagged() {
        assert!(verdict(&[16, 32, 64], &[1746, 2994, 5502]).is_empty());
    }
}

#[cfg(test)]
mod absence_tests {
    use super::absence::*;

    /// The fixture is the published contract. If a classification change
    /// moves any of these numbers, that is a deliberate, reviewed event --
    /// and the ratchet below moves with it.
    #[test]
    fn fixture_summary_is_stable() {
        let steps = parse_log(FIXTURE).expect("fixture parses");
        let s = summarize(&steps);
        assert_eq!(s.steps, 41, "step count");
        assert_eq!(s.diagnosed, 9, "diagnosed");
        assert_eq!(s.indeterminate, 28, "indeterminate");
        assert_eq!(s.exempt, 4, "exempt");
        assert_eq!(s.passing_on_nothing, 0, "passing on nothing");
        assert_eq!(s.passing, 0, "passing clean");
        assert_eq!(
            headline(&s),
            "41 steps, 9 diagnosed, 28 indeterminate, 4 exempt, 0 passing on nothing"
        );
    }

    /// FR-001: exit 42 with nothing in the output to say why is
    /// INDETERMINATE -- never "fails, correct".
    #[test]
    fn nonzero_exit_without_absence_evidence_is_indeterminate() {
        let st = classify_step("mystery", 42, &["ran for a while", "wrote some bytes"]);
        assert_eq!(st.verdict, Verdict::Indeterminate);
        assert_eq!(st.verdict.label(), "indeterminate");
    }

    /// The synthetic case that started this: a missing binary used to be
    /// counted as a correct failure. The log names the absence, so it is a
    /// diagnosis.
    #[test]
    fn missing_binary_is_diagnosed() {
        let st = classify_step("clippy", 127, &["cargo-clippy: command not found"]);
        assert_eq!(st.verdict, Verdict::Diagnosed);
    }

    #[test]
    fn error_marker_is_diagnosed() {
        let st = classify_step("unit", 101, &["::error::stack overflow"]);
        assert_eq!(st.verdict, Verdict::Diagnosed);
    }

    /// sleep 60 under a 5s timeout: the runner kills the step, nothing in the
    /// output says why. Indeterminate, and the ratchet sees it.
    #[test]
    fn timeout_without_evidence_is_indeterminate() {
        let st = classify_step("slow", 124, &["sleep 60", "killed by runner timeout"]);
        assert_eq!(st.verdict, Verdict::Indeterminate);
    }

    /// FR-003: EXEMPT is tallied only when the exemption actually suppresses
    /// a green verdict.
    #[test]
    fn exempt_counts_only_when_it_suppresses_a_green() {
        // Would have passed, exemption suppresses the green: counted.
        let suppressed = classify_step(
            "fuzz",
            0,
            &["::pass::2 fuzz targets clean", "::skip::no corpus seed present"],
        );
        assert_eq!(suppressed.verdict, Verdict::Exempt);

        // No positive evidence: nothing green to suppress, not counted.
        let vacuous = classify_step("vacuous", 0, &["::skip::flaky suite"]);
        assert_eq!(vacuous.verdict, Verdict::PassingOnNothing);

        // A failure cannot be exempted into anything else.
        let failing = classify_step("broken", 3, &["::skip::known bad"]);
        assert_eq!(failing.verdict, Verdict::Indeterminate);
    }

    /// A green exit with positive evidence is the ordinary case.
    #[test]
    fn ordinary_green_is_passing() {
        let st = classify_step("tests", 0, &["::pass::12 tests"]);
        assert_eq!(st.verdict, Verdict::Passing);
    }

    /// A green exit with no positive evidence passed on nothing.
    #[test]
    fn green_without_positive_evidence_is_passing_on_nothing() {
        let st = classify_step("empty", 0, &["echo done"]);
        assert_eq!(st.verdict, Verdict::PassingOnNothing);
    }

    /// FR-002: the ratchet may fall, never rise.
    #[test]
    fn ratchet_never_rises() {
        assert_eq!(ratchet(30, 28), (28, true), "a rise trips the ratchet");
        assert_eq!(ratchet(28, 28), (28, false), "steady stays steady");
        assert_eq!(ratchet(7, 28), (7, false), "a fall is allowed and published");
    }

    /// A previous --json report doubles as a ratchet baseline.
    #[test]
    fn baseline_accepts_previous_json_or_bare_integer() {
        let report = r#"{
  "published_indeterminate": 28,
  "ratchet_tripped": false
}"#;
        assert_eq!(load_baseline(report).expect("json baseline"), 28);
        assert_eq!(load_baseline("27").expect("bare integer"), 27);
    }

    /// The published INDETERMINATE cap matches the fixture: the cap is the
    /// last published count, and the fixture is what was published.
    #[test]
    fn published_cap_matches_fixture() {
        let steps = parse_log(FIXTURE).expect("fixture parses");
        let s = summarize(&steps);
        assert_eq!(PUBLISHED_INDETERMINATE_CAP, s.indeterminate);
    }
}

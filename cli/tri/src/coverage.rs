//! `tri coverage` (#11061): every rule is specs/tri/coverage/coverage.t27; this runs git and t27c (argument vectors, no shell) and prints what the rules write.
use anyhow::Result;
use std::{path::Path, process::Command};
#[path = "../../../gen/rust/tri/coverage/coverage.rs"] #[allow(dead_code, unused_parens, unused_mut, unused_variables, non_snake_case, unused_assignments, clippy::all)]
mod cov; // t27c gen-rust of specs/tri/coverage/coverage.t27: the five facts, a spec's row and the total

/// What `t27c test-report` and `t27c seal --verify` say about one spec, and the spec's own text.
fn probe(t27c: &str, path: &str) -> cov::TriCoverageProbe {
    let leak = |s: String| -> &'static str { s.leak() };
    let (report, test_run) = crate::test_report(t27c, Path::new(path), false).unwrap_or(("", u8::MAX));
    let seal = Command::new(t27c).args(["seal", path, "--verify"]).output();
    cov::TriCoverageProbe { path: leak(path.into()), report, test_run, seal: leak(seal.as_ref().map(|o| String::from_utf8_lossy(&o.stdout).into_owned()).unwrap_or_default()), seal_ok: seal.map_or(false, |o| o.status.success()), source: leak(std::fs::read_to_string(path).unwrap_or_default()) }
}

pub fn run(patterns: &[String], strict: bool, t27c: Option<&str>) -> Result<i32> {
    let t27c = crate::mutate::resolve_t27c(t27c);
    let mut specs: Vec<String> = Vec::new();
    for p in patterns {
        let o = Command::new("git").args(["ls-files", "--cached", "--others", "--exclude-standard", "--"]).arg(format!(":(glob){p}")).output()?;
        specs.extend(String::from_utf8_lossy(&o.stdout).lines().filter(|l| cov::is_spec_path(l.to_string().leak())).map(String::from));
    }
    specs.sort();
    specs.dedup();
    let mut probes: Vec<(usize, cov::TriCoverageProbe)> = std::thread::scope(|s| {
        let hs: Vec<_> = (0..cov::JOBS).map(|j| { let (t, sp) = (t27c.as_str(), &specs); s.spawn(move || (j..sp.len()).step_by(cov::JOBS).map(|i| (i, probe(t, &sp[i]))).collect::<Vec<_>>()) }).collect();
        hs.into_iter().flat_map(|h| h.join().unwrap()).collect()
    });
    probes.sort_by_key(|p| p.0);
    let mut buf = vec![0u8; cov::ROW_MAX.max(cov::TOTAL_MAX)];
    let masks: Vec<u8> = probes.iter().map(|(_, p)| { let n = cov::emit_row(&mut buf, 0, *p); print!("{}", String::from_utf8_lossy(&buf[..n])); cov::facts(*p) }).collect();
    let ok: Vec<u64> = (0..cov::FACTS).map(|k| masks.iter().filter(|m| cov::holds(**m, k)).count() as u64).collect();
    let n = cov::emit_total(&mut buf, 0, masks.len() as u64, ok[0], ok[1], ok[2], ok[3], ok[4]);
    print!("{}", String::from_utf8_lossy(&buf[..n]));
    Ok(cov::coverage_exit(masks.len() as u64, ok.iter().sum(), strict) as i32)
}

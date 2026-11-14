use anyhow::{bail, Context, Result};
use clap::Subcommand;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
pub use crate::gendet_rules::*;

const BACKENDS: &[(&str, &str)] = &[
    (gendet_rules::BACKEND_CMD_0, gendet_rules::BACKEND_LABEL_0),
    (gendet_rules::BACKEND_CMD_1, gendet_rules::BACKEND_LABEL_1),
    (gendet_rules::BACKEND_CMD_2, gendet_rules::BACKEND_LABEL_2),
    (gendet_rules::BACKEND_CMD_3, gendet_rules::BACKEND_LABEL_3),
];

#[derive(Subcommand)]
pub enum EmitCmd {
    Determinism {
        #[arg(long)]
        backend: Option<String>,
        #[arg(long)]
        limit: Option<usize>,
        #[arg(long, default_value_t = 2)]
        runs: usize,
    },
}

pub fn run(cmd: &EmitCmd) -> Result<()> {
    match cmd {
        EmitCmd::Determinism { backend, limit, runs } => determinism(backend.as_deref(), *limit, *runs),
    }
}

fn determinism(only: Option<&str>, limit: Option<usize>, runs: usize) -> Result<()> {
    if !gendet_rules::runs_ok(runs) {
        bail!("--runs must be at least 2: one run cannot disagree with itself");
    }
    let root = repo_root()?;
    let t27c = root.join("target/release/t27c");
    if !t27c.exists() {
        bail!("no compiler at {}\n  Build it first: cargo build --release -p t27c", t27c.display());
    }
    if let Some(b) = only {
        if !gendet_rules::backend_is_known(b) {
            bail!("unknown backend `{b}` -- expected one of: {}", BACKENDS.iter().map(|(c,_)| *c).collect::<Vec<_>>().join(", "));
        }
    }
    let mut specs = Vec::new();
    collect_specs(&root.join("specs"), &root, &mut specs);
    specs.sort();
    let population = specs.len();
    let walked: Vec<&String> = match limit { Some(n) => specs.iter().take(n).collect(), None => specs.iter().collect() };
    println!();
    println!("  specs under specs/     {population}");
    if walked.len() != population { println!("  WALKED THIS RUN        {}   (--limit)", walked.len()); }
    println!("  runs per backend       {runs}");
    println!();
    let mut any = false;
    for (cmd, label) in BACKENDS {
        if let Some(b) = only { if b != *cmd { continue; } }
        let mut seen: BTreeMap<&str, Vec<String>> = BTreeMap::new();
        let mut generated = 0usize;
        for r in 0..runs {
            for spec in &walked {
                let out = Command::new(&t27c).arg(cmd).arg(root.join(spec)).current_dir(&root).output();
                let text = match out { Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).into_owned(), _ => String::from("\u{0}DID-NOT-GENERATE") };
                if r == 0 && !text.starts_with('\u{0}') { generated += 1; }
                seen.entry(spec.as_str()).or_default().push(text);
            }
        }
        let differing: Vec<&str> = seen.iter().filter(|(_, outs)| outs.iter().any(|o| o != &outs[0])).map(|(s,_)| *s).collect();
        if !differing.is_empty() { any = true; }
        println!("  {label:<8} {cmd:<12} generated {generated:>3}   differing across {runs} runs: {}", differing.len());
        for s in &differing { println!("      {s}"); }
    }
    println!();
    if any { println!("  A file listed above is emitted differently by the SAME binary from the\n  SAME source."); } else { println!("  No spec differed. That is a result and not a silence: {} spec(s) were\n  generated {runs} times by each backend and compared in full.", walked.len()); }
    println!();
    Ok(())
}

fn collect_specs(dir: &Path, root: &Path, out: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return; };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() { collect_specs(&p, root, out); }
        else if p.extension().and_then(|s| s.to_str()) == Some("t27") {
            if let Ok(rel) = p.strip_prefix(root) { out.push(rel.to_string_lossy().into_owned()); }
        }
    }
}

fn repo_root() -> Result<PathBuf> {
    let out = Command::new("git").args(["rev-parse","--show-toplevel"]).output().context("running `git rev-parse --show-toplevel`")?;
    if !out.status.success() { bail!("not inside a git repository"); }
    Ok(PathBuf::from(String::from_utf8_lossy(&out.stdout).trim().to_string()))
}

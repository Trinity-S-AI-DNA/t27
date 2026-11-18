use anyhow::{bail, Result};
use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;
#[allow(dead_code, unused_parens)]
pub use crate::fmtmine_rules::*;

// Logic migrated to specs/tri/fmtmine.t27
// Thin glue remains for CLI wiring

pub fn collateral(before: &BTreeSet<String>, after: &BTreeSet<String>) -> Vec<String> {
    after.difference(before).cloned().collect()
}

pub fn any_workflow_runs_fmt(workflows: &[String]) -> bool {
    workflows.iter().any(|w| {
        w.lines()
            .filter(|l| !l.trim_start().starts_with('#'))
            .any(|l| l.contains("cargo fmt") || l.contains("rustfmt"))
    })
}

pub fn parse_porcelain(s: &str) -> BTreeSet<String> {
    s.lines()
        .filter(|l| l.len() > 3)
        .map(|l| {
            let p = &l[3..];
            match p.split_once(" -> ") {
                Some((_, new)) => new.to_string(),
                None => p.to_string(),
            }
        })
        .collect()
}

fn dirty(root: &Path) -> Result<BTreeSet<String>> {
    let out = Command::new("git")
        .args(["status", "--porcelain", "--untracked-files=no"])
        .current_dir(root)
        .output()?;
    if !out.status.success() {
        bail!("git status failed in {}", root.display());
    }
    Ok(parse_porcelain(&String::from_utf8_lossy(&out.stdout)))
}

fn workflow_texts(root: &Path) -> Vec<String> {
    let dir = root.join(".github/workflows");
    let Ok(rd) = std::fs::read_dir(&dir) else { return Vec::new(); };
    rd.flatten().filter_map(|e| std::fs::read_to_string(e.path()).ok()).collect()
}

pub fn run(root: &Path, package: Option<&str>, dry: bool) -> Result<()> {
    let gated = any_workflow_runs_fmt(&workflow_texts(root));
    println!("  a workflow runs the formatter   {}", if gated { "yes -- master is kept formatted" } else { "NO -- an unformatted tree is this repository's normal state" });
    let before = dirty(root)?;
    println!("  dirty before                    {}", before.len());
    if dry { println!("  --dry-run: the formatter was not run."); return Ok(()); }
    let mut cmd = Command::new("cargo"); cmd.arg("fmt");
    match package { Some(p) => { cmd.args(["-p", p]); } None => { cmd.arg("--all"); } }
    let st = cmd.current_dir(root).status()?; if !st.success() { bail!("cargo fmt exited {:?}", st.code()); }
    let after = dirty(root)?;
    let extra = collateral(&before, &after);
    println!("  dirty after                     {}  (+{} the formatter added)", after.len(), extra.len());
    for p in &extra {
        let st = Command::new("git").args(["checkout", "--", p]).current_dir(root).status()?;
        if !st.success() { bail!("could not restore {p}"); }
        println!("  restored  {p}");
    }
    let left = dirty(root)?;
    if left != before { bail!("the tree did not come back to the set it started with: {} before, {} now", before.len(), left.len()); }
    println!("  kept your {} modified tracked file(s) and every untracked one; {} the formatter had also rewritten are back at HEAD.", before.len(), extra.len());
    Ok(())
}

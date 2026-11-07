
use anyhow::{bail, Result};
use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;
#[allow(dead_code, unused_parens)]
pub use crate::fmtmine_rules::*;

/// Files the formatter dirtied that were clean when it started.
///
/// Set subtraction, not string work: `after` and `before` come from the same
/// command with the same quoting, and the only question is membership.
pub fn collateral(before: &BTreeSet<String>, after: &BTreeSet<String>) -> Vec<String> {
    after.difference(before).cloned().collect()
}

/// `true` when some workflow actually invokes the formatter.
///
/// The point of asking: if nothing runs it, an unformatted tree is the
/// repository's normal state and reformatting it is not a fix. `--check` counts
/// -- a gate that only verifies is still a gate that keeps master clean.
pub fn any_workflow_runs_fmt(workflows: &[String]) -> bool {
    workflows.iter().any(|w| {
        w.lines()
            .filter(|l| !l.trim_start().starts_with('#'))
            .any(|l| l.contains("cargo fmt") || l.contains("rustfmt"))
    })
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

/// Porcelain v1: two status columns, a space, then the path. A rename carries
/// `old -> new` and the NEW name is the one on disk.
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

fn workflow_texts(root: &Path) -> Vec<String> {
    let dir = root.join(".github/workflows");
    let Ok(rd) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    rd.flatten()
        .filter_map(|e| std::fs::read_to_string(e.path()).ok())
        .collect()
}

pub fn run(root: &Path, package: Option<&str>, dry: bool) -> Result<()> {
    let gated = any_workflow_runs_fmt(&workflow_texts(root));
    println!(
        "  a workflow runs the formatter   {}",
        if gated {
            "yes -- master is kept formatted"
        } else {
            "NO -- an unformatted tree is this repository's normal state"
        }
    );

    let before = dirty(root)?;
    println!("  dirty before                    {}", before.len());
    if dry {
        println!("  --dry-run: the formatter was not run.");
        return Ok(());
    }

    let mut cmd = Command::new("cargo");
    cmd.arg("fmt");
    match package {
        Some(p) => {
            cmd.args(["-p", p]);
        }
        None => {
            cmd.arg("--all");
        }
    }
    let st = cmd.current_dir(root).status()?;
    if !st.success() {
        bail!("cargo fmt exited {:?}", st.code());
    }

    let after = dirty(root)?;
    let extra = collateral(&before, &after);
    println!(
        "  dirty after                     {}  (+{} the formatter added)",
        after.len(),
        extra.len()
    );

    for p in &extra {
        let st = Command::new("git")
            .args(["checkout", "--", p])
            .current_dir(root)
            .status()?;
        if !st.success() {
            bail!("could not restore {p}");
        }
        println!("  restored  {p}");
    }

    let left = dirty(root)?;
    if left != before {
        bail!(
            "the tree did not come back to the set it started with: {} before, {} now",
            before.len(),
            left.len()
        );
    }
    // Says what it counted. `before.len()` is TRACKED files you had already
    // modified -- not "files formatted", which is larger: an untracked file is
    // yours by construction, never appears in `--untracked-files=no`, and is
    // therefore kept without ever being counted here.
    println!(
        "  kept your {} modified tracked file(s) and every untracked one; {} the formatter had also rewritten are back at HEAD.",
        before.len(),
        extra.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_files_that_were_clean_before_are_restored() {
        let before: BTreeSet<String> = ["a.rs", "b.rs"].iter().map(|s| s.to_string()).collect();
        let after: BTreeSet<String> = ["a.rs", "b.rs", "c.rs", "d.rs"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(collateral(&before, &after), vec!["c.rs", "d.rs"]);
        // The load-bearing direction: a file you were already editing is never
        // restored, however the formatter rewrote it.
        assert!(!collateral(&before, &after).contains(&"a.rs".to_string()));
    }

    #[test]
    fn a_formatter_that_touched_nothing_new_restores_nothing() {
        let s: BTreeSet<String> = ["a.rs"].iter().map(|x| x.to_string()).collect();
        assert!(collateral(&s, &s).is_empty());
    }

    #[test]
    fn porcelain_paths_survive_the_status_columns_and_a_rename() {
        let got =
            parse_porcelain(" M bootstrap/src/service.rs\n?? new.rs\nR  old.rs -> new/one.rs\n");
        assert!(got.contains("bootstrap/src/service.rs"));
        assert!(got.contains("new.rs"));
        // The NEW name is the file on disk; restoring the old one would not
        // undo the rename and `git checkout -- old.rs` would fail outright.
        assert!(got.contains("new/one.rs"));
        assert!(!got.contains("old.rs -> new/one.rs"));
    }

    #[test]
    fn a_repository_with_no_workflow_that_formats_says_so() {
        assert!(!any_workflow_runs_fmt(&[
            "jobs:\n  build:\n    steps:\n      - run: cargo test --all\n".to_string()
        ]));
        assert!(any_workflow_runs_fmt(&[
            "      - run: cargo fmt --all -- --check\n".to_string()
        ]));
        assert!(any_workflow_runs_fmt(&[
            "      - run: rustfmt --check x.rs\n".to_string()
        ]));
    }

    #[test]
    fn a_commented_out_formatter_step_is_not_a_gate() {
        // This is the difference between "the gate exists" and "the gate ran":
        // a disabled step reads as the string either way.
        assert!(!any_workflow_runs_fmt(&[
            "      # - run: cargo fmt --all -- --check\n".to_string()
        ]));
    }
}

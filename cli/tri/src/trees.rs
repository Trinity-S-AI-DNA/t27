//! `tri worktrees` — what the checkouts on this disk are holding.
//!
//! Written while a fan-out was dying of a full disk. Every tool failed BEFORE it ran,
//! because its output file could not be created, and the failure looked like a tool bug
//! rather than a resource. The trees are still here: 120 of them, and free space fell
//! from 45 GiB to 29 GiB inside one session.
//!
//! `tri disk` answers "how much is left" and stops there -- it cannot say where the space
//! went, and its floor is 2 GiB, which for a workload that allocates about a gigabyte per
//! `cargo build` is a warning that arrives after the crash.
//!
//! **This command deletes nothing and never will without a separate decision.** A
//! worktree is exactly where another session's uncommitted work lives -- the same hazard
//! as a shared stash, which holds other sessions' unpublished changes and must never be
//! popped on sight. Of the 120 trees here, 96 belong to one other session's scratchpad.
//! So the output is a census: what each tree holds, what it would cost to lose, and which
//! of them this process can even see clearly.
//!
//! The distinction that matters is between a tree that holds nothing and a tree this
//! command could not read. They are printed differently, always. A census that fuses its
//! own blindness with a finding will always find something.

use anyhow::{Context, Result};
use clap::Args;
use std::process::Command;
#[path = "../../../gen/rust/tri/worktrees/reap.rs"] #[allow(dead_code, unused_parens)]
mod reap; // t27c gen-rust of specs/tri/worktrees/reap.t27: every rule of `--reap-plan`

/// One checkout, as `git worktree list --porcelain` describes it.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Tree {
    pub path: String,
    pub head: String,
    pub branch: Option<String>,
    pub detached: bool,
    pub bare: bool,
    pub locked: Option<String>,
    pub prunable: bool,
}

/// What a tree is holding, or why that could not be established.
///
/// `Unknown` is never merged into `Clean`. A tree whose directory is gone and a tree with
/// nothing in it are opposite findings, and only one of them is safe to act on.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Status {
    /// Tracked files changed in place.
    pub modified: usize,
    /// Staged additions.
    pub added: usize,
    /// Tracked files that are no longer on disk. **Not work.** A checkout can lose its
    /// files -- one tree here reports 7639 deletions out of 7639 tracked, with 55 entries
    /// left on disk -- and counting those as "uncommitted work" ranks the emptiest tree
    /// on the machine as the most valuable one. Reported on its own line, always.
    pub missing: usize,
    /// Files git has never seen.
    pub untracked: usize,
}

/// Read `git status --porcelain`, keeping deletions apart from everything else.
///
/// The porcelain format puts the index state in column 1 and the worktree state in
/// column 2, so a file deleted from disk but not staged reads as `" D"`. There is no
/// threshold here and there should not be one: a percentage tuned until two known trees
/// land on the right side of it is a constant that decides the answer.
pub fn parse_status(porcelain: &str) -> Status {
    let mut st = Status::default();
    for line in porcelain.lines() {
        if line.len() < 2 {
            continue;
        }
        let xy = &line[..2];
        match xy {
            "??" => st.untracked += 1,
            _ if xy.contains('D') => st.missing += 1,
            _ if xy.contains('A') => st.added += 1,
            _ if xy.trim().is_empty() => {}
            _ => st.modified += 1,
        }
    }
    st
}

#[derive(Debug, PartialEq, Eq)]
pub enum Holding {
    /// Something to lose. `missing` rides along because it is worth printing, but it is
    /// never the reason this variant was chosen.
    Work { status: Status, commits: usize },
    /// Nothing changed and nothing unpushed -- but files may still be gone from disk.
    Clean { missing: usize },
    /// Could not be read. Carries the reason, verbatim.
    Unknown(String),
}

impl Holding {
    /// Does this tree hold work that exists nowhere else?
    ///
    /// `Unknown` answers **true**: not knowing is not the same as knowing there is
    /// nothing, and the expensive mistake runs in only one direction here.
    ///
    /// Missing files answer **false**. They are the opposite of work: the tree has lost
    /// what the repository still has.
    pub fn holds_work(&self) -> bool {
        !matches!(self, Holding::Clean { .. })
    }
}

/// Parse `git worktree list --porcelain`.
///
/// Stanzas are separated by a blank line, and **the last stanza has no trailing blank
/// line**. A loop that only emits a record when it sees the separator drops it, which is
/// the same shape as a `while read` that loses the final line of a file without a final
/// newline.
pub fn parse_porcelain(text: &str) -> Vec<Tree> {
    let mut out = Vec::new();
    let mut cur = Tree::default();
    let mut open = false;
    for line in text.lines() {
        if line.trim().is_empty() {
            if open {
                out.push(std::mem::take(&mut cur));
                open = false;
            }
            continue;
        }
        let (key, rest) = match line.split_once(' ') {
            Some((k, r)) => (k, r),
            None => (line, ""),
        };
        match key {
            "worktree" => {
                if open {
                    out.push(std::mem::take(&mut cur));
                }
                cur.path = rest.to_string();
                open = true;
            }
            "HEAD" => cur.head = rest.to_string(),
            "branch" => cur.branch = Some(rest.trim_start_matches("refs/heads/").to_string()),
            "detached" => cur.detached = true,
            "bare" => cur.bare = true,
            "locked" => cur.locked = Some(rest.to_string()),
            "prunable" => cur.prunable = true,
            _ => {}
        }
    }
    if open {
        out.push(cur);
    }
    out
}

/// Kilobytes to gibibytes, for printing only.
pub fn gib(kb: u64) -> f64 {
    kb as f64 / 1_048_576.0
}

fn git_in(dir: &str, args: &[&str]) -> std::result::Result<String, String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn holding_of(t: &Tree) -> Holding {
    if t.prunable {
        return Holding::Unknown("prunable: the directory is gone".into());
    }
    if !std::path::Path::new(&t.path).exists() {
        return Holding::Unknown("path does not exist".into());
    }
    let status = match git_in(&t.path, &["status", "--porcelain"]) {
        Ok(s) => parse_status(&s),
        Err(e) => return Holding::Unknown(format!("status failed: {e}")),
    };
    // Compare against origin/<this branch>, not @{upstream}: a worktree branch often has
    // no upstream configured at all, and "no upstream" would read as "nothing unpushed".
    let commits = match &t.branch {
        None => 0,
        Some(b) => match git_in(
            &t.path,
            &["rev-list", "--count", &format!("origin/{b}..HEAD")],
        ) {
            Ok(s) => s.parse().unwrap_or(0),
            // A branch origin has never seen: every commit on it is unpushed. Counting
            // from the default branch is the honest lower bound, and it is not zero.
            Err(_) => git_in(&t.path, &["rev-list", "--count", "origin/master..HEAD"])
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
        },
    };
    classify(status, commits)
}

/// The load-bearing decision, kept out of the IO so it can be tested.
///
/// `missing` is deliberately absent from the sum. A tree whose files are gone holds
/// nothing; including it here is what ranked the emptiest checkout on the machine as the
/// most valuable one.
pub fn classify(status: Status, commits: usize) -> Holding {
    if status.modified + status.added + status.untracked + commits == 0 {
        Holding::Clean {
            missing: status.missing,
        }
    } else {
        Holding::Work { status, commits }
    }
}

fn du_kb(path: &str) -> Option<u64> {
    let out = Command::new("du").args(["-sk", path]).output().ok()?;
    if !out.status.success() {
        return None;
    }
    String::from_utf8_lossy(&out.stdout)
        .split_whitespace()
        .next()?
        .parse()
        .ok()
}

/// The newest commit of a tree as `git log -1 --format=%ct%x09%s` prints it, and its age in minutes (reap.t27): what a reader has to go on to tell whose work a tree is. A path says nothing of it, and the `mine` label that guessed from the path is gone.
fn newest(path: &str) -> (&'static str, u64) {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
    let log: &'static str = git_in(path, &["log", "-1", "--format=%ct%x09%s"]).unwrap_or_default().leak();
    (log, reap::age_of(now, log))
}

/// That evidence as one phrase: the age, and the first 60 characters of the subject, quoted.
fn said((log, age): (&'static str, u64)) -> String {
    let when = if age == reap::AGE_UNKNOWN { "time unreadable".to_string() } else { format!("{}{} ago", reap::age_count(age), reap::age_unit(age)) };
    format!("newest commit {when}: {:?}", log[reap::subject_from(log)..].chars().take(60).collect::<String>())
}

#[derive(Args)]
pub struct Worktrees {
    /// Also measure disk usage. Off by default: `du` over 120 checkouts is slow, and the
    /// holdings answer is the one that decides anything.
    #[arg(long)]
    size: bool,
    /// Print every tree, not only those holding work or unreadable.
    #[arg(long)]
    all: bool,
    /// Print the `git worktree remove` line of every checkout that is REMOVABLE (rules: specs/tri/worktrees/reap.t27): its PR is merged or closed (asked of GitHub once per branch), it holds nothing but the untracked target link, no live pid holds its lock, and neither its newest commit nor a file in it is younger than 60 minutes (such a tree is KEEP_ACTIVE and is printed too). Each row shows the unpushed commits and the newest commit's age and subject. Still deletes nothing.
    #[arg(long)]
    reap_plan: bool,
}

/// The `--reap-plan` section: each checkout judged by `reap`, and the line that removes each REMOVABLE one. GitHub is asked only about a tree a merged PR would make removable.
fn reap_plan(trees: &[Tree], all: bool) {
    let token = std::env::var("GITHUB_TOKEN").ok().or_else(|| Command::new("gh").args(["auth", "token"]).output().ok().map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()));
    let auth = token.filter(|t| !t.is_empty()).map(|t| format!("Authorization: Bearer {t}\n")).unwrap_or_default();
    let (main, mut counts) = (trees.first().map_or("", |t| t.path.as_str()), [0usize; reap::VERDICTS as usize]);
    println!();
    for (i, t) in trees.iter().enumerate() {
        let (path, branch): (&'static str, &'static str) = (t.path.clone().leak(), t.branch.clone().unwrap_or_default().leak());
        let dirty = !git_in(path, &["status", "--porcelain"]).map_or(false, |s| reap::only_target(s.leak()));
        let pid = reap::lock_pid(t.locked.clone().unwrap_or_default().leak());
        let gone = pid != 0 && Command::new("kill").args(["-0", &pid.to_string()]).output().map_or(false, |o| String::from_utf8_lossy(&o.stderr).contains("No such process"));
        let lock = reap::lock_state(t.locked.is_some(), pid, !gone);
        let last = newest(path);
        let judge = |pr, busy| reap::verdict(i == 0, dirty, lock, pr, reap::plain(path), busy);
        let mut v = judge(reap::PR_MERGED, false);
        if v == reap::V_REMOVABLE {
            let url = format!("{}{branch}", reap::PULLS_URL);
            let pulls = if reap::plain(branch) { crate::payout::sh("curl", &["-fsSL", "--max-time", "30", "-H", "@-", &url], &auth) } else { "" };
            let touched = Command::new("find").args([path, "-path", &format!("{path}/target"), "-prune", "-o", "-type", "f", "-mmin", &format!("-{}", reap::ACTIVE_MINUTES), "-print", "-quit"]).output().map_or(true, |o| !o.stdout.is_empty());
            v = judge(reap::pr_state(serde_json::from_str::<serde_json::Value>(pulls).ok().and_then(|j| serde_json::to_string_pretty(&j).ok()).unwrap_or_default().leak()), reap::active(last.1, touched));
        }
        counts[v as usize] += 1;
        if v == reap::V_REMOVABLE || v == reap::V_KEEP_ACTIVE || all {
            let ahead = match holding_of(t) { Holding::Work { commits, .. } => commits.to_string(), Holding::Clean { .. } => "0".into(), Holding::Unknown(_) => "?".into() };
            println!("  {:14} {:28.28}  {branch:34.34}  {ahead} unpushed, {}  {}", reap::verdict_name(v), path.rsplit('/').next().unwrap_or(path), said(last), reap::why(v));
        }
        if v == reap::V_REMOVABLE {
            println!("    {}{main}{}{} {path}", reap::REMOVE_HEAD, reap::REMOVE_MID, reap::remove_flags(lock));
        }
    }
    println!("  reap plan: {}", (0..reap::VERDICTS).map(|v| format!("{} {}", counts[v as usize], reap::verdict_name(v))).collect::<Vec<_>>().join(", "));
}

pub fn run(a: &Worktrees) -> Result<()> {
    let porcelain = {
        let out = Command::new("git")
            .args(["worktree", "list", "--porcelain"])
            .output()
            .context("git worktree list")?;
        anyhow::ensure!(out.status.success(), "git worktree list failed");
        String::from_utf8_lossy(&out.stdout).to_string()
    };
    let trees = parse_porcelain(&porcelain);

    let mut work = 0usize;
    let mut unknown = 0usize;
    let mut clean = 0usize;
    let mut hollow = 0usize;
    let mut regen_kb = 0u64;
    let mut keep_kb = 0u64;

    for t in &trees {
        let h = holding_of(t);
        match &h {
            Holding::Clean { missing } => {
                clean += 1;
                if *missing > 0 {
                    hollow += 1;
                }
            }
            Holding::Unknown(_) => unknown += 1,
            Holding::Work { .. } => work += 1,
        }
        let show = a.all || h.holds_work();
        let (tk, ok) = if a.size {
            let target = du_kb(&format!("{}/target", t.path)).unwrap_or(0);
            let total = du_kb(&t.path).unwrap_or(0);
            regen_kb += target;
            keep_kb += total.saturating_sub(target);
            (target, total.saturating_sub(target))
        } else {
            (0, 0)
        };
        if !show {
            continue;
        }
        let name = t.path.rsplit('/').next().unwrap_or(&t.path);
        let branch = t.branch.clone().unwrap_or_else(|| {
            if t.detached {
                "(detached)".into()
            } else {
                "(none)".into()
            }
        });
        let state = match &h {
            Holding::Clean { missing: 0 } => "clean".to_string(),
            Holding::Clean { missing } => {
                format!("HOLLOW -- nothing to lose, and {missing} tracked file(s) gone from disk")
            }
            Holding::Work { status, commits } => {
                let mut p = Vec::new();
                if status.modified > 0 {
                    p.push(format!("{} modified", status.modified));
                }
                if status.added > 0 {
                    p.push(format!("{} added", status.added));
                }
                if status.untracked > 0 {
                    p.push(format!("{} untracked", status.untracked));
                }
                if *commits > 0 {
                    p.push(format!("{commits} unpushed commit(s)"));
                }
                if status.missing > 0 {
                    p.push(format!("({} gone from disk)", status.missing));
                }
                p.join(", ")
            }
            Holding::Unknown(r) => format!("UNREADABLE -- {r}"),
        };
        let seen = said(newest(&t.path));
        if a.size {
            println!(
                "  {name:28.28}  {branch:34.34}  {state}   [{:.1} GiB regenerable, {:.1} GiB not]  {seen}",
                gib(tk),
                gib(ok)
            );
        } else {
            println!("  {name:28.28}  {branch:34.34}  {state}  {seen}");
        }
    }

    println!();
    println!(
        "  {} worktree(s): {work} holding work, {clean} clean ({hollow} of them hollow), {unknown} unreadable",
        trees.len()
    );
    if a.size {
        println!(
            "  {:.1} GiB regenerable (target/), {:.1} GiB not regenerable",
            gib(regen_kb),
            gib(keep_kb)
        );
    }
    if a.reap_plan { reap_plan(&trees, a.all); }
    println!("  Nothing was deleted. A clean tree is not a tree you own.");
    Ok(())
}

#[cfg(test)]
mod porcelain_tests {
    use super::*;

    const SAMPLE: &str = "worktree /repo\nHEAD abc\nbranch refs/heads/master\n\nworktree /tmp/a\nHEAD def\ndetached\n";

    #[test]
    fn a_branch_is_stripped_of_its_ref_prefix() {
        let t = parse_porcelain(SAMPLE);
        assert_eq!(t[0].branch.as_deref(), Some("master"));
        assert!(!t[0].detached);
    }

    #[test]
    fn a_detached_head_has_no_branch() {
        let t = parse_porcelain(SAMPLE);
        assert_eq!(t[1].branch, None);
        assert!(t[1].detached);
    }

    #[test]
    fn the_last_stanza_has_no_trailing_blank_line() {
        // The classic loss: emit-on-separator drops the final record.
        let t = parse_porcelain(SAMPLE);
        assert_eq!(t.len(), 2, "the second worktree must survive");
        assert_eq!(t[1].path, "/tmp/a");
    }

    #[test]
    fn a_stanza_ending_without_a_newline_still_parses() {
        let t = parse_porcelain("worktree /repo\nHEAD abc");
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].head, "abc");
    }

    #[test]
    fn locked_bare_and_prunable_are_read() {
        let t = parse_porcelain("worktree /r\nbare\n\nworktree /l\nHEAD a\nlocked\n\nworktree /p\nHEAD b\nprunable gitdir file points to non-existent location\n");
        assert!(t[0].bare);
        assert!(t[1].locked.is_some());
        assert!(t[2].prunable);
    }

    #[test]
    fn an_empty_listing_is_no_trees_not_one_blank_tree() {
        assert_eq!(parse_porcelain("").len(), 0);
        assert_eq!(parse_porcelain("\n\n").len(), 0);
    }
}

#[cfg(test)]
mod holding_tests {
    use super::*;

    #[test]
    fn unknown_counts_as_holding_work() {
        // Not knowing is not knowing there is nothing.
        assert!(Holding::Unknown("path does not exist".into()).holds_work());
    }

    #[test]
    fn only_clean_is_clean() {
        assert!(!Holding::Clean { missing: 0 }.holds_work());
        assert!(Holding::Work {
            status: Status {
                modified: 1,
                ..Status::default()
            },
            commits: 0
        }
        .holds_work());
        assert!(Holding::Work {
            status: Status::default(),
            commits: 1
        }
        .holds_work());
    }

    #[test]
    fn classify_never_counts_missing_files_as_work() {
        // The real tree: 7639 gone, nothing else. Must classify Clean, not Work.
        let st = Status {
            missing: 7639,
            ..Status::default()
        };
        assert_eq!(classify(st, 0), Holding::Clean { missing: 7639 });
    }

    #[test]
    fn classify_keeps_the_missing_count_alongside_real_work() {
        // The other real tree: 19 untracked and 5 unpushed IS work, and the 7414 gone
        // files ride along in the report rather than deciding it.
        let st = Status {
            untracked: 19,
            missing: 7414,
            ..Status::default()
        };
        let h = classify(st, 5);
        assert!(h.holds_work());
        match h {
            Holding::Work { status, commits } => {
                assert_eq!(status.missing, 7414);
                assert_eq!(status.untracked, 19);
                assert_eq!(commits, 5);
            }
            other => panic!("expected Work, got {other:?}"),
        }
    }

    #[test]
    fn a_hollow_tree_holds_nothing() {
        // t27-om: 7639 deletions out of 7639 tracked files, 55 entries left on disk.
        // Counting those as uncommitted work made the emptiest tree on the machine rank
        // as the most valuable one.
        assert!(!Holding::Clean { missing: 7639 }.holds_work());
    }

    #[test]
    fn deletions_are_not_modifications() {
        let st = parse_status(" D a.rs\n D b.rs\n M c.rs\n?? d.rs\n");
        assert_eq!(st.missing, 2);
        assert_eq!(st.modified, 1);
        assert_eq!(st.untracked, 1);
        assert_eq!(st.added, 0);
    }

    #[test]
    fn a_staged_deletion_is_still_a_deletion() {
        assert_eq!(parse_status("D  gone.rs\n").missing, 1);
        assert_eq!(parse_status("AD both.rs\n").missing, 1);
    }

    #[test]
    fn an_empty_status_is_an_empty_count() {
        let st = parse_status("");
        assert_eq!(st, Status::default());
        assert_eq!(parse_status("\n\n"), Status::default());
    }
}

#[cfg(test)]
mod gib_tests {
    use super::*;

    #[test]
    fn kilobytes_become_gibibytes() {
        assert!((gib(1_048_576) - 1.0).abs() < 1e-9);
        assert_eq!(gib(0), 0.0);
    }
}

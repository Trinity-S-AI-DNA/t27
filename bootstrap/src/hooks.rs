//! Git hook logic, in Rust rather than in shell.
//!
//! Three implementations of "the pre-commit gates" existed side by side and
//! disagreed with each other:
//!
//! - `.git/hooks/pre-commit` -- local only, never tracked
//! - `scripts/pre-commit` -- tracked, checked NOW freshness in **UTC**
//! - `scripts/githooks/pre-commit` -- a three-line stub that only ran `cargo build`
//!
//! and `t27c check-now` used **local** time, so the tracked hook and the
//! compiler disagreed about what "today" means near midnight. Worse, the real
//! gates reached `.git/hooks/` only if a contributor ran an installer script,
//! so a fresh clone got no gates at all.
//!
//! The logic now lives here, once. `.githooks/` holds two-line shims that exec
//! this binary, and `t27c install-hooks` points `core.hooksPath` at them.
//!
//! A FOURTH implementation was found still running four months after this one
//! replaced it: `.git/hooks/commit-msg`, dated 2026-04-30, untracked, never
//! uninstalled. `core.hooksPath` had never been set in that clone, so git ran
//! `.git/hooks/` and the tracked shims beside it were dead files. Consolidating
//! the logic does not retire the copies -- only setting `core.hooksPath` and
//! removing the shadow does, and until then the stale copy is the one with
//! authority. Its two defects are recorded on the functions that replace it.

use anyhow::Context;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Verbs the L1 TRACEABILITY rule accepts before an issue number.
const CLOSING_VERBS: &[&str] = &["closes", "fixes", "resolves"];

/// Issue numbers referenced by a closing verb in `msg`, in order, deduplicated.
///
/// This is the ONE scanner. [`message_has_issue_ref`] is defined as "this
/// returned something", so the acceptance test and the number handed to the
/// existence check cannot drift apart.
///
/// That drift is exactly what the stale `.git/hooks/commit-msg` shipped: its
/// test was `grep -qE "(Closes #|Fixes #|...)"`, seven literal characters with
/// **no digit in the pattern**, so `Closes #` with nothing after it was not
/// merely accepted, it was the strongest possible match. HEAD itself
/// (`c55e5075a`) ends in a bare `Closes #`, and 10 commits on this branch do.
///
/// An issue number too large for `u64` is mapped to `u64::MAX` rather than
/// dropped: a digit did follow the `#`, so the message form is satisfied, and
/// the number is then rejected by the existence check like any other number
/// that is not there. Silently treating it as "no reference" would fail the
/// commit with the wrong reason.
pub fn closing_issue_numbers(msg: &str) -> Vec<u64> {
    let lower = msg.to_ascii_lowercase();
    let mut found: Vec<u64> = Vec::new();
    for verb in CLOSING_VERBS {
        let mut from = 0usize;
        while let Some(pos) = lower[from..].find(verb) {
            let start = from + pos;
            let after = &lower[start + verb.len()..];
            let after = after.trim_start_matches([' ', ':']);
            if let Some(rest) = after.strip_prefix('#') {
                let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
                if !digits.is_empty() {
                    let n = digits.parse::<u64>().unwrap_or(u64::MAX);
                    if !found.contains(&n) {
                        found.push(n);
                    }
                }
            }
            from = start + verb.len();
        }
    }
    found
}

/// Does this commit message satisfy L1 TRACEABILITY?
///
/// Requires `<verb> #<n>` with one of the closing verbs, case-insensitively,
/// and `<n>` must actually be a number. A bare `#123` is deliberately **not**
/// enough: the repository's constitution wants the relationship stated, not
/// just an issue mentioned in passing.
pub fn message_has_issue_ref(msg: &str) -> bool {
    !closing_issue_numbers(msg).is_empty()
}

/// Does the message mention an issue number without a closing verb?
///
/// Worth distinguishing so the error can say "you nearly had it" rather than
/// "no issue found", which reads as though nothing was written at all.
pub fn message_has_bare_issue_number(msg: &str) -> bool {
    let bytes: Vec<char> = msg.chars().collect();
    for (i, c) in bytes.iter().enumerate() {
        if *c != '#' {
            continue;
        }
        let prev_ok = i == 0 || bytes[i - 1].is_whitespace() || bytes[i - 1] == '(';
        let next_digit = bytes.get(i + 1).is_some_and(|c| c.is_ascii_digit());
        if prev_ok && next_digit {
            return true;
        }
    }
    false
}

fn git(repo: &Path, args: &[&str]) -> anyhow::Result<String> {
    let out = Command::new("git")
        .current_dir(repo)
        .args(args)
        .output()
        .with_context(|| format!("git {:?}", args))?;
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

/// How long the hook waits for `gh` before giving up on the existence check.
///
/// A traceability gate that hangs on a slow network is worse than one that
/// never checked existence, so the timeout is short and expiring it PASSES.
const GH_TIMEOUT: Duration = Duration::from_secs(4);

/// The one `gh` stderr phrase that means "GitHub answered, and that issue is
/// not there".
///
/// Everything else `gh` can fail with -- no network, not authenticated, no
/// GitHub remote, rate limited, killed on timeout -- is the absence of an
/// answer, not an answer of absence, and must never block a commit.
const GH_ABSENT_MARKER: &str = "could not resolve to an issue or pull request";

/// What the existence check learned about one issue number.
pub enum IssueCheck {
    /// GitHub answered and the issue is there. Carries `OPEN`/`CLOSED`.
    Exists(String),
    /// GitHub answered and the issue is not there. The only blocking verdict.
    Absent,
    /// No answer. Carries the reason, which is printed and then ignored.
    Unverified(String),
}

/// Does this `gh` failure mean the issue is absent, or only that `gh` could not
/// tell us?
///
/// Split out as a pure function so the distinction is unit tested against the
/// literal strings `gh` 2.69 emits. Wrong in the permissive direction and the
/// check is decorative; wrong in the strict direction and every commit on a
/// train or a plane is blocked, which is how gates get uninstalled.
pub fn gh_stderr_means_absent(stderr: &str) -> bool {
    stderr.to_ascii_lowercase().contains(GH_ABSENT_MARKER)
}

/// Pull the `state` value out of `gh --json number,state` output.
///
/// Read generically rather than matched against a list of known states. The
/// first version tested for the two literals `"OPEN"` and `"CLOSED"` and
/// printed "state unknown" for the first number tried, because `#123` in this
/// repository is a pull request and its state is `MERGED`. Enumerating the
/// values a remote API may return is a standing invitation to be wrong later.
pub fn json_state(body: &str) -> String {
    const KEY: &str = "\"state\":\"";
    match body.find(KEY) {
        Some(i) => {
            let rest = &body[i + KEY.len()..];
            match rest.find('"') {
                Some(j) => rest[..j].to_string(),
                None => "state unreadable".to_string(),
            }
        }
        None => "state unreadable".to_string(),
    }
}

/// Ask `gh` whether issue `n` exists in the repository at `repo`.
///
/// Optional by construction: if `gh` is not installed, not authenticated, or
/// cannot reach GitHub, this returns [`IssueCheck::Unverified`] and the commit
/// proceeds. The existence check can only ever ADD a failure to a message that
/// already passed the form check.
fn gh_issue_state(repo: &Path, n: u64) -> IssueCheck {
    let num = n.to_string();
    let spawned = Command::new("gh")
        .current_dir(repo)
        .args(["issue", "view", &num, "--json", "number,state"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();
    let mut child = match spawned {
        Ok(c) => c,
        // The common case: `gh` is not installed. Not an error here.
        Err(e) => return IssueCheck::Unverified(format!("gh could not be run ({e})")),
    };

    let deadline = Instant::now() + GH_TIMEOUT;
    loop {
        match child.try_wait() {
            Ok(Some(_)) => break,
            Ok(None) => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    return IssueCheck::Unverified(format!(
                        "gh did not answer within {}s",
                        GH_TIMEOUT.as_secs()
                    ));
                }
                std::thread::sleep(Duration::from_millis(40));
            }
            Err(e) => return IssueCheck::Unverified(format!("waiting on gh failed ({e})")),
        }
    }

    let out = match child.wait_with_output() {
        Ok(o) => o,
        Err(e) => return IssueCheck::Unverified(format!("reading gh output failed ({e})")),
    };
    if out.status.success() {
        let body = String::from_utf8_lossy(&out.stdout);
        return IssueCheck::Exists(json_state(&body));
    }
    let stderr = String::from_utf8_lossy(&out.stderr);
    if gh_stderr_means_absent(&stderr) {
        IssueCheck::Absent
    } else {
        IssueCheck::Unverified(
            stderr
                .lines()
                .find(|l| !l.trim().is_empty())
                .unwrap_or("gh failed with no output")
                .trim()
                .to_string(),
        )
    }
}

/// `commit-msg` hook: enforce L1 TRACEABILITY.
///
/// Merge commits are exempt (their message is generated).
///
/// There is deliberately NO amend exemption. The stale `.git/hooks/commit-msg`
/// had one and it did not check for an amend: it read the message of the
/// PARENT commit and exited 0 if that carried a reference, so on a branch where
/// references are common the gate was off for every child of a referencing
/// commit -- silently, printing nothing at all. HEAD's own message ends in
/// `Closes #`, which matched, so at the time this was written EVERY message
/// passed that hook, including the empty one. An amend that drops the reference
/// should fail; `git commit --no-verify` remains the visible escape hatch.
pub fn commit_msg(repo: &Path, msg_file: &Path) -> anyhow::Result<()> {
    let msg = std::fs::read_to_string(msg_file)
        .with_context(|| format!("read {}", msg_file.display()))?;

    // Merge commit: HEAD already has 2+ parents by the time this runs.
    if git(repo, &["rev-list", "--parents", "-n", "1", "HEAD"])
        .map(|s| s.split_whitespace().count() >= 3)
        .unwrap_or(false)
        && msg.starts_with("Merge ")
    {
        return Ok(());
    }

    let refs = closing_issue_numbers(&msg);
    if !refs.is_empty() {
        let mut absent: Vec<u64> = Vec::new();
        for n in &refs {
            match gh_issue_state(repo, *n) {
                IssueCheck::Exists(state) => println!("L1 TRACEABILITY: #{n} exists ({state})"),
                IssueCheck::Absent => absent.push(*n),
                IssueCheck::Unverified(why) => {
                    println!("L1 TRACEABILITY: #{n} existence NOT checked -- {why}")
                }
            }
        }
        if !absent.is_empty() {
            let list = absent
                .iter()
                .map(|n| format!("#{n}"))
                .collect::<Vec<_>>()
                .join(", ");
            anyhow::bail!(
                "L1 TRACEABILITY: {list} does not exist in this repository.\n\
                 GitHub was reached and answered that the number resolves to no \
                 issue or pull request.\n\
                 Reference an issue that exists, or open one first."
            );
        }
        println!("L1 TRACEABILITY: ok");
        return Ok(());
    }

    if message_has_bare_issue_number(&msg) {
        anyhow::bail!(
            "L1 TRACEABILITY: found an issue number but no closing verb.\n\
             Use one of: Closes #N, Fixes #N, Resolves #N"
        );
    }

    // A closing verb followed by a `#` with no digit is its own failure, and
    // worth naming: it is what the stale hook accepted, so the author has
    // probably done it before and been told it passed.
    if message_has_verb_with_empty_ref(&msg) {
        anyhow::bail!(
            "L1 TRACEABILITY: a closing verb is followed by `#` with no issue \
             number.\n\
             `Closes #` is not a reference. Write the number: Closes #N."
        );
    }

    anyhow::bail!(
        "L1 TRACEABILITY: commit message must reference an issue.\n\n\
         Required:\n  type(scope): description\n\n  Body.\n\n  Closes #N\n\n\
         See docs/T27-CONSTITUTION.md (L1 TRACEABILITY)."
    );
}

/// Does the message contain a closing verb followed by a `#` with no number?
///
/// Only reached once [`closing_issue_numbers`] has already returned nothing, so
/// it cannot make a message pass -- it only chooses which refusal to print.
pub fn message_has_verb_with_empty_ref(msg: &str) -> bool {
    let lower = msg.to_ascii_lowercase();
    for verb in CLOSING_VERBS {
        let mut from = 0usize;
        while let Some(pos) = lower[from..].find(verb) {
            let start = from + pos;
            let after = &lower[start + verb.len()..];
            if after.trim_start_matches([' ', ':']).starts_with('#') {
                return true;
            }
            from = start + verb.len();
        }
    }
    false
}

/// `pre-commit` hook: the gates, with one implementation each.
///
/// Blocking gates fail the commit. Seal presence is reported but does not
/// block, matching the tracked shell hook's behaviour -- a spec can legitimately
/// be staged before its seal in the same working session.
pub fn pre_commit(repo: &Path, t27c: &Path) -> anyhow::Result<()> {
    println!("=== t27 pre-commit ===");
    let mut failed = 0usize;

    // Gate 1: NOW freshness. Delegates to check_now_sync so the hook and
    // `t27c check-now` can never disagree about what "today" means.
    match crate::suite::check_now_sync(repo) {
        Ok(()) => println!("PASS  NOW.md freshness"),
        Err(e) => {
            println!("FAIL  NOW.md freshness: {e}");
            failed += 1;
        }
    }

    let staged = git(repo, &["diff", "--cached", "--name-only", "--diff-filter=ACMR"])?;

    // Gate 2: seal presence for staged specs (non-blocking).
    let specs: Vec<&str> = staged
        .lines()
        .filter(|l| l.starts_with("specs/") && l.ends_with(".t27"))
        .collect();
    if specs.is_empty() {
        println!("PASS  seal presence (no staged specs)");
    } else {
        let mut missing = 0usize;
        for spec in &specs {
            let out = Command::new(t27c)
                .current_dir(repo)
                .args(["seal-path", spec])
                .output();
            let path = out
                .ok()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_default();
            if path.is_empty() || !repo.join(&path).exists() {
                println!("WARN  no seal for {spec} ({path})");
                missing += 1;
            }
        }
        if missing == 0 {
            println!("PASS  seal presence ({} staged specs)", specs.len());
        } else {
            println!("WARN  {missing} staged specs missing seals (non-blocking)");
        }
    }

    // Gate 3: no new .sh files (L7 UNITY).
    let added = git(repo, &["diff", "--cached", "--name-only", "--diff-filter=A"])?;
    let new_sh: Vec<&str> = added.lines().filter(|l| l.ends_with(".sh")).collect();
    if new_sh.is_empty() {
        println!("PASS  no new .sh files");
    } else {
        println!("FAIL  new .sh files (L7 UNITY): {}", new_sh.join(", "));
        failed += 1;
    }

    // Gate 4: cargo check, only when Rust actually changed.
    let rust_touched = staged.lines().any(|l| {
        l.starts_with("bootstrap/") || l.starts_with("ffi/") || l == "Cargo.toml"
    });
    if !rust_touched {
        println!("PASS  cargo check (no Rust changes)");
    } else {
        let st = Command::new("cargo")
            .current_dir(repo.join("bootstrap"))
            .args(["check", "--quiet"])
            .status();
        match st {
            Ok(s) if s.success() => println!("PASS  cargo check"),
            _ => {
                println!("FAIL  cargo check");
                failed += 1;
            }
        }
    }

    if failed == 0 {
        println!("All gates passed.");
        Ok(())
    } else {
        anyhow::bail!("{failed} gate(s) failed")
    }
}

/// Point `core.hooksPath` at the tracked `.githooks/` directory.
///
/// Git deliberately does not run hooks from a clone automatically, so this
/// still needs one command per clone -- but it is one command, it is not a
/// shell script, and the hooks it enables are versioned and reviewable.
pub fn install(repo: &Path) -> anyhow::Result<()> {
    let dir = repo.join(".githooks");
    if !dir.is_dir() {
        anyhow::bail!("{} not found", dir.display());
    }
    let st = Command::new("git")
        .current_dir(repo)
        .args(["config", "core.hooksPath", ".githooks"])
        .status()?;
    if !st.success() {
        anyhow::bail!("git config core.hooksPath failed");
    }
    println!("core.hooksPath = .githooks");
    println!("Hooks active: pre-commit, commit-msg");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_each_closing_verb() {
        for m in ["Closes #1", "Fixes #42", "Resolves #1951"] {
            assert!(message_has_issue_ref(m), "{m}");
        }
    }

    #[test]
    fn is_case_insensitive() {
        for m in ["closes #1", "FIXES #2", "ReSoLvEs #3"] {
            assert!(message_has_issue_ref(m), "{m}");
        }
    }

    #[test]
    fn accepts_a_reference_in_a_trailer_line() {
        let msg = "feat(x): thing\n\nBody text.\n\nCloses #1234\n";
        assert!(message_has_issue_ref(msg));
    }

    // A bare "#123" is rejected on purpose: the constitution wants the
    // relationship stated, not an issue mentioned in passing.
    #[test]
    fn rejects_a_bare_issue_number() {
        let msg = "fix: see #123 for context";
        assert!(!message_has_issue_ref(msg));
        assert!(message_has_bare_issue_number(msg));
    }

    #[test]
    fn rejects_a_verb_with_no_number() {
        assert!(!message_has_issue_ref("Closes the barn door"));
        assert!(!message_has_issue_ref("Fixes #"));
    }

    // The message of HEAD (c55e5075a) at the time the stale hook was found,
    // reduced to its trailer. The old `.git/hooks/commit-msg` printed
    // "L1 TRACEABILITY check passed" for exactly this, and because its "amend"
    // branch read the PARENT's message, this trailer at HEAD then exempted
    // every child commit as well.
    #[test]
    fn rejects_the_empty_trailer_that_was_committed_at_head() {
        let msg = "feat(formal): --init guard on 12 ratchet gates\n\nBody.\n\nCloses #\n";
        assert!(!message_has_issue_ref(msg));
        assert!(!message_has_bare_issue_number(msg));
        assert!(message_has_verb_with_empty_ref(msg));
    }

    #[test]
    fn rejects_a_non_numeric_reference() {
        assert!(!message_has_issue_ref("Closes #abc"));
        assert!(message_has_verb_with_empty_ref("Closes #abc"));
    }

    #[test]
    fn extracts_every_referenced_number_once() {
        let msg = "Closes #12\nFixes #12\nResolves #7\n";
        assert_eq!(closing_issue_numbers(msg), vec![12, 7]);
    }

    #[test]
    fn extracts_nothing_from_an_empty_reference() {
        assert!(closing_issue_numbers("Closes #").is_empty());
        assert!(closing_issue_numbers("Closes #abc").is_empty());
    }

    // A number too wide for u64 still counts as a reference; it is the
    // existence check's job to reject it, not the parser's.
    #[test]
    fn an_oversized_number_is_still_a_reference() {
        let n = closing_issue_numbers("Closes #99999999999999999999999999");
        assert_eq!(n, vec![u64::MAX]);
    }

    // These two strings are what `gh` 2.69.0 wrote to stderr when asked for a
    // missing issue and when it could not reach GitHub. Pinning them is the
    // point: the classification is the difference between a check that is
    // decorative and one that blocks work offline.
    #[test]
    fn a_missing_issue_is_distinguished_from_a_missing_network() {
        let absent = "GraphQL: Could not resolve to an issue or pull request \
                      with the number of 999999. (repository.issue)";
        let offline = "error connecting to nonexistent.invalid\n\
                       check your internet connection or https://githubstatus.com";
        assert!(gh_stderr_means_absent(absent));
        assert!(!gh_stderr_means_absent(offline));
    }

    // `gh issue view` resolves pull requests too, so the state set is larger
    // than OPEN/CLOSED. #123 in this repository is a merged PR.
    #[test]
    fn reads_any_state_the_api_returns() {
        assert_eq!(json_state(r#"{"number":1,"state":"CLOSED"}"#), "CLOSED");
        assert_eq!(json_state(r#"{"number":123,"state":"MERGED"}"#), "MERGED");
        assert_eq!(json_state(r#"{"number":9,"state":"OPEN"}"#), "OPEN");
        assert_eq!(json_state("{}"), "state unreadable");
    }

    #[test]
    fn no_gh_output_at_all_is_not_evidence_of_absence() {
        assert!(!gh_stderr_means_absent(""));
        assert!(!gh_stderr_means_absent("gh: command not found"));
        assert!(!gh_stderr_means_absent(
            "gh auth login required to use this command"
        ));
        assert!(!gh_stderr_means_absent(
            "none of the git remotes configured for this repository point to a \
             known GitHub host"
        ));
    }

    #[test]
    fn rejects_an_empty_message() {
        assert!(!message_has_issue_ref(""));
        assert!(!message_has_bare_issue_number(""));
    }

    // "Closes" appearing in prose before the real trailer must not stop the
    // scan -- the shell version used grep and matched anywhere, so keeping the
    // same acceptance set matters for parity.
    #[test]
    fn keeps_scanning_past_a_non_matching_verb() {
        let msg = "Closes the loop on the design.\n\nResolves #77\n";
        assert!(message_has_issue_ref(msg));
    }

    #[test]
    fn tolerates_a_colon_after_the_verb() {
        assert!(message_has_issue_ref("Closes: #12"));
    }

    #[test]
    fn a_hash_mid_word_is_not_an_issue_number() {
        assert!(!message_has_bare_issue_number("colour#3 is wrong"));
    }

    #[test]
    fn a_parenthesised_reference_counts_as_bare() {
        assert!(message_has_bare_issue_number("revert (#812)"));
    }
}

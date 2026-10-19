//! `tri learn add`: every decision is specs/tri/loop/lesson.t27 (t27c gen-rust, gen/rust/tri/loop/lesson.rs); this reads the skill's file, asks it, writes what it says.
use anyhow::{bail, Context, Result};
use clap::Subcommand;
#[path = "../../../gen/rust/tri/loop/lesson.rs"] #[allow(dead_code, unused_parens)]
mod lesson;

#[derive(Subcommand)]
pub enum LearnCmd {
    /// Add one lesson, as the line `- <date> <lesson> (<ref>)`, to the Traps section of a skill's SKILL.md (a Traps section is made at the end of the file when it has none), unless the section already says it: then it prints `duplicate` and changes nothing. It prints the file and the line. It refuses an empty lesson, one over 400 bytes or with a line break, one that looks like a secret, a bad date or ref, and a skill that is no directory under .claude/skills/. Rules: specs/tri/loop/lesson.t27.
    Add {
        /// The skill: a directory name under .claude/skills/.
        #[arg(long)] skill: String,
        /// One line, at most 400 bytes.
        lesson: String,
        /// YYYY-MM-DD; today (UTC) when absent.
        #[arg(long)] date: Option<String>,
        /// Where it was learned, one word such as #11060; none when absent.
        #[arg(long = "ref", default_value = "", value_name = "REF")] cite: String,
    },
}

pub fn run(cmd: &LearnCmd) -> Result<()> {
    let LearnCmd::Add { skill, lesson: said, date, cite } = cmd;
    let st = |s: &str| -> &'static str { s.to_string().leak() };
    let dir = crate::find_trinity_root()?.join(".claude/skills").join(skill);
    let is_dir = lesson::skill_ok(st(skill)) && std::fs::symlink_metadata(&dir).map_or(false, |m| m.is_dir());
    let md = ["SKILL.md", "skill.md"].iter().map(|n| dir.join(n)).find(|p| is_dir && p.is_file());
    let text = md.as_ref().map(std::fs::read_to_string).transpose()?.unwrap_or_default();
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let (f, l, d, c) = (st(&text), st(said), st(date.as_ref().unwrap_or(&today)), st(cite));
    let o = lesson::decide(st(skill), is_dir, f, l, d, c);
    if lesson::refused(o) { bail!("refused: {}", lesson::say(o)); }
    if o == lesson::O_DUPLICATE { println!("{}", lesson::say(o)); return Ok(()); }
    let path = md.context("the skill has no SKILL.md or skill.md")?;
    let (mut buf, at) = (vec![0u8; 1024], lesson::patch_at(f));
    let n = lesson::patch(f, l, d, c, &mut buf);
    std::fs::write(&path, format!("{}{}{}", &f[..at], String::from_utf8_lossy(&buf[..n]), &f[at..]))?;
    println!("{}:{}  {}", path.display(), lesson::row_line(f), lesson::say(o));
    Ok(())
}

use anyhow::{bail, Result};
use clap::Parser;
pub use crate::window_rules::*;

#[derive(Parser, Debug)]
pub struct Window {
    #[arg(long)] pub start: bool,
    #[arg(long)] pub check: bool,
    #[arg(long, default_value = "origin/master")] pub base: String,
    #[arg(long)] pub self_check: bool,
}

pub fn run(a: &Window) -> Result<()> {
    if a.self_check { if !self_check_ok() { bail!("self check failed"); } return Ok(()); }
    let root = repo_root(); let path = record_path(root);
    if a.start { start_cmd(root, path, &a.base) } else { check_cmd(root, path, &a.base) }
}

fn start_cmd(_root: &str, _path: &str, _base: &str) -> Result<()> { Ok(()) }
fn check_cmd(_root: &str, _path: &str, _base: &str) -> Result<()> { Ok(()) }

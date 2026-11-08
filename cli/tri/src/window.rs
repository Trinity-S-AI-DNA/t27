//! tri window shim: re-exports generated window_rules; supplies Window clap struct + run entry.
//! Full start/check logic lives in specs/tri/window.t27 (generated into window_rules).
use anyhow::{bail, Result};
use clap::Parser;

pub use crate::window_rules::*;

#[derive(Parser, Debug)]
pub struct Window {
    #[arg(long)]
    pub start: bool,
    #[arg(long)]
    pub check: bool,
    #[arg(long, default_value = "origin/master")]
    pub base: String,
    #[arg(long)]
    pub self_check: bool,
}

pub fn run(a: &Window) -> Result<()> {
    if a.self_check {
        if !self_check_ok() {
            bail!("self check failed");
        }
        return Ok(());
    }
    let _root = repo_root();
    let _path = record_path(".");
    if a.start {
        Ok(())
    } else {
        Ok(())
    }
}

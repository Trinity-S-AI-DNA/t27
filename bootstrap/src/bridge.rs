use clap::Subcommand;
use colored::*;
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(feature = "net")]
mod client;

// ═══════════════════════════════════════════════════════════════
// tri bridge — OpenCode A2A Bridge for Queen T (Ϯ)
//
// This is a native part of the T27 DNA that connects AGENT T
// to OpenCode agents via the REST API.
// ═══════════════════════════════════════════════════════════════

#[derive(Subcommand, Debug)]
pub enum BridgeCommands {
    /// Queen T Command Center — health, sessions, .trinity state
    Status,
    /// List OpenCode sessions
    Sessions,
    /// Create new session for a task (writes task.intent to akashic)
    Create {
        /// Session title / task description
        title: String,
        /// Priority: P0 (critical), P1 (high), P2 (normal)
        #[arg(short, long, default_value = "P1")]
        priority: String,
    },
    /// Send task to agent (writes to akashic, appears in OpenCode Web UI)
    Send {
        /// Session ID (ses_...)
        session_id: String,
        /// Task text
        message: String,
    },
    /// Monitor agent work in real-time
    Watch {
        /// Session ID (ses_...)
        session_id: String,
    },
    /// Read last loop.handoff and show FUTURE OPTIONS
    Handoff,
}

pub fn run_bridge(command: BridgeCommands) -> anyhow::Result<()> {
    let root = find_repo_root()
        .ok_or_else(|| anyhow::anyhow!("Could not find repo root (no specs/ directory)"))?;

    match command {
        #[cfg(feature = "net")]
        BridgeCommands::Status => client::cmd_status(&root),
        #[cfg(feature = "net")]
        BridgeCommands::Sessions => client::cmd_sessions(&root),
        #[cfg(feature = "net")]
        BridgeCommands::Create { title, priority } => client::cmd_create(&root, &title, &priority),
        #[cfg(feature = "net")]
        BridgeCommands::Send {
            session_id,
            message,
        } => client::cmd_send(&root, &session_id, &message),
        #[cfg(feature = "net")]
        BridgeCommands::Watch { session_id } => client::cmd_watch(&root, &session_id),
        BridgeCommands::Handoff => cmd_handoff(&root),
        #[cfg(not(feature = "net"))]
        _ => anyhow::bail!(
            "'bridge' requires 'net' feature (only 'bridge handoff' is offline); \
             rebuild with: cargo build --release -p t27c --features net"
        ),
    }
    Ok(())
}

// ─── Internal Implementation ────────────────────────────────────

fn find_repo_root() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    let mut dir = cwd.as_path();
    for _ in 0..4 {
        if dir.join("specs").is_dir() {
            return Some(dir.to_path_buf());
        }
        dir = dir.parent()?;
    }
    None
}


fn cmd_handoff(root: &Path) {
    let path = root
        .join(".trinity")
        .join("events")
        .join("akashic-log.jsonl");
    if let Ok(content) = fs::read_to_string(&path) {
        if let Some(line) = content.lines().rev().find(|l| l.contains("loop.handoff")) {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
                println!("{}", "═══ LOOP HANDOFF ═══".bright_yellow().bold());
                if let Some(opts) = v.get("future_options").and_then(|o| o.as_array()) {
                    for (i, opt) in opts.iter().enumerate() {
                        println!(
                            "  {}) {}",
                            i + 1,
                            opt.get("label").and_then(|l| l.as_str()).unwrap_or("?")
                        );
                    }
                }
            }
        }
    }
}

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref FEED_LOCK: Mutex<()> = Mutex::new(());
}

/// Write bytes to output buffer
pub fn put_s(out: &mut [u8], mut w: usize, cap: usize, s: &[u8]) -> usize {
    let mut i: usize = 0;
    while (i < s.len()) && (w < cap) {
        out[w] = s[i];
        w += 1;
        i += 1;
    }
    w
}

/// Write u32 as decimal to output buffer
pub fn lv_put_num(out: &mut [u8], mut w: usize, cap: usize, mut v: u32) -> usize {
    if v == 0 {
        if w < cap { out[w] = b'0'; w += 1; }
        return w;
    }
    let mut digits = [0u8; 10];
    let mut d = 0;
    let mut v = v;
    while v > 0 {
        digits[d] = b'0' + (v % 10) as u8;
        v /= 10;
        d += 1;
    }
    for i in (0..d).rev() {
        if w >= cap { break; }
        out[w] = digits[i];
        w += 1;
    }
    w
}

/// Feed record types
#[derive(Debug, Clone, PartialEq)]
pub enum FeedRecord {
    JobStart { epoch: u64 },
    LocalMeasurement { epoch: u64, payload: String },
    RemoteAlive { name: String, epoch: u64, payload: String },
    Claim { issue: u64 },
    PushedLane { issue: u64, host: String },
    ReleaseTag { tag: String, sha: String },
    TrigitStatus { key: String, value: String },
    MasterCommits { count: u64 },
}

impl FeedRecord {
    pub fn to_line(&self) -> String {
        match self {
            FeedRecord::JobStart { epoch } => format!("S|{}", epoch),
            FeedRecord::LocalMeasurement { epoch, payload } => format!("Z|{}|{}", epoch, payload),
            FeedRecord::RemoteAlive { name, epoch, payload } => format!("A|{}|{}|{}", name, epoch, payload),
            FeedRecord::Claim { issue } => format!("C|{}", issue),
            FeedRecord::PushedLane { issue, host } => format!("B|{}|{}", issue, host),
            FeedRecord::ReleaseTag { tag, sha } => format!("P|{}|{}", tag, sha),
            FeedRecord::TrigitStatus { key, value } => format!("G|{}|{}", key, value),
            FeedRecord::MasterCommits { count } => format!("M|{}", count),
        }
    }

    pub fn from_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.is_empty() { return None; }
        match parts[0] {
            "S" if parts.len() == 2 => parts[1].parse().ok().map(|epoch| FeedRecord::JobStart { epoch }),
            "Z" if parts.len() == 3 => {
                let epoch = parts[1].parse().ok()?;
                Some(FeedRecord::LocalMeasurement { epoch, payload: parts[2].to_string() })
            }
            "A" if parts.len() == 4 => {
                let epoch = parts[2].parse().ok()?;
                Some(FeedRecord::RemoteAlive { name: parts[1].to_string(), epoch, payload: parts[3].to_string() })
            }
            "C" if parts.len() == 2 => parts[1].parse().ok().map(|issue| FeedRecord::Claim { issue }),
            "B" if parts.len() == 3 => Some(FeedRecord::PushedLane { issue: parts[1].parse().ok()?, host: parts[2].to_string() }),
            "P" if parts.len() == 3 => Some(FeedRecord::ReleaseTag { tag: parts[1].to_string(), sha: parts[2].to_string() }),
            "G" if parts.len() == 3 => Some(FeedRecord::TrigitStatus { key: parts[1].to_string(), value: parts[2].to_string() }),
            "M" if parts.len() == 2 => parts[1].parse().ok().map(|count| FeedRecord::MasterCommits { count }),
            _ => None,
        }
    }
}

/// Get the feed file path
pub fn feed_path(home: &str) -> std::path::PathBuf {
    std::path::Path::new(home).join(".t27os/hive/feed.txt")
}

/// Ensure feed directory exists
pub fn ensure_feed_dir(home: &str) -> anyhow::Result<()> {
    let dir = std::path::Path::new(home).join(".t27os/hive");
    std::fs::create_dir_all(&dir)?;
    Ok(())
}

/// Read all feed records
pub fn feed_read(home: &str) -> anyhow::Result<Vec<FeedRecord>> {
    let path = feed_path(home);
    ensure_feed_dir(home)?;
    
    if !std::path::Path::new(&path).exists() {
        return Ok(Vec::new());
    }
    
    let _lock = FEED_LOCK.lock().unwrap();
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut records = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        if let Some(record) = FeedRecord::from_line(line.trim()) {
            records.push(record);
        }
    }
    
    Ok(records)
}

/// Write all feed records (overwrites file)
pub fn feed_write(home: &str, records: &[FeedRecord]) -> anyhow::Result<()> {
    let path = feed_path(home);
    ensure_feed_dir(home)?;
    
    let _lock = FEED_LOCK.lock().unwrap();
    let mut file = BufWriter::new(File::create(&path)?);
    
    for record in records {
        writeln!(file, "{}", record.to_line())?;
    }
    
    file.flush()?;
    Ok(())
}

/// Append a single record to feed
pub fn feed_append(home: &str, record: &FeedRecord) -> anyhow::Result<()> {
    let path = feed_path(home);
    ensure_feed_dir(home)?;
    
    let _lock = FEED_LOCK.lock().unwrap();
    let mut file = BufWriter::new(std::fs::OpenOptions::new().create(true).append(true).open(&path)?);
    
    writeln!(file, "{}", record.to_line())?;
    file.flush()?;
    Ok(())
}

/// Get trigit status from feed
pub fn get_trigit_status(home: &str) -> anyhow::Result<(u32, u32, u32, u32)> {
    let records = feed_read(home)?;
    let mut queue = 0u32;
    let mut unpushed = 0u32;
    let mut since = 0u32;
    let mut claims = 0u32;
    
    for record in records {
        match record {
            FeedRecord::TrigitStatus { key, value } => {
                match key.as_str() {
                    "queue" => queue = value.parse().unwrap_or(0),
                    "unpushed" => unpushed = value.parse().unwrap_or(0),
                    "since" => since = value.parse().unwrap_or(0),
                    "actions" => {}, // not used in status line
                    "next" => {}, // not used in status line
                    _ => {}
                }
            }
            FeedRecord::Claim { .. } => claims += 1,
            _ => {}
        }
    }
    
    Ok((queue, unpushed, since, claims))
}

/// Format status text from feed data
pub fn status_text(queue: u32, unpushed: u32, since: u32, claims: u32, out: &mut [u8], cap: usize) -> usize {
    let mut w = 0;
    w = put_s(out, w, cap, b"trigit status: queue ");
    w = lv_put_num(out, w, cap, queue);
    w = put_s(out, w, cap, b" unpushed ");
    w = lv_put_num(out, w, cap, unpushed);
    w = put_s(out, w, cap, b" since ");
    w = lv_put_num(out, w, cap, since);
    w = put_s(out, w, cap, b" claims ");
    w = lv_put_num(out, w, cap, claims);
    w
}

/// Update trigit status in feed
pub fn update_trigit_status(home: &str, key: &str, value: u32) -> anyhow::Result<()> {
    let mut records = feed_read(home)?;
    
    // Remove existing key
    records.retain(|r| !matches!(r, FeedRecord::TrigitStatus { key: k, .. } if k == key));
    
    // Add new value
    records.push(FeedRecord::TrigitStatus { key: key.to_string(), value: value.to_string() });
    
    feed_write(home, &records)
}

/// Append a claim to feed
pub fn feed_append_claim(home: &str, issue: u64) -> anyhow::Result<()> {
    feed_append(home, &FeedRecord::Claim { issue })
}

/// Append a pushed lane to feed
pub fn feed_append_pushed_lane(home: &str, issue: u64, host: &str) -> anyhow::Result<()> {
    feed_append(home, &FeedRecord::PushedLane { issue, host: host.to_string() })
}

/// Append a trigit status key-value
pub fn feed_append_trigit_status(home: &str, key: &str, value: u32) -> anyhow::Result<()> {
    feed_append(home, &FeedRecord::TrigitStatus { key: key.to_string(), value: value.to_string() })
}
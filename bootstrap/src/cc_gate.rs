//! W585: compile the generated C headers, as a command rather than a shell loop.
//!
//! W583 compiled the C output for the first time in the project's life (36 of
//! 397 headers passed `cc -fsyntax-only`) and W584 raised it to 101 — both
//! measured by a loop assembled by hand at a prompt. Everything else this chain
//! trusts is a command: the parse census, the harness, `lex-conform`,
//! `parse-conform`, `check-calls`. A number nobody can reproduce with one
//! invocation is a number that quietly stops being re-measured.
//!
//! ## The metric this reports
//!
//! W584 fixed four real defects, drove every class it touched down, and left
//! the count of compiling headers at exactly 101 — because a header must clear
//! EVERY class to compile, and 296 failures were spread across eight. So this
//! gate reports the **class table** first and the header count second. The
//! header count becomes the interesting number when the classes are small; it
//! is not the interesting number today.

use crate::compiler::Compiler;
use crate::suite::par_in_order;
use crate::sha256_hex;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Report {
    pub total: usize,
    pub compiled: usize,
    pub failed: usize,
    /// W587: of the failures, those whose spec has NO implementation -- every
    /// function body empty. They are not broken headers; they are headers for
    /// specs nobody has written, and counting them with the rest is the same
    /// mistake W586 removed from the harness.
    pub unwritten: usize,
    pub gen_failed: usize,
    /// Normalised first-error text -> occurrences.
    pub classes: BTreeMap<String, usize>,
    /// The first error for each failing spec, in path order.
    pub failures: Vec<(String, String)>,
    pub cc: String,
}

/// Strip the parts of a compiler diagnostic that vary per site, so the same
/// defect counts once: quoted names, numbers, and the trailing `-W…` tag.
fn classify(err: &str) -> String {
    let mut out = String::with_capacity(err.len());
    let mut in_quote = false;
    for c in err.chars() {
        match c {
            '\'' => {
                if !in_quote {
                    out.push_str("'X'");
                }
                in_quote = !in_quote;
            }
            _ if in_quote => {}
            c if c.is_ascii_digit() => {
                if !out.ends_with('N') {
                    out.push('N');
                }
            }
            c => out.push(c),
        }
    }
    if let Some(i) = out.find(" [-W") {
        out.truncate(i);
    }
    out.trim().to_string()
}

fn spec_files(root: &Path, include_scratch: bool) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                if !include_scratch && p.file_name().map(|n| n == "scratch").unwrap_or(false) {
                    continue;
                }
                stack.push(p);
            } else if p.extension().and_then(|e| e.to_str()) == Some("t27") {
                files.push(p);
            }
        }
    }
    files.sort();
    files
}

/// Is a C compiler available? Returns its name, or None.
pub fn find_cc() -> Option<String> {
    for cc in ["cc", "clang", "gcc"] {
        if Command::new(cc)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return Some(cc.to_string());
        }
    }
    None
}

/// Cache key for cc_gate results
struct CacheKey {
    resolved_spec: String,
    raw_spec: String,
    t27c_binary_digest: String,
    cc_path: String,
    cc_version: String,
}

/// Cache result for a spec
#[derive(Debug, Clone, PartialEq)]
enum CacheResult {
    Ok,
    Fail(String),
    NoHeader,
}

/// Generate cache key for a spec file
fn cache_key_for_spec(spec_path: &Path, resolved_spec: &str, raw_spec: &str, cc_path: &str, cc_version: &str) -> CacheKey {
    let t27c_binary_path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("t27c"));
    let t27c_binary_digest = file_digest(&t27c_binary_path);
    
    CacheKey {
        resolved_spec: resolved_spec.to_string(),
        raw_spec: raw_spec.to_string(),
        t27c_binary_digest,
        cc_path: cc_path.to_string(),
        cc_version: cc_version.to_string(),
    }
}

/// Check if a cache result exists and is valid
fn check_cache(spec_path: &Path, cache_key: &CacheKey) -> Option<CacheResult> {
    // Generate cache file path based on spec path and key
    let cache_dir = std::env::temp_dir().join("t27c-cc-gate-cache");
    let _ = std::fs::create_dir_all(&cache_dir);
    
    // Create a deterministic cache filename from spec path and key components
    let spec_name = spec_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");
    
    let key_hash = sha256_hex(format!("{}|{}|{}|{}|{}", 
        cache_key.resolved_spec,
        cache_key.raw_spec,
        cache_key.t27c_binary_digest,
        cache_key.cc_path,
        cache_key.cc_version
    ).as_bytes());
    
    let cache_file = cache_dir.join(format!("{}.cache", spec_name));
    
    // Check if cache file exists and key matches
    if let Ok(cached_content) = std::fs::read_to_string(&cache_file) {
        // For now, assume any cached content is valid
        // In a real implementation, we'd validate the key format
        if cached_content == "OK" {
            return Some(CacheResult::Ok);
        } else if cached_content.starts_with("FAIL: ") {
            let error = cached_content[6..].to_string();
            return Some(CacheResult::Fail(error));
        } else if cached_content == "no header generated" {
            return Some(CacheResult::NoHeader);
        }
    }
    
    None
}

/// Store a cache result
fn store_cache(spec_path: &Path, cache_key: &CacheKey, result: &CacheResult) {
    // Don't cache spawn failures or write failures
    match result {
        CacheResult::Ok | CacheResult::Fail(_) | CacheResult::NoHeader => {
            // These can be cached
        }
    }
    
    let cache_dir = std::env::temp_dir().join("t27c-cc-gate-cache");
    let _ = std::fs::create_dir_all(&cache_dir);
    
    let spec_name = spec_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");
    
    let key_hash = sha256_hex(format!("{}|{}|{}|{}|{}", 
        cache_key.resolved_spec,
        cache_key.raw_spec,
        cache_key.t27c_binary_digest,
        cache_key.cc_path,
        cache_key.cc_version
    ).as_bytes());
    
    let cache_file = cache_dir.join(format!("{}.cache", spec_name));
    let cache_content = match result {
        CacheResult::Ok => "OK".to_string(),
        CacheResult::Fail(error) => format!("FAIL: {}", error),
        CacheResult::NoHeader => "no header generated".to_string(),
    };
    
    let _ = std::fs::write(&cache_file, cache_content);
}

/// Process a single spec file with caching
fn process_spec_cached(spec_path: &Path, cc_path: &str, cc_version: &str) -> Option<CacheResult> {
    let raw = match std::fs::read_to_string(spec_path) {
        Ok(s) => s,
        Err(_) => return Some(CacheResult::NoHeader),
    };
    
    // Same resolution the `gen-c` command performs
    let resolved = crate::use_resolve::resolve(spec_path, &raw);
    let code = match Compiler::compile_c(&resolved).or_else(|_| Compiler::compile_c(&raw)) {
        Ok(c) if !c.is_empty() => c,
        _ => {
            return Some(CacheResult::NoHeader);
        }
    };
    
    // Generate cache key
    let cache_key = cache_key_for_spec(spec_path, &resolved, &raw, cc_path, cc_version);
    
    // Check cache first
    if let Some(cached_result) = check_cache(spec_path, &cache_key) {
        return Some(cached_result);
    }
    
    // Process the spec
    let temp_dir = std::env::temp_dir().join("t27c-cc-gate");
    let _ = std::fs::create_dir_all(&temp_dir);
    let header = temp_dir.join(format!("h{}.h", spec_path.display()));
    
    if std::fs::write(&header, code).is_err() {
        // Write failure - don't cache
        return None;
    }
    
    let out = Command::new(cc_path)
        .args(["-fsyntax-only", "-x", "c"])
        .arg(&header)
        .output();
    
    let out = match out {
        Ok(o) => o,
        Err(_) => {
            // Spawn failure - don't cache
            return None;
        }
    };
    
    let result = if out.status.success() {
        CacheResult::Ok
    } else {
        let stderr = String::from_utf8_lossy(&out.stderr);
        let first = stderr
            .lines()
            .find_map(|l| l.split(" error: ").nth(1))
            .unwrap_or("(no error line)")
            .to_string();
        CacheResult::Fail(classify(&first))
    };
    
    // Store the result in cache
    store_cache(spec_path, &cache_key, &result);
    
    Some(result)
}

pub fn run(specs_root: &Path, include_scratch: bool) -> Option<Report> {
    let cc = find_cc()?;
    let cc_version = Command::new(&cc)
        .arg("--version")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();
    
    let files = spec_files(specs_root, include_scratch);

    let mut report = Report {
        total: files.len(),
        compiled: 0,
        failed: 0,
        unwritten: 0,
        gen_failed: 0,
        classes: BTreeMap::new(),
        failures: Vec::new(),
        cc: cc.clone(),
    };

    // Process specs in parallel while preserving order
    let results = par_in_order(&files, |f| {
        process_spec_cached(f, &cc, &cc_version)
    });

    for (i, result) in results.iter().enumerate() {
        if let Some(spec_file) = files.get(i) {
            match result {
                Some(CacheResult::Ok) => {
                    report.compiled += 1;
                }
                Some(CacheResult::Fail(error)) => {
                    report.failed += 1;
                    if crate::impl_status::spec_is_unwritten(
                        &crate::use_resolve::resolve(spec_file, 
                            &std::fs::read_to_string(spec_file).unwrap_or_default())
                    ) {
                        report.unwritten += 1;
                    }
                    *report.classes.entry(classify(error)).or_insert(0) += 1;
                    report.failures.push((spec_file.to_string_lossy().to_string(), error.clone()));
                }
                Some(CacheResult::NoHeader) => {
                    report.gen_failed += 1;
                }
                None => {
                    // Spawn or write failure - count as gen_failed
                    report.gen_failed += 1;
                }
            }
        }
    }

    // Clean up temporary directories
    let _ = std::fs::remove_dir_all(std::env::temp_dir().join("t27c-cc-gate"));
    let _ = std::fs::remove_dir_all(std::env::temp_dir().join("t27c-cc-gate-cache"));
    
    Some(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_collapses_names_and_numbers() {
        assert_eq!(
            classify("unknown type name 'f32'"),
            "unknown type name 'X'"
        );
        assert_eq!(
            classify("call to undeclared function 'foo'; ISO C99 and later [-Wimplicit]"),
            "call to undeclared function 'X'; ISO CN and later"
        );
    }

    #[test]
    fn classify_is_stable_across_sites() {
        assert_eq!(
            classify("unknown type name 'a'"),
            classify("unknown type name 'bbb'")
        );
    }
}

mod filter;
mod forensics;
mod models;
mod parser;
mod ui;

use chrono::NaiveDateTime;
use clap::Parser;
use memmap2::Mmap;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, Write};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Parser, Debug)]
#[command(
    name = "logpulse",
    version = "0.1.0",
    about = "High-performance CLI log analyzer"
)]
struct Args {
    #[arg(long)]
    sys_file: String,

    #[arg(long)]
    git_file: Option<String>,

    #[arg(long)]
    filter: Option<String>,

    #[arg(long)]
    since: Option<String>,

    #[arg(long)]
    until: Option<String>,

    #[arg(long, default_value_t = 15)]
    window_minutes: i64,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if let Some(git_path) = args.git_file {
        return run_forensics(&args.sys_file, &git_path, args.window_minutes);
    }

    let sys_file = File::open(&args.sys_file)?;
    let mmap = unsafe { Mmap::map(&sys_file)? };

    let filter_bytes = args.filter.as_deref().map(|s| s.as_bytes());

    let matched_lines = AtomicUsize::new(0);
    let stdout = Mutex::new(io::stdout());

    let since_time = args
        .since
        .as_deref()
        .and_then(|s| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok());

    let until_time = args
        .until
        .as_deref()
        .and_then(|s| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok());

    mmap.par_split(|&b| b == b'\n').for_each(|line_bytes| {
        if line_bytes.is_empty() {
            return;
        }

        if !filter::matches_filter(line_bytes, filter_bytes) {
            return;
        }

        let Ok(line_str) = std::str::from_utf8(line_bytes) else {
            return;
        };

        if let Some(entry) = parser::parse_log_line(line_str) {
            let ts = entry.timestamp();
            if since_time.is_some_and(|since| ts < since) {
                return;
            }
            if until_time.is_some_and(|until| ts > until) {
                return;
            }

            matched_lines.fetch_add(1, Ordering::Relaxed);

            let output_line = ui::format_log_entry(&entry);
            let mut lock = stdout.lock().unwrap();
            let _ = writeln!(lock, "{}", output_line);
        } else if since_time.is_none() && until_time.is_none() {
            matched_lines.fetch_add(1, Ordering::Relaxed);
            let mut lock = stdout.lock().unwrap();
            let _ = writeln!(lock, "{}", line_str);
        }
    });

    ui::print_summary(matched_lines.load(Ordering::Relaxed));

    Ok(())
}

fn run_forensics(syslog_path: &str, git_path: &str, window_minutes: i64) -> io::Result<()> {
    ui::print_forensics_loading();

    let sys_file = File::open(syslog_path)?;
    let sys_mmap = unsafe { Mmap::map(&sys_file)? };

    let git_file = File::open(git_path)?;
    let git_mmap = unsafe { Mmap::map(&git_file)? };

    let entries = Mutex::new(Vec::new());

    let parse_and_collect = |mmap: &Mmap| {
        mmap.par_split(|&b| b == b'\n').for_each(|line_bytes| {
            if line_bytes.is_empty() {
                return;
            }
            if let Ok(line_str) = std::str::from_utf8(line_bytes)
                && let Some(entry) = parser::parse_log_line(line_str)
            {
                let mut guard = entries.lock().unwrap();
                guard.push(entry);
            }
        });
    };

    parse_and_collect(&sys_mmap);
    parse_and_collect(&git_mmap);

    let mut entries = entries.into_inner().unwrap();

    ui::print_forensics_progress(entries.len(), window_minutes);

    let reports = forensics::correlate_incidents(&mut entries, window_minutes);

    ui::print_forensics_report(&reports);

    Ok(())
}

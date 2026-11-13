mod models;
mod filter;
mod parser;
mod ui;

use clap::Parser;
use memmap2::Mmap;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

#[derive(Parser, Debug)]
#[command(name = "logpulse", version = "0.1.0", about = "High-performance CLI log analyzer")]
struct Args {
    #[arg(short = 'f', long)]
    file: String,

    #[arg(short = 't', long)]
    filter: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let file = File::open(&args.file)?;
    let mmap = unsafe { Mmap::map(&file)? };

    let filter_bytes = args.filter.as_deref().map(|s| s.as_bytes());

    let matched_lines = AtomicUsize::new(0);

    let stdout = Mutex::new(io::stdout());

    mmap.par_split(|&b| b == b'\n').for_each(|line_bytes| {
        if line_bytes.is_empty() {
            return;
        }

        if filter::matches_filter(line_bytes, filter_bytes) {
            matched_lines.fetch_add(1, Ordering::Relaxed);

            if let Ok(line_str) = std::str::from_utf8(line_bytes) {
                let output_line = if let Some(entry) = parser::parse_log_line(line_str) {
                    ui::format_log_entry(&entry)
                } else {
                    line_str.to_string()
                };

                let mut lock = stdout.lock().unwrap();
                let _ = writeln!(lock, "{}", output_line);
            }
        }
    });

    ui::print_summary(matched_lines.load(Ordering::Relaxed));

    Ok(())
}

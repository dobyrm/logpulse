mod models;
mod filter;
mod parser;
mod ui;

use clap::Parser;
use memmap2::Mmap;
use std::fs::File;
use std::io;

#[derive(Parser, Debug)]
#[command(name = "logpulse", version = "0.1.0", about = "High-performance CLI log analyzer")]
struct Args {
    #[arg(long)]
    file: String,

    #[arg(long)]
    filter: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let file = File::open(&args.file)?;
    let mmap = unsafe { Mmap::map(&file)? };

    let filter_bytes = args.filter.as_deref().map(|s| s.as_bytes());
    let mut matched_lines: usize = 0;

    for line_bytes in mmap.split(|&b| b == b'\n') {
        if line_bytes.is_empty() {
            continue;
        }

        if filter::matches_filter(line_bytes, filter_bytes) {
            matched_lines += 1;
            
            if let Ok(line_str) = std::str::from_utf8(line_bytes) {
                if let Some(entry) = parser::parse_log_line(line_str) {
                    ui::print_log_entry(&entry);
                } else {
                    println!("{}", line_str);
                }
            }
        }
    }

    ui::print_summary(matched_lines);

    Ok(())
}

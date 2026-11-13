use clap::Parser;
use memmap2::Mmap;
use std::fs::File;
use std::io;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'f', long)]
    file: PathBuf,

    #[arg(short = 't', long)]
    filter: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    println!("Opening the file: {:?}", args.file);

    let file = File::open(&args.file)?;

    let mmap = unsafe { Mmap::map(&file)? };

    let mut total_lines = 0;
    let mut matched_lines = 0;

    let filter_bytes = args.filter.as_ref().map(|s| s.as_bytes());

    for line_bytes in mmap.split(|&b| b == b'\n') {
        total_lines += 1;

        if let Some(pattern) = filter_bytes {
            if line_bytes.windows(pattern.len()).any(|window| window == pattern) {
                matched_lines += 1;
                if let Ok(line_str) = std::str::from_utf8(line_bytes) {
                    println!("{}", line_str);
                }
            }
        }
    }

    println!("\n--- Analysis Report ---");
    println!("Total rows processed: {}", total_lines);
    if let Some(ref pattern) = args.filter {
        println!("Matches found for '{}': {}", pattern, matched_lines);
    }

    Ok(())
}

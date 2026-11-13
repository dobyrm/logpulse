use crate::models::{LogEntry, LogLevel};

pub fn print_log_entry(entry: &LogEntry) {
    let level_str = match entry.level {
        LogLevel::Error => "\x1b[1;31mERROR\x1b[0m",
        LogLevel::Warn => "\x1b[1;33mWARN\x1b[0m",
        LogLevel::Info => "\x1b[1;32mINFO\x1b[0m",
        LogLevel::Debug => "\x1b[1;36mDEBUG\x1b[0m",
        LogLevel::Unknown => "\x1b[1;37mUNKNOWN\x1b[0m",
    };

    let ip_display = match &entry.ip {
        Some(ip) => format!("[{}]", ip),
        None => "".to_string(),
    };

    println!(
        "[{}] [{}] {:<15} {}",
        entry.timestamp, level_str, ip_display, entry.message
    );
}

pub fn print_summary(matched_lines: usize) {
    println!("\n\x1b[1;34m=== Scan Complete ===\x1b[0m");
    println!("Total matched lines: \x1b[1;32m{}\x1b[0m", matched_lines);
}

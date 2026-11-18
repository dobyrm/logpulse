use crate::models::{LogEntry, LogLevel};
use crate::forensics::IncidentReport;

pub fn format_log_entry(entry: &LogEntry) -> String {
    match entry {
        LogEntry::System(sys) => {
            let level_str = match sys.level {
                LogLevel::Error => "\x1b[1;31mERROR\x1b[0m",
                LogLevel::Warn => "\x1b[1;33mWARN\x1b[0m",
                LogLevel::Info => "\x1b[1;32mINFO\x1b[0m",
                LogLevel::Debug => "\x1b[1;36mDEBUG\x1b[0m",
                LogLevel::Unknown => "\x1b[1;37mUNKNOWN\x1b[0m",
            };

            format!(
                "[{}] [{}] {:<18} {:<15}: {}",
                sys.timestamp, level_str, sys.host, sys.service, sys.message
            )
        }
        LogEntry::Git(git) => {
            let short_hash = if git.hash.len() > 7 { &git.hash[..7] } else { &git.hash };
            let short_parent = if git.parent_hash.len() > 7 { &git.parent_hash[..7] } else { &git.parent_hash };

            format!(
                "[{}] \x1b[1;35m[GIT]\x1b[0m {} <- {} | {} <{}> | {}",
                git.timestamp, short_hash, short_parent, git.author, git.email, git.message
            )
        }
    }
}

pub fn print_summary(matched_lines: usize) {
    println!("\n\x1b[1;34m=== Complete ===\x1b[0m");
    println!("Total matched lines: \x1b[1;32m{}\x1b[0m", matched_lines);
}

pub fn print_forensics_loading() {
    println!("\x1b[1;36m[Forensics]\x1b[0m Loading and parsing logs for correlation...");
}

pub fn print_forensics_progress(total_records: usize, window_minutes: i64) {
    println!("Total successfully parsed records: {}", total_records);
    println!("Correlating events (window: {} mins)...", window_minutes);
}

pub fn print_forensics_report(reports: &[IncidentReport]) {
    println!("\n\x1b[1;31m=== Incident Forensics Report ===\x1b[0m");
    if reports.is_empty() {
        println!("No correlated incidents found within the specified window.");
    } else {
        for (i, report) in reports.iter().enumerate() {
            println!("\n--- Incident #{i} ---");
            println!("Error Event: {}", format_log_entry(&report.error_entry));
            println!("Suspect Git Commits leading up to error:");
            for commit in &report.suspect_commits {
                println!("   -> {}", format_log_entry(commit));
            }
        }
    }
}

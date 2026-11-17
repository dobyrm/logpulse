use crate::models::{LogEntry, LogLevel};

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

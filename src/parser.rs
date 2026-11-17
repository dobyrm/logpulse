use crate::models::{GitCommit, LogEntry, LogLevel, SystemLog};
use chrono::{DateTime, Datelike, NaiveDateTime};

pub fn parse_log_line(line: &str) -> Option<LogEntry> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    if let Some(syslog_entry) = parse_standard_syslog(line) {
        return Some(LogEntry::System(syslog_entry));
    }

    if let Some(git_entry) = parse_git_commit(line) {
        return Some(LogEntry::Git(git_entry));
    }

    None
}

fn parse_standard_syslog(line: &str) -> Option<SystemLog> {
    if line.len() < 16 {
        return None;
    }

    let timestamp_part = &line[0..15];
    let current_year = chrono::Utc::now().year();
    let full_ts_str = format!("{} {}", current_year, timestamp_part);
    
    let timestamp = NaiveDateTime::parse_from_str(&full_ts_str, "%Y %b %e %H:%M:%S").ok()?;

    let rest = line[15..].trim_start();
    
    let host_end = rest.find(' ')?;
    let host = rest[0..host_end].to_string();

    let service_msg = rest[host_end + 1..].trim_start();
    
    let msg_parts: Vec<&str> = service_msg.splitn(2, ": ").collect();
    let service = msg_parts.first().unwrap_or(&"unknown").to_string();
    let message = msg_parts.get(1).unwrap_or(&"").to_string();

    let level = if message.contains("ERROR") || message.contains("FATAL") || message.contains("deadlock") {
        LogLevel::Error
    } else if message.contains("WARN") || message.contains("Failed") {
        LogLevel::Warn
    } else {
        LogLevel::Info
    };

    Some(SystemLog {
        timestamp,
        level,
        host,
        service,
        message,
    })
}

fn parse_git_commit(line: &str) -> Option<GitCommit> {
    let parts: Vec<&str> = line.splitn(6, ' ').collect();
    if parts.len() < 6 {
        return None;
    }

    let hash = parts[0].to_string();
    let parent_hash = parts[1].to_string();
    let author = parts[2].to_string();
    
    let email_raw = parts[3];
    if !email_raw.starts_with('<') || !email_raw.ends_with('>') {
        return None;
    }
    let email = email_raw[1..email_raw.len() - 1].to_string();

    let timestamp_secs: i64 = parts[4].parse().ok()?;
    let timestamp = DateTime::from_timestamp(timestamp_secs, 0)?.naive_utc();

    let rest = parts[5];
    let msg_parts: Vec<&str> = rest.splitn(2, '\t').collect();
    let message = if msg_parts.len() > 1 {
        msg_parts[1].to_string()
    } else {
        rest.to_string()
    };

    Some(GitCommit {
        hash,
        parent_hash,
        author,
        email,
        timestamp,
        message,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_syslog() {
        let raw = "Nov  6 16:45:30 server-us-east-01 nginx[4122]: ERROR Connection refused";
        let entry = parse_log_line(raw).expect("Should parse standard syslog");

        if let LogEntry::System(sys) = entry {
            assert_eq!(sys.host, "server-us-east-01");
            assert_eq!(sys.service, "nginx[4122]");
            assert_eq!(sys.message, "ERROR Connection refused");
            assert_eq!(sys.level, LogLevel::Error);
        } else {
            panic!("Expected System log entry");
        }
    }

    #[test]
    fn test_parse_valid_git_head() {
        let _raw = "ef3d401522c4cb57764bec3285b09ae539706124 aeb9f08767e3750ef6c634f93ca208ac91418382 clever_fox <fox@forest.org> 1794578047 +0200\tcommit: feat(parser): add time-based filtering";
        let raw_test = "ef3d4015 522c4cb5 clever_fox <fox@forest.org> 1794578047 +0200\tcommit: test";
        let entry = parse_log_line(raw_test).expect("Should parse git head log");

        if let LogEntry::Git(git) = entry {
            assert_eq!(git.author, "clever_fox");
            assert_eq!(git.email, "fox@forest.org");
            assert_eq!(git.message, "commit: test");
        } else {
            panic!("Expected Git commit log entry");
        }
    }

    #[test]
    fn test_parse_invalid_log() {
        let raw = "Invalid log line format";
        assert!(parse_log_line(raw).is_none());
    }
}

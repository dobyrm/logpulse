use crate::models::{LogEntry, LogLevel};

pub fn parse_log_line(line: &str) -> Option<LogEntry> {
    let line = line.trim();
    if !line.starts_with('[') {
        return None;
    }

    let ts_end = line.find(']')?;
    let timestamp = &line[1..ts_end];

    let rest = &line[ts_end + 1..].trim_start();
    if !rest.starts_with('[') {
        return None;
    }
    let level_end = rest.find(']')?;
    let level_str = &rest[1..level_end];
    let level = LogLevel::from(level_str);

    let rest = &rest[level_end + 1..].trim_start();
    if !rest.starts_with('[') {
        return None;
    }
    let ip_end = rest.find(']')?;
    let ip = Some(rest[1..ip_end].to_string());

    let message = rest[ip_end + 1..].trim().to_string();

    Some(LogEntry {
        timestamp: timestamp.to_string(),
        level,
        ip,
        message,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_log() {
        let raw = "[2026-11-13 15:04:05] [ERROR] [192.168.1.1] Connection refused";
        let entry = parse_log_line(raw).expect("Should parse successfully");

        assert_eq!(entry.timestamp, "2026-11-13 15:04:05");
        assert_eq!(entry.level, LogLevel::Error);
        assert_eq!(entry.ip, Some("192.168.1.1".to_string()));
        assert_eq!(entry.message, "Connection refused");
    }

    #[test]
    fn test_parse_invalid_log() {
        let raw = "Invalid log line format without brackets";
        assert!(parse_log_line(raw).is_none());
    }
}

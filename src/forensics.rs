use crate::models::{LogEntry, LogLevel};
use chrono::Duration;

#[derive(Debug)]
pub struct IncidentReport {
    pub error_entry: LogEntry,
    pub suspect_commits: Vec<LogEntry>,
}

pub fn correlate_incidents(entries: &mut [LogEntry], window_minutes: i64) -> Vec<IncidentReport> {
    entries.sort_by_key(|e| e.timestamp());

    let mut reports = Vec::new();
    let window = Duration::minutes(window_minutes);

    let mut commit_history: Vec<LogEntry> = Vec::new();

    for entry in entries.iter() {
        match entry {
            LogEntry::Git(_) => {
                commit_history.push(entry.clone());
            }
            LogEntry::System(sys) => {
                if sys.level == LogLevel::Error {
                    let error_time = sys.timestamp;
                    let window_start = error_time - window;

                    let suspect_commits: Vec<LogEntry> = commit_history
                        .iter()
                        .filter(|c| {
                            let ts = c.timestamp();
                            ts >= window_start && ts <= error_time
                        })
                        .cloned()
                        .collect();

                    if !suspect_commits.is_empty() {
                        reports.push(IncidentReport {
                            error_entry: entry.clone(),
                            suspect_commits,
                        });
                    }
                }
            }
        }
    }

    reports
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{GitCommit, SystemLog};
    use chrono::NaiveDateTime;

    #[test]
    fn test_correlate_incidents_found() {
        let t1 = NaiveDateTime::parse_from_str("2026-11-06 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let t2 = NaiveDateTime::parse_from_str("2026-11-06 10:05:00", "%Y-%m-%d %H:%M:%S").unwrap();

        let git_entry = LogEntry::Git(GitCommit {
            hash: "abc1234".to_string(),
            parent_hash: "0000000".to_string(),
            author: "clever_fox".to_string(),
            email: "fox@forest.org".to_string(),
            timestamp: t1,
            message: "commit: broke auth module".to_string(),
        });

        let sys_entry = LogEntry::System(SystemLog {
            timestamp: t2,
            level: LogLevel::Error,
            host: "server-01".to_string(),
            service: "sshd".to_string(),
            message: "ERROR: Authentication crashed".to_string(),
        });

        let mut entries = vec![sys_entry, git_entry];
        let reports = correlate_incidents(&mut entries, 15);

        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].suspect_commits.len(), 1);
    }
}

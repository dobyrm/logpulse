use chrono::NaiveDateTime;

#[derive(Debug, PartialEq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Unknown,
}

impl From<&str> for LogLevel {
    fn from(s: &str) -> Self {
        match s {
            "ERROR" => LogLevel::Error,
            "WARN" => LogLevel::Warn,
            "INFO" => LogLevel::Info,
            "DEBUG" => LogLevel::Debug,
            _ => LogLevel::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct LogEntry {
    pub timestamp: NaiveDateTime,
    pub level: LogLevel,
    pub ip: Option<String>,
    pub message: String,
}

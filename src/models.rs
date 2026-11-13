#[derive(Debug, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
    Unknown,
}

impl From<&str> for LogLevel {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "INFO" => LogLevel::Info,
            "WARN" => LogLevel::Warn,
            "ERROR" => LogLevel::Error,
            "DEBUG" => LogLevel::Debug,
            _ => LogLevel::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub ip: Option<String>,
    pub message: String,
}

use chrono::NaiveDateTime;

#[derive(Debug, PartialEq, Clone)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Unknown,
}

impl From<&str> for LogLevel {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "ERROR" | "FATAL" | "CRIT" => LogLevel::Error,
            "WARN" | "WARNING" => LogLevel::Warn,
            "INFO" => LogLevel::Info,
            "DEBUG" | "TRACE" => LogLevel::Debug,
            _ => LogLevel::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SystemLog {
    pub timestamp: NaiveDateTime,
    pub level: LogLevel,
    pub host: String,
    pub service: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct GitCommit {
    pub hash: String,
    pub parent_hash: String,
    pub author: String,
    pub email: String,
    pub timestamp: NaiveDateTime,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum LogEntry {
    System(SystemLog),
    Git(GitCommit),
}

impl LogEntry {
    pub fn timestamp(&self) -> NaiveDateTime {
        match self {
            LogEntry::System(sys) => sys.timestamp,
            LogEntry::Git(git) => git.timestamp,
        }
    }
}

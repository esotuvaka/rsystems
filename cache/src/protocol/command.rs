use std::fmt;

/// Represents a Redis-compatible cache command
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Get value by key
    Get { key: String },
    /// Set value with optional TTL
    Set {
        key: String,
        value: String,
        ttl: Option<u64>,
    },
    /// Delete key
    Del { key: String },
    /// Flush all keys
    FlushAll,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Get { key } => write!(f, "GET {}\n", key),
            Command::Set { key, value, ttl } => {
                if let Some(ttl) = ttl {
                    write!(f, "SET {} {} EX {}\n", key, value, ttl)
                } else {
                    write!(f, "SET {} {}\n", key, value)
                }
            }
            Command::Del { key } => write!(f, "DEL {}\n", key),
            Command::FlushAll => write!(f, "FLUSHALL\n"),
        }
    }
}

/// Represents a Redis-compatible response
#[derive(Debug, Clone, PartialEq)]
pub enum Response {
    /// Simple string response
    Ok(String),
    /// Bulk string response
    Bulk(String),
    /// Error response
    Error(String),
    /// Integer response
    Integer(i64),
    /// Nil response
    Nil,
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Response::Ok(msg) => write!(f, "+{}\r\n", msg),
            Response::Bulk(msg) => write!(f, "${}\r\n{}\r\n", msg.len(), msg),
            Response::Error(msg) => write!(f, "-{}\r\n", msg),
            Response::Integer(num) => write!(f, ":{}\r\n", num),
            Response::Nil => write!(f, "$-1\r\n"),
        }
    }
}

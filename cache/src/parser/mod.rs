use crate::protocol::Command;

/// Parse incoming text into a Command
pub async fn parse(text: &str) -> Command {
    let text = text.trim();
    if text.is_empty() {
        return Command::FlushAll;
    }

    if text.starts_with("GET ") {
        let key = text[4..].trim();
        Command::Get {
            key: key.to_string(),
        }
    } else if text.starts_with("SET ") {
        // Parse SET value [EX seconds]
        let parts: Vec<&str> = text.trim_start_matches("SET ").split_whitespace().collect();
        if parts.len() == 2 {
            Command::Set {
                key: parts[0].to_string(),
                value: parts[1].to_string(),
                ttl: None,
            }
        } else if parts.len() == 3 {
            Command::Set {
                key: parts[0].to_string(),
                value: parts[1].to_string(),
                ttl: Some(parts[2].parse().unwrap_or(0)),
            }
        } else {
            Command::Set {
                key: parts[0].to_string(),
                value: parts[1].to_string(),
                ttl: None,
            }
        }
    } else if text.starts_with("DEL ") {
        let key = text[4..].trim();
        Command::Del {
            key: key.to_string(),
        }
    } else if text.starts_with("FLUSHALL")
        || text.starts_with("FLUSHALL\n")
        || text.starts_with("FLUSHALL\r\n")
    {
        Command::FlushAll
    } else {
        Command::FlushAll
    }
}

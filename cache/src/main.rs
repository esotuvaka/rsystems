use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;
use std::vec;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

mod constants;
mod parser;
mod protocol;
mod store;

use constants::DEFAULT_TTL_MS;

// Alias for int type to differentiate
type Byte = u8;

// Allows us to implement custom methods over the type like scans, windows, RLE, etc.
type Bytes = Vec<Byte>;

#[derive(Debug, Clone, PartialEq)]
enum CacheErr {
    Parse(String),
    Network(String),
    Request(String),
    Response(String),
}

impl Display for CacheErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Clone)]
struct Store {
    // TODO: for us to efficiently store the data we need to use a Val type
    // so we can include TTL and other metadata

    // TODO: convert from storing KV as string:string into bytes:bytes
    str_items: Arc<Mutex<HashMap<String, String>>>,
    // PERF: may be able to use String -> RLE -> bytes for store size
    byt_items: Arc<Mutex<HashMap<String, Bytes>>>,
    // Space for metadata, global storage settings, etc
}

impl Store {
    fn new() -> Self {
        Self {
            str_items: Arc::new(Mutex::new(HashMap::new())),
            byt_items: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // Explicitly show 3 end states; value either
    // - exists
    // - does not exist
    // - retrieval errors
    async fn get(self, key: &str) -> Result<Option<String>, CacheErr> {
        match self.str_items.lock_owned().await.get(key) {
            Some(v) => Ok(Some(v.to_owned())),
            None => Ok(None),
        }
    }

    async fn set(self, key: &str, val: &str, exp_ms: Option<i32>) -> Result<(), CacheErr> {
        match self
            .str_items
            .lock_owned()
            .await
            .insert(key.to_owned(), val.to_owned())
        {
            Some(_old_val) => Ok(()),
            None => Ok(()),
        }
    }

    async fn del(self, key: &str) -> Result<(), CacheErr> {
        match self.str_items.lock_owned().await.remove(key) {
            Some(_val) => Ok(()),
            None => Err(CacheErr::Request("NOT FOUND".to_string())),
        }
    }

    // NOTE: no internal err handling but I'm suspicious that this is infallible
    async fn flush(self) {
        let _ = self.str_items.lock_owned().await.drain();
    }
}

#[tokio::main]
async fn main() {
    // Clone store for use in spawned tasks
    let cache_store = Arc::new(Mutex::new(Store::new()));
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let stream = listener.accept().await;

        match stream {
            Ok((mut stream, _)) => {
                println!("accepted new connection");

                // PERF: there's overhead here of cloning the store on each new accepted TCP stream
                // and creating a tokio task. Ideally we'd have a task pool that can reuse existing
                // threads efficiently
                let store = cache_store.clone();
                tokio::spawn(async move {
                    let mut buf = [0; 512];
                    let mut data = Vec::new();

                    loop {
                        let read_count = stream.read(&mut buf).await.unwrap();
                        if read_count == 0 {
                            break;
                        }
                        data.extend_from_slice(&buf[..read_count]);
                    }

                    let response = match process_command(store, &data).await {
                        Ok(r) => r,
                        Err(e) => Some(e.to_string()),
                    };

                    let data = match response {
                        Some(r) => r.as_bytes().to_owned(),
                        None => Vec::new(),
                    };

                    // TODO: write something like Axum's `IntoResponse` to convert command responses
                    // and errors into structured responses
                    stream.write_all(&data).await.unwrap();
                });
            }
            Err(e) => {
                println!("error: {}", e)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Command {
    Get {
        key: String,
    },
    Set {
        key: String,
        val: String,
        exp_ms: Option<i32>,
    },
    Del {
        key: String,
    },
    FlushAll,
}

impl TryFrom<&str> for Command {
    type Error = CacheErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split_whitespace().into_iter();
        let maybe_cmd = parts.next();
        match maybe_cmd {
            None => Err(CacheErr::Request("EMPTY CMD".to_string())),
            Some(cmd) => {
                // NOTE: these are just the default command parts. Some commands may have different
                // uses and need different variable names for the various command parts[0, 1, etc.]

                // FIXME: these shouldn't return the error but should be Options that each command
                // branch can handle independently
                let key = match parts.next() {
                    Some(k) => k.to_string(),
                    None => return Err(CacheErr::Request("EMPTY SET KEY".to_string())),
                };
                let val = match parts.next() {
                    Some(v) => v.to_string(),
                    None => return Err(CacheErr::Request("EMPTY SET VALUE".to_string())),
                };
                let ttl_ms = match parts.next() {
                    Some(t) => Some(t.parse::<i32>().unwrap_or(DEFAULT_TTL_MS)),
                    None => None,
                };

                match cmd.to_uppercase().trim() {
                    "GET" => Ok(Command::Get { key }),
                    "SET" => Ok(Command::Set {
                        key,
                        val,
                        exp_ms: ttl_ms,
                    }),
                    "DEL" => Ok(Command::Del { key }),
                    "FLUSHALL" => Ok(Command::FlushAll),
                    _ => {
                        eprintln!("INVALID CMD: {}", cmd);
                        return Err(CacheErr::Request("INVALID CMD".to_string()));
                    }
                }
            }
        }
    }
}

fn parse(input: &str) -> Result<Command, CacheErr> {
    Ok(Command::try_from(input)?)
}

#[cfg(test)]
mod parse_command_tests {
    use super::*;

    #[test]
    fn fail_parse_empty_cmd() {
        let cmd = parse("");
        assert_eq!(cmd, Err(CacheErr::Request("EMPTY CMD".to_string())));

        let cmd = parse(" ");
        assert_eq!(cmd, Err(CacheErr::Request("EMPTY CMD".to_string())))
    }

    #[test]
    fn parse_valid_get() {
        let cmd = parse("GET 123");
        assert_eq!(
            cmd,
            Ok(Command::Get {
                key: "123".to_string()
            })
        )
    }

    #[test]
    fn parse_invalid_get() {
        let cmd = parse("GET");
        assert_eq!(cmd, Err(CacheErr::Request("EMPTY GET KEY".to_string())));

        let cmd = parse("GET ");
        assert_eq!(cmd, Err(CacheErr::Request("EMPTY GET KEY".to_string())));

        let cmd = parse("GET123");
        assert_eq!(cmd, Err(CacheErr::Request("INVALID CMD".to_string())))
    }
}

async fn process_command(
    store: Arc<Mutex<Store>>,
    data: &[u8],
) -> Result<Option<String>, CacheErr> {
    let input = String::from_utf8_lossy(data).to_string();
    let cmd = parse(&input)?;

    match cmd {
        Command::Get { key } => {
            let response = store.lock().await.clone().get(&key).await?;
            Ok(response)
        }
        Command::Set { key, val, exp_ms } => {
            store.lock().await.clone().set(&key, &val, exp_ms).await?;
            Ok(None)
        }
        Command::Del { key } => {
            store.lock().await.clone().del(&key).await?;
            Ok(None)
        }
        Command::FlushAll => {
            store.lock().await.clone().flush().await;
            Ok(None)
        }
    }
}

#[cfg(test)]
mod process_command_tests {
    use super::*;

    #[tokio::test]
    async fn process_get() {
        let store = Arc::new(Mutex::new(Store::new()));
        store
            .lock()
            .await
            .clone()
            .set("123", "TEST", None)
            .await
            .unwrap();

        let data = store.lock().await.clone().get("123").await.unwrap();
        assert_eq!(data, Some("TEST".to_string()))
    }
}

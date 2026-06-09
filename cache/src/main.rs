use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

mod parser;
mod protocol;
mod store;

// Alias for int type to differentiate
type Byte = u8;

// Allows us to implement custom methods over the type like scans, windows, RLE, etc.
type Bytes = Vec<Byte>;

enum CacheErr {
    Network(String),
    Request(String),
    Response(String),
}

struct Store {
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

    async fn set(self, key: &str, val: &str, exp_ms: Option<i32>) -> Result<(), CacheErr> {}
}

#[tokio::main]
async fn main() {
    // Clone store for use in spawned tasks
    let cache_store = Arc::new(Store::new());
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let stream = listener.accept().await;

        match stream {
            Ok((mut stream, _)) => {
                println!("accepted new connection");

                let store_clone = Arc::clone(&cache_store);
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

                    // Process incoming data
                    let response = process_command(&store_clone, &data).await;

                    stream.write_all(response.as_bytes()).await.unwrap();
                });
            }
            Err(e) => {
                println!("error: {}", e)
            }
        }
    }
}

/// Process incoming command and generate response
async fn process_command(store: &Store, data: &[u8]) -> String {
    let input = String::from_utf8_lossy(data).to_string();
    let command = parser::parse(&input).await;

    match command {
        Command::Get { key } => {
            let response = store.get(&key).await;
            response.to_string()
        }
        Command::Set { key, value, ttl } => {
            let response = store.set(&key, &value, ttl).await;
            response.to_string()
        }
        Command::Del { key } => {
            let response = store.del(&key).await;
            response.to_string()
        }
        Command::FlushAll => Response::Ok("OK".to_string()).to_string(),
    }
}

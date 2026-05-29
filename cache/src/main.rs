use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

mod parser;
mod protocol;
mod store;

use protocol::{Command, Response};
use store::CacheStore;

#[tokio::main]
async fn main() {
    // Clone store for use in spawned tasks
    let cache_store = Arc::new(CacheStore::new());
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
async fn process_command(store: &CacheStore, data: &[u8]) -> String {
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

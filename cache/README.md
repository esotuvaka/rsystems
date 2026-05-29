# Cache Crate

A Redis-like in-memory cache server built with Tokio.

## Features

- **GET**: Retrieve value by key
- **SET**: Store value with optional TTL (Time-To-Live)
- **DEL**: Delete keys
- **FLUSHALL**: Clear all keys
- **TTL-based expiration**: Automatic memory management

## Architecture

```
cache/
├── src/
│   ├── main.rs           # TCP server entry point
│   ├── protocol/         # Command/response types
│   │   └── command.rs
│   ├── parser/           # Command parsing
│   │   └── mod.rs
│   ├── store/            # In-memory cache with TTL
│   │   └── mod.rs
│   ├── network/          # Future: Connection handling
│   ├── parser/           # Future: Protocol parsing
│   ├── queries/          # Future: Query optimization
│   ├── streams/          # Future: Pub/sub
│   └── transactions/      # Future: Atomic ops
```

## Usage

### Start Server

```bash
cargo run --release
```

Server listens on `127.0.0.1:6379`

### Commands

```bash
# Set a value
echo "SET mykey myvalue" | nc 127.0.0.1 6379

# Get a value
echo "GET mykey" | nc 127.0.0.1 6379

# Set with 60 second TTL
echo "SET mykey myvalue EX 60" | nc 127.0.0.1 6379

# Delete a key
echo "DEL mykey" | nc 127.0.0.1 6379
```

## High Impact Feature: GET/SET with TTL

The core functionality that transforms this from a TCP echo server into a real cache:

1. **Core Caching Operations**: GET retrieves values, SET stores them
2. **TTL Support**: `SET key value EX seconds` - prevents memory leaks
3. **Automatic Cleanup**: Expired entries are automatically removed on GET
4. **Redis-compatible protocol**: Standard responses (+OK, $bulk, etc.)

## Response Format

- `+OK` - Simple ok response
- `$<len>\r\n<value>\r\n` - Bulk string
- `:1\r\n` - Integer
- `-error\r\n` - Error

## Testing

```bash
chmod +x test-cache.sh
./test-cache.sh
```

## Dependencies

- tokio 1.52.3 (async runtime)

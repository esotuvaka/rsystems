#!/bin/bash

# Quick test script for the cache server
# Start server in background
cd /home/esot/Code/personal/rsystems/cache
cargo run &
SERVER_PID=$!

# Wait for server to start
sleep 2

echo "=== Testing Cache Server ==="
echo ""

# Test SET
echo "1. SET mykey value123"
echo "SET mykey value123" | nc -w 1 127.0.0.1 6379
echo ""

# Test GET
echo "2. GET mykey"
echo "GET mykey" | nc -w 1 127.0.0.1 6379
echo ""

# Test SET with TTL
echo "3. SET myttl mydata EX 10"
echo "SET myttl mydata EX 10" | nc -w 1 127.0.0.1 6379
echo ""

# Wait 15 seconds for TTL to expire
echo "4. Waiting 15 seconds for TTL to expire..."
sleep 15

# Try to get expired key
echo "5. GET myttl (should be nil after expiration)"
echo "GET myttl" | nc -w 1 127.0.0.1 6379
echo ""

# Clean up
kill $SERVER_PID 2>/dev/null || true
echo "=== Tests Complete ==="

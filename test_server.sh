#!/bin/bash

echo "Starting MCP System Monitor Server..."
echo "====================================="

# Start the server in background
cargo run &
SERVER_PID=$!

# Wait for server to start
echo "Waiting for server to start..."
sleep 3

# Test health endpoint
echo -e "\n1. Testing health endpoint..."
curl -s http://localhost:8080/health | jq '.'

# Test system info endpoint
echo -e "\n2. Testing system info endpoint..."
curl -s http://localhost:8080/api/system/info | jq '.'

# Test CPU info endpoint
echo -e "\n3. Testing CPU info endpoint..."
curl -s http://localhost:8080/api/system/cpu | jq '.'

# Test memory info endpoint
echo -e "\n4. Testing memory info endpoint..."
curl -s http://localhost:8080/api/system/memory | jq '.'

# Test disk info endpoint
echo -e "\n5. Testing disk info endpoint..."
curl -s http://localhost:8080/api/system/disks | jq '.'

# Test network info endpoint
echo -e "\n6. Testing network info endpoint..."
curl -s http://localhost:8080/api/system/networks | jq '.'

# Test processes endpoint
echo -e "\n7. Testing processes endpoint..."
curl -s http://localhost:8080/api/system/processes | jq '.'

# Test process by PID endpoint
echo -e "\n8. Testing process by PID endpoint..."
curl -s http://localhost:8080/api/system/processes/1 | jq '.'

# Test system metrics endpoint
echo -e "\n9. Testing system metrics endpoint..."
curl -s http://localhost:8080/api/system/metrics | jq '.'

# Test monitoring start endpoint
echo -e "\n10. Testing monitoring start endpoint..."
curl -s -X POST http://localhost:8080/api/monitoring/start | jq '.'

# Test monitoring status endpoint
echo -e "\n11. Testing monitoring status endpoint..."
curl -s http://localhost:8080/api/monitoring/status | jq '.'

# Test monitoring stop endpoint
echo -e "\n12. Testing monitoring stop endpoint..."
curl -s -X POST http://localhost:8080/api/monitoring/stop | jq '.'

echo -e "\n====================================="
echo "All tests completed!"

# Stop the server
echo "Stopping server..."
kill $SERVER_PID
wait $SERVER_PID 2>/dev/null

echo "Server stopped." 
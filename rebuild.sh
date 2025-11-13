#!/bin/bash
# Rebuild script for Config Manager
# Performs full build cycle: format, build backend and frontend

set -e

cd "$(dirname "$0")"

echo "[INFO] Rebuilding Config Manager..."
echo ""

# Kill any running servers
echo "[INFO] Stopping running servers..."
pkill -f config-manager-server || true
sleep 1
echo ""

# Format code
echo "[INFO] Formatting code..."
cargo fmt --all

# Build backend (dev and release)
echo "[INFO] Building backend (dev profile)..."
cargo build

echo "[INFO] Building backend (release profile with auditable)..."
cargo auditable build --release

# Build frontend
cd frontend
echo "[INFO] Formatting frontend..."
cargo fmt

echo "[INFO] Building WASM frontend with Trunk (release)..."
trunk build --release

echo "[INFO] Building WASM frontend with Trunk (dev, for cargo run)..."
trunk build

cd ..

echo ""
echo "[OK] Rebuild complete!"
echo "[INFO] Backend built with audit metadata"
echo ""

# Start server in background
echo "[INFO] Starting server..."
LOG_FILE="server.log"
nohup cargo run --bin config-manager-server > "$LOG_FILE" 2>&1 &
SERVER_PID=$!
sleep 2

# Check if server started successfully
if kill -0 $SERVER_PID 2>/dev/null; then
    echo "[OK] Server started (PID: $SERVER_PID)"
    echo "[INFO] Access at http://10.1.1.30:3000"
    echo "[INFO] Server logs: tail -f $LOG_FILE"
    echo "[INFO] Stop server: kill $SERVER_PID"
    echo ""
    echo "[INFO] Refresh your browser to see changes"
else
    echo "[ERROR] Server failed to start"
    echo "[ERROR] Check $LOG_FILE for details"
    exit 1
fi

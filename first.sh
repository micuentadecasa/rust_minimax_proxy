#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PORT="${PORT:-8080}"
PYTHON="$ROOT_DIR/.venv/bin/python"
LOG_FILE="${LOG_FILE:-/tmp/minimax_proxy_${PORT}.log}"

if [[ ! -x "$PYTHON" ]]; then
  PYTHON="python3"
fi

cd "$ROOT_DIR"

# Stop any existing proxy on this port to avoid using stale code.
EXISTING_PID="$(lsof -tiTCP:"$PORT" -sTCP:LISTEN || true)"
if [[ -n "$EXISTING_PID" ]]; then
  echo "Stopping existing process on port $PORT: $EXISTING_PID"
  kill $EXISTING_PID || true
  sleep 1
fi

echo "Starting MiniMax proxy on port $PORT..."
PORT="$PORT" cargo run >"$LOG_FILE" 2>&1 &
PROXY_PID=$!

cleanup() {
  if kill -0 "$PROXY_PID" 2>/dev/null; then
    echo "Stopping proxy (pid $PROXY_PID)..."
    kill "$PROXY_PID" || true
  fi
}
trap cleanup EXIT INT TERM

# Wait for the proxy to be ready.
echo "Waiting for proxy health check..."
for _ in {1..60}; do
  if curl -fsS "http://localhost:$PORT/health" >/dev/null 2>&1; then
    echo "Proxy is ready. Log: $LOG_FILE"
    break
  fi

  if ! kill -0 "$PROXY_PID" 2>/dev/null; then
    echo "Proxy exited early. Log output:"
    tail -100 "$LOG_FILE" || true
    exit 1
  fi

  sleep 1
done

if ! curl -fsS "http://localhost:$PORT/health" >/dev/null 2>&1; then
  echo "Timed out waiting for proxy. Log output:"
  tail -100 "$LOG_FILE" || true
  exit 1
fi

# Run the test client. Any arguments passed to this shell script are forwarded.
echo "Running test client..."
"$PYTHON" "$ROOT_DIR/test_client.py" "$@"

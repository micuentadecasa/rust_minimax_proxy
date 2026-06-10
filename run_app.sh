#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PORT="${PORT:-8080}"
APP_PORT="${APP_PORT:-5173}"
JOBS_PORT="${JOBS_PORT:-8090}"
PYTHON="${PYTHON:-$ROOT_DIR/.venv/bin/python}"
LOG_DIR="${LOG_DIR:-/tmp/minimax_proxy_app}"
PROXY_LOG="$LOG_DIR/proxy-${PORT}.log"
JOBS_LOG="$LOG_DIR/jobs-${JOBS_PORT}.log"
APP_LOG="$LOG_DIR/app-${APP_PORT}.log"

mkdir -p "$LOG_DIR"
cd "$ROOT_DIR"

cleanup() {
  trap - EXIT INT TERM

  if [[ -n "${APP_PID:-}" ]] && kill -0 "$APP_PID" 2>/dev/null; then
    echo "Stopping React app (pid $APP_PID)..."
    kill "$APP_PID" || true
  fi
  if [[ -n "${JOBS_PID:-}" ]] && kill -0 "$JOBS_PID" 2>/dev/null; then
    echo "Stopping jobs scraper service (pid $JOBS_PID)..."
    kill "$JOBS_PID" || true
  fi
  if [[ -n "${PROXY_PID:-}" ]] && kill -0 "$PROXY_PID" 2>/dev/null; then
    echo "Stopping proxy (pid $PROXY_PID)..."
    kill "$PROXY_PID" || true
  fi

  # npm/vite and uvicorn can leave child processes listening after TERM.
  local app_port_pids jobs_port_pids proxy_port_pids
  app_port_pids="$(lsof -tiTCP:"$APP_PORT" -sTCP:LISTEN || true)"
  jobs_port_pids="$(lsof -tiTCP:"$JOBS_PORT" -sTCP:LISTEN || true)"
  proxy_port_pids="$(lsof -tiTCP:"$PORT" -sTCP:LISTEN || true)"
  [[ -z "$app_port_pids" ]] || kill $app_port_pids || true
  [[ -z "$jobs_port_pids" ]] || kill $jobs_port_pids || true
  [[ -z "$proxy_port_pids" ]] || kill $proxy_port_pids || true
}

interrupt() {
  cleanup
  exit 130
}

trap cleanup EXIT
trap interrupt INT TERM

stop_port() {
  local port="$1"
  local pid
  pid="$(lsof -tiTCP:"$port" -sTCP:LISTEN || true)"
  if [[ -n "$pid" ]]; then
    echo "Stopping existing process on port $port: $pid"
    kill $pid || true
    sleep 1
  fi
}

stop_port "$PORT"
stop_port "$JOBS_PORT"
stop_port "$APP_PORT"

echo "Starting MiniMax proxy on http://localhost:$PORT ..."
PORT="$PORT" cargo run >"$PROXY_LOG" 2>&1 &
PROXY_PID=$!

for _ in {1..60}; do
  if curl -fsS "http://localhost:$PORT/health" >/dev/null 2>&1; then
    echo "Proxy ready. Log: $PROXY_LOG"
    break
  fi
  if ! kill -0 "$PROXY_PID" 2>/dev/null; then
    echo "Proxy exited early. Log output:"
    tail -100 "$PROXY_LOG" || true
    exit 1
  fi
  sleep 1
done

if ! curl -fsS "http://localhost:$PORT/health" >/dev/null 2>&1; then
  echo "Timed out waiting for proxy. Log output:"
  tail -100 "$PROXY_LOG" || true
  exit 1
fi

if [[ ! -x "$PYTHON" ]]; then
  PYTHON="python3"
fi

if ! "$PYTHON" - <<'PY' >/dev/null 2>&1
import fastapi, scrapy, uvicorn
PY
then
  echo "Installing Python jobs scraper dependencies..."
  "$PYTHON" -m pip install -r "$ROOT_DIR/requirements.txt"
fi

echo "Starting UNJobNet jobs scraper service on http://localhost:$JOBS_PORT ..."
(
  cd "$ROOT_DIR"
  "$PYTHON" -m uvicorn jobs_scraper.service:app --host 127.0.0.1 --port "$JOBS_PORT"
) >"$JOBS_LOG" 2>&1 &
JOBS_PID=$!

for _ in {1..60}; do
  if curl -fsS "http://localhost:$JOBS_PORT/health" >/dev/null 2>&1; then
    echo "Jobs scraper ready. Log: $JOBS_LOG"
    break
  fi
  if ! kill -0 "$JOBS_PID" 2>/dev/null; then
    echo "Jobs scraper exited early. Log output:"
    tail -100 "$JOBS_LOG" || true
    exit 1
  fi
  sleep 1
done

if ! curl -fsS "http://localhost:$JOBS_PORT/health" >/dev/null 2>&1; then
  echo "Timed out waiting for jobs scraper. Log output:"
  tail -100 "$JOBS_LOG" || true
  exit 1
fi

if [[ ! -d "$ROOT_DIR/app/node_modules" ]]; then
  echo "Installing React app dependencies..."
  (cd "$ROOT_DIR/app" && npm install)
fi

echo "Starting React app on http://localhost:$APP_PORT ..."
(
  cd "$ROOT_DIR/app"
  APP_PORT="$APP_PORT" \
    VITE_PROXY_BASE_URL="http://localhost:$PORT" \
    VITE_JOBS_API_BASE_URL="http://localhost:$JOBS_PORT" \
    npm run dev -- --port "$APP_PORT"
) >"$APP_LOG" 2>&1 &
APP_PID=$!

for _ in {1..60}; do
  if curl -fsS "http://localhost:$APP_PORT" >/dev/null 2>&1; then
    echo "React app ready. Log: $APP_LOG"
    echo
    echo "Open: http://localhost:$APP_PORT"
    echo "Press Ctrl+C to stop proxy, jobs scraper, and app."
    wait "$APP_PID"
    exit $?
  fi
  if ! kill -0 "$APP_PID" 2>/dev/null; then
    echo "React app exited early. Log output:"
    tail -100 "$APP_LOG" || true
    exit 1
  fi
  sleep 1
done

echo "Timed out waiting for React app. Log output:"
tail -100 "$APP_LOG" || true
exit 1

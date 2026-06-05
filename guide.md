# MiniMax OAuth Proxy - Testing Guide

This proxy automatically handles MiniMax's OAuth authentication on startup. Just run `cargo run` and it will open your browser to authenticate.

## Quick Start

### 1. Build (if needed)

```bash
cd /Users/luis/projects/rust_minimax_proxy
cargo build
```

### 2. Run

```bash
cargo run
```

**That's it!** The proxy will:

1. Check for existing valid credentials
2. If not authenticated, automatically start OAuth flow
3. Open your browser to the authorization page
4. Show you the user code to enter
5. Poll for the token automatically
6. Once authenticated, start the server

## What You'll See

```
==============================================
  MiniMax OAuth Proxy
==============================================

==============================================
  🔐 MiniMax OAuth Authentication Required
==============================================

  User Code: XXXX-XXXX

  Opening browser for authorization...

  If browser doesn't open, visit:
  https://platform.minimax.io/oauth-authorize?user_code=XXXX-XXXX&client=MiniMax+CLI

  Enter the user code: XXXX-XXXX
  Then click Authorize

  Waiting for authentication...
  (Press Ctrl+C to exit and try again)

  ........
  ✅ Authentication successful!

==============================================
  MiniMax Proxy Server (OAuth)
  port: http://0.0.0.0:8080
==============================================
```

## Authorization Steps

1. Browser opens automatically (or manually visit the URL)
2. Enter the **User Code** shown in the terminal
3. Click **Authorize**
4. The proxy will detect authorization and start serving

## Test the API

Once the server is running:

```bash
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "messages": [{"role": "user", "content": "Say hello in exactly 5 words"}],
    "model": "MiniMax-M2.7"
  }'
```

Or use the Python test client:

```bash
python test_client.py "What is AI?"
python test_client.py --multi
```

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check + auth status |
| `/auth/status` | GET | Check authentication status |
| `/auth/login` | POST | (Disabled - auto-auth on startup) |
| `/auth/token` | POST | Poll for OAuth token (if needed manually) |
| `/auth/refresh` | POST | Refresh OAuth token |
| `/v1/chat/completions` | POST | Chat completions (OpenAI-compatible) |

## Error Handling

### Token Expired
If your token expires, call:

```bash
curl -X POST http://localhost:8080/auth/refresh
```

### Re-authenticate
If refresh fails, stop the server (Ctrl+C) and run `cargo run` again - it will re-authenticate automatically.

## Architecture

```
┌─────────────┐    ┌─────────────────┐    ┌──────────────────┐
│   Agents/   │───▶│  Rust Proxy     │───▶│  MiniMax OAuth   │
│  Services   │◀───│  (API Server)   │◀───│  API             │
└─────────────┘    └─────────────────┘    └──────────────────┘
       │                   │
       │                   └── Auto-auth on startup
       │                       (opens browser)
       │
       └── OpenAI-compatible API
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `8080` | Server port |

Example:
```bash
PORT=9000 cargo run
```

## Credentials

Credentials are stored in `~/.mmx/credentials.json` and persist between runs. You only need to authenticate once until the token expires (~1 hour).

## Troubleshooting

### Browser doesn't open
Manually visit the URL shown in the terminal.

### Auth times out
Stop (Ctrl+C) and run `cargo run` again.

### Server already running on port 8080
```bash
pkill -f minimax_proxy
cargo run
```

## Full Flow

```bash
# Just this! Everything is automatic:
cargo run

# While running, test in another terminal:
curl http://localhost:8080/health
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"messages": [{"role": "user", "content": "Hello"}]}'
```
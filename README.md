Small Rust backend using [axum](https://crates.io/crates/axum) that exposes a single endpoint to forward contact form submissions to the Resend API.

Overview
- Source: [src/main.rs](src/main.rs) — implements the HTTP server and handlers:
  - [`main`](src/main.rs)
  - [`contact_handler`](src/main.rs)
  - [`AppState`](src/main.rs)
  - [`ResendPayload`](src/main.rs)
  - [`AppError`](src/main.rs)

Prerequisites
- Rust toolchain (stable)
- A Resend API key and a verified sender email

Setup
1. Copy or update the [.env.example](.env.example) file with your values:
   - RESEND_API_KEY
   - FROM_EMAIL
   - TO_EMAIL

Build & run
```sh
# build
cargo build

# run (development)
cargo run

# run optimized
cargo run --release
```

Notes
- Configuration is loaded via dotenvy from the [.env](.env) file.
- CORS is enabled (open) via tower-http in [src/main.rs](src/main.rs).
- The server listens on http://127.0.0.1:3001 and exposes POST /api/contact.

Example request
```sh
curl -X POST http://127.0.0.1:3001/api/contact \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice","email":"alice@example.com","message":"Hello!"}'
```

# Email Validation API

Email Validation API is a lightweight HTTP service for validating email input and returning a clear boolean result that can be consumed by web apps, backend systems, and automation flows.

## Service overview

This API is built for one job: receive an email, evaluate it, and return a consistent response contract.

Core capabilities:

- Fast validation response for API clients
- Stable JSON contract (`email`, `valid`)
- Health endpoint for uptime checks and monitoring
- Operational defaults suitable for production deployment

## Technology stack

- **Language:** Rust
- **HTTP framework:** Axum
- **Runtime:** Tokio
- **Middleware:** Tower / Tower HTTP
- **Observability:** Tracing (structured logs)

## How the service works

1. Client sends a `POST` request with an email payload.
2. Service reads and normalizes the input.
3. Validation engine returns a boolean decision.
4. API responds with the original email and `valid: true/false`.

## API endpoints

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/health` | Health check endpoint |
| `POST` | `/v1/validate-email` | Validate an email address |

## Validate email

### Request

`POST /v1/validate-email`

```json
{
  "email": "hello@example.com"
}
```

### Success response

```json
{
  "email": "hello@example.com",
  "valid": true
}
```

### Error response example

```json
{
  "error": "email must not be empty"
}
```

## Health check

`GET /health`

```json
{
  "status": "ok"
}
```

## Local run

```bash
cargo run
```

## Example usage

```bash
curl -sS -X POST http://127.0.0.1:8080/v1/validate-email \
  -H 'content-type: application/json' \
  -d '{"email":"hello@example.com"}'
```

## Configuration

| Variable | Default | Description |
| --- | --- | --- |
| `APP_HOST` | `0.0.0.0` | Service bind host |
| `APP_PORT` | `8080` | Service bind port |
| `APP_REQUEST_TIMEOUT_SECONDS` | `10` | Request timeout |
| `RUST_LOG` | `info` | Log verbosity level |

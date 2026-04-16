# Email Validator API (Rust)

Production-ready scaffold for an email validation API service.

## Current behavior

`POST /v1/validate-email` accepts an email and returns:

```json
{
  "email": "user@example.com",
  "valid": true
}
```

The `valid` result is intentionally random for now.

## API endpoints

- `GET /health`
- `POST /v1/validate-email`

### Request body

```json
{
  "email": "user@example.com"
}
```

### Response body

```json
{
  "email": "user@example.com",
  "valid": false
}
```

## Configuration

Environment variables:

- `APP_HOST` (default: `0.0.0.0`)
- `APP_PORT` (default: `8080`)
- `APP_REQUEST_TIMEOUT_SECONDS` (default: `10`)
- `RUST_LOG` (default: `info`)

## Run

```bash
cargo run
```

## Quick check

```bash
curl -sS http://127.0.0.1:8080/health
curl -sS -X POST http://127.0.0.1:8080/v1/validate-email \
  -H 'content-type: application/json' \
  -d '{"email":"hello@example.com"}'
```

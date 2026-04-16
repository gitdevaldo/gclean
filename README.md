# Email Validation API

Email Validation API is a service that checks an email input and returns a clear validation result for your app or workflow.

## Service overview

This service is built for one job: receive an email, evaluate it, and return a consistent result.

Core value:

- Simple integration for product teams
- Predictable response format
- Health endpoint for service status

## How the service works

1. Send an email to the validation endpoint.
2. The service checks the email.
3. You receive a response with `email` and `valid`.

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

### Error response

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

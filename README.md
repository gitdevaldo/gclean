# Email Validation Service

This service checks an email input and returns a simple validation result that your application can consume.

## What this service provides

- A single API to submit an email and get a yes/no validation result.
- A consistent JSON response contract for easy integration with frontend, backend, or automation workflows.
- A health endpoint so you can quickly confirm the service is up.

## How email checking works (current phase)

For now, the validation decision is a **simulation** and returned randomly (`true` or `false`).

This is intentional so you can already integrate with the API contract while real validation logic is being prepared.

## How to check an email

Send a `POST` request to:

`/v1/validate-email`

with body:

```json
{
  "email": "hello@example.com"
}
```

The service returns:

```json
{
  "email": "hello@example.com",
  "valid": true
}
```

## Result meaning

- `valid: true` means the email passed the current check.
- `valid: false` means the email did not pass the current check.

## Service health check

Use:

`GET /health`

Expected response:

```json
{
  "status": "ok"
}
```

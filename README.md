# Email Validation Actor

This project is an Apify actor that checks email addresses by calling an external validation API.

## What this actor does

The actor receives a list of emails, sends each one to the validation API, and saves the API results into dataset output.

It is useful for:

- signup filtering
- lead cleanup
- CRM hygiene
- fraud reduction workflows

## How checking works

1. The actor takes your email list as input.
2. Each email is sent to the validation API endpoint.
3. The API returns either a validation result or an error message.
4. The actor stores all returned results into the dataset.

## What result you get

The actor mirrors the API output format:

- success result includes `email` and `valid`
- invalid request result includes `error`

## Notes

The validation decision is produced by the external API service (`https://zrbot.devaldo.workers.dev/v1/validate-email`), while this actor handles batch execution and dataset output.

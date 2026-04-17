# What are Apify Actors?

Apify Actors are containerized cloud programs. They accept structured JSON input, run a focused job, and store structured output in Apify storages.

This repository contains an Actor focused on one job: validating emails by forwarding each email to an external validation API and saving the returned results.

## Actor directory structure

```text
.actor/
├── actor.json
├── input_schema.json
├── dataset_schema.json
└── output_schema.json
src/
└── main.rs
Dockerfile
README.md
AGENTS.md
```

## Development flow for this project

1. Update `.actor/actor.json` metadata for the release (name, version, build tag, generatedBy if needed).
2. Update `.actor/input_schema.json`, `.actor/dataset_schema.json`, and `.actor/output_schema.json` if input or output behavior changes.
3. Implement code changes in `src/`.
4. Run the Actor locally with `apify run`.
5. Authenticate using `apify login`.
6. Push to the Apify platform with `apify push`.

## Project-specific behavior

- Input supports only `emails` (single = one item, batch = multiple items).
- Each email is sent to: `https://zrbot.devaldo.workers.dev/v1/validate-email`.
- Dataset items mirror the API response shape:
  - success: `{ "email": "...", "valid": true|false }`
  - error: `{ "error": "..." }`

## Do

- Keep the Actor focused on external API orchestration and dataset output.
- Preserve input/output schema compatibility when making updates.
- Validate input early and return clear failure reasons.
- Keep logs useful but concise.

## Don't

- Do not turn this Actor into a crawler template.
- Do not introduce hidden output transformations that change API response semantics.
- Do not initialize or commit local `storage/` data.

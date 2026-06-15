# Gemini Instructions

Guidance for Gemini and other code-generating assistants in this repository.

## Development Workflow

- Prefer Make targets for development tasks. They encode the repo's host
  wrapper, Docker Compose service networking, and validation defaults.
- Use raw `cargo`, `npm`, or `docker compose` commands only when no Make target
  covers the task or when a Make target explicitly documents that escape hatch.
- Keep Rust code formatted with `make fmt`; use `make fmt-compose` when the
  check must run in the Compose test-runner container.

## Database Migrations

- The source-controlled Diesel schema is `src/infra/postgres/schema.rs`.
- Run migrations with `make migrate`; redo the latest migration with
  `make migrate-redo`.
- Regenerate only the schema with `make schema`.
- Generate migration directories with `make migration-generate NAME=...`.
- Run uncommon Diesel subcommands with
  `make diesel-compose DIESEL_ARGS='...'`.
- Do not require or document a host Diesel CLI path for normal development.

## Migration Rules

- Modify existing tables with care; inspect the current schema before adding a
  migration that changes an existing table.
- Prefer altering existing tables over creating duplicate replacement tables.
- Include reversible `up.sql` and `down.sql` files whenever possible.

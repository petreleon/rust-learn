# AGENTS.md

Guidance for AI agents and contributors working in this repository.

## Project snapshot

RustLearn is an incentivized learning platform. The backend is a Rust/Actix Web API backed by PostgreSQL/Diesel, S3-compatible object storage, background video-processing workers, and Ethereum smart-contract integrations for LearnToken rewards. A Next.js frontend lives in `web/`.

## Repository map

- `src/main.rs` — thin Actix Web binary entrypoint; bootstrap owns state setup,
  route wiring, DB/S3 setup, and startup contract deployment.
- `src/lib.rs` — library exports for tests and secondary binaries.
- `src/http/` — Actix route handlers, route scopes, extractors, and HTTP DTOs.
- `src/application/` — use-case handlers, ports, commands, outputs, and
  orchestration for migrated contexts.
- `src/domain/` — pure domain vocabulary, role/permission names, invariants,
  and transition helpers.
- `src/infra/` — concrete adapters such as PostgreSQL, object storage, and
  Ethereum integrations for migrated contexts.
- `src/bootstrap/` — process wiring, app state, app data registration, startup,
  and top-level routes.
- `src/http/middlewares/` — JWT, conditional access, hierarchy, and permission middleware.
- `src/bin/worker.rs` — background upload/video processing worker.
- `ethereum/contracts/` — Solidity contracts; generated ABI/bin artifacts live in `ethereum/artifacts/`.
- `migrations/` — Diesel migrations. Keep `up.sql` and `down.sql` reversible when possible.
- `tests/` — integration and permission tests.
- `web/` — Next.js frontend. Do not edit `web/node_modules/`.
- `k8s/` — Kubernetes manifests.
- `skills/` — repo-local Codex skill definitions for product planning,
  backend/frontend development, and testing workflows.

## Development rules

- Prefer small, focused patches with clear tests.
- Do not commit secrets. `.env` is ignored; `.env.example` should contain placeholders only.
- Do not add generated dependency directories such as `target/` or `web/node_modules/`.
- Keep manually maintained non-Markdown files at or below 180 lines. Generated files, lockfiles, binary assets, and tool-owned artifacts are exempt.
- Keep Rust code formatted with `cargo fmt` before committing.
- Prefer application use cases and context-owned infra adapters for business
  workflows/database logic instead of embedding complex queries directly in
  route handlers.
- When adding or changing permissions, update the matching constants, seed migrations, middleware usage, and `PERMISSIONS.md` if the documented matrix changes.
- When adding migrations, include both `up.sql` and `down.sql`, and regenerate/check `src/infra/postgres/schema.rs` with `make migrate`, `make migrate-redo`, or `make schema` when schema changes require it. Diesel development commands should run through Make/Compose, not a required host Diesel CLI.
- When changing Ethereum contracts, update artifacts using the existing tooling/tests and run blockchain integration tests when feasible.
- When changing worker behavior, document any new environment variables in `.env.example`, `README.md`, and `TODO/` if they affect operations.

## Useful commands

```bash
make fmt
make preflight
make test
make test-compose
make test-integration
make dev
make dev-worker
make migrate
make schema
```

Prefer Make targets for development workflows. Use `make test` for host Rust
tests; it rewrites Compose-only service hosts to localhost, adds local native
library paths such as Homebrew `libpq`, and keeps host artifacts in
`target/host-tests`. For direct Cargo-style host test filters, use
`./scripts/run-host-tests.sh cargo test ...` instead of bare `cargo test`.
Diesel development commands must go through the Compose tool container via
`make migrate`, `make migrate-redo`, `make schema`,
`make migration-generate NAME=...`, or `make diesel-compose DIESEL_ARGS='...'`;
do not require a host Diesel CLI.
The worker binary can require a large Docker VM memory allocation during release
builds.

## Pull request checklist

- [ ] `make preflight` passes, or limitations are documented.
- [ ] Relevant `make test`, `make test-compose`, or host-wrapper Cargo tests pass, or limitations are documented.
- [ ] Documentation is updated for behavior, environment, deployment, or permission changes.
- [ ] No secrets, generated build outputs, or dependency folders are committed.

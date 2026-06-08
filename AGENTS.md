# AGENTS.md

Guidance for AI agents and contributors working in this repository.

## Project snapshot

RustLearn is an incentivized learning platform. The backend is a Rust/Actix Web API backed by PostgreSQL/Diesel, S3-compatible object storage, background video-processing workers, and Ethereum smart-contract integrations for LearnToken rewards. A Next.js frontend lives in `web/`.

## Repository map

- `src/main.rs` — Actix Web binary entrypoint, application state, DB setup, S3 setup, and startup contract deployment.
- `src/lib.rs` — library exports for tests and secondary binaries.
- `src/api/` — HTTP route handlers and route scopes.
- `src/middlewares/` — JWT, conditional access, hierarchy, and permission middleware.
- `src/models/` — Diesel models and request/response domain types.
- `src/repositories/` — database access helpers.
- `src/services/` — higher-level business workflows.
- `src/config/` — database setup/versioning and role/permission constants.
- `src/utils/` — JWT, S3, notifications, wallet, and Ethereum helpers.
- `src/bin/worker.rs` — background upload/video processing worker.
- `ethereum/contracts/` — Solidity contracts; generated ABI/bin artifacts live in `ethereum/artifacts/`.
- `migrations/` — Diesel migrations. Keep `up.sql` and `down.sql` reversible when possible.
- `tests/` — integration and permission tests.
- `web/` — Next.js frontend. Do not edit `web/node_modules/`.
- `k8s/` — Kubernetes manifests.

## Development rules

- Prefer small, focused patches with clear tests.
- Do not commit secrets. `.env` is ignored; `.env.example` should contain placeholders only.
- Do not add generated dependency directories such as `target/` or `web/node_modules/`.
- Keep Rust code formatted with `cargo fmt` before committing.
- Prefer repository/service layers for database logic instead of embedding complex queries directly in route handlers.
- When adding or changing permissions, update the matching constants, seed migrations, middleware usage, and `PERMISSIONS.md` if the documented matrix changes.
- When adding migrations, include both `up.sql` and `down.sql`, and regenerate/check `src/db/schema.rs` when schema changes require it.
- When changing Ethereum contracts, update artifacts using the existing tooling/tests and run blockchain integration tests when feasible.
- When changing worker behavior, document any new environment variables in `.env.example`, `README.md`, and `TODO.md` if they affect operations.

## Useful commands

```bash
cargo fmt --all --check
make preflight
make test
make test-compose
make test-integration
make dev
make dev-worker
```

Use `make test` for host Rust tests; it rewrites Compose-only service hosts to localhost, adds local native library paths such as Homebrew `libpq`, and keeps host artifacts in `target/host-tests`. For direct Cargo-style host test filters, use `./scripts/run-host-tests.sh cargo test ...` instead of bare `cargo test`. For containerized development, use Docker Compose commands from the README or Makefile. The worker binary can require a large Docker VM memory allocation during release builds.

## Pull request checklist

- [ ] `make preflight` passes, or limitations are documented.
- [ ] Relevant `make test`, `make test-compose`, or host-wrapper Cargo tests pass, or limitations are documented.
- [ ] Documentation is updated for behavior, environment, deployment, or permission changes.
- [ ] No secrets, generated build outputs, or dependency folders are committed.

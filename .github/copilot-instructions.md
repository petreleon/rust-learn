## Getting started

- Prefer the Makefile targets; they encode the local host wrapper, Docker
  Compose service networking, and Kubernetes validation paths used by this repo.
- Use `make setup` once to create a local `.env` and development JWT key pair,
  then edit `.env` for environment-specific values.
- Start the local Docker Compose stack with `make dev` or selected infra with
  `docker compose up -d db rustfs anvil`.

## Quick context for code-generating agents

- Language: Rust (edition 2021). Web server: Actix Web. DB: PostgreSQL via Diesel async/deadpool.
- Smart contracts live under `ethereum/contracts/` and are compiled/deployed via `src/utils/eth/` helpers such as `try_compile_contract`, `try_deploy_contract`, `try_get_provider`, `try_load_wallet_from_env`, and `deploy_startup`. Prefer using these fallible helpers instead of invoking solc directly.
- On startup (`src/main.rs`) the app:
  - loads `.env`, establishes the DB pool (`db::establish_connection()`),
  - initializes S3 state (`utils::s3_utils::S3State::new_from_env()`),
  - runs `version_updater` to migrate DB versioning,
  - ensures the LearnToken contracts are deployed via `deploy_startup`, then launches the Actix server with DB pool and S3 in `App::data()`.

## Useful developer workflows

- Host-side Rust checks:

  - Start host-run dependencies: `make dev-deps`
  - Run the backend on the host: `make dev-run`
  - Full host test suite: `make test`
  - Narrow host test suite: `make test CARGO_TEST_ARGS='--lib'`
  - Ad hoc host Cargo command: `./scripts/run-host-tests.sh cargo test --test authentication_flow`
  - Formatting: `cargo fmt --all --check`
  - Clippy: `make clippy`

- Docker Compose checks:

  - Start services: `make dev`
  - Run app locally through Compose: `docker compose up -d app web worker`
  - Rebuild/restart selected app or web services after code changes: `make dev-refresh`
  - Test through Compose service networking: `make test-compose`
  - Narrow Compose test suite: `make test-compose CARGO_TEST_ARGS='--lib'`
  - Blockchain integration tests (requires `ETH_MNEMONIC` in `.env` and Anvil up):
    `make test-integration`
  - Frontend lint through Compose: `make web-lint-compose`
  - Frontend production image build: `make web-build-compose`

- Runtime and Kubernetes checks:

  - Runtime health across Compose and Kubernetes: `make runtime-verify`
  - Recent runtime warnings/errors: `make runtime-log-scan`
  - Render Kubernetes manifests: `make k8s-validate`
  - Refresh local Kubernetes dev images and deployments: `make k8s-dev-refresh`
  - Refresh only the local Kubernetes web image after frontend changes:
    `make k8s-dev-refresh-web`

- Commands that intentionally run inside Compose containers:

  - Open a shell in a running service: `make shell SERVICE=app`
  - Open a shell manually: `docker compose exec app bash`
  - The Compose `app` service already starts the API under `PROD_MODE=TRUE`;
    use `make dev-refresh COMPOSE_REFRESH_SERVICES=app` after API image changes.
  - Database migrations: `make migrate` runs Diesel through Docker Compose.
  - Export ABI/bytecode example:
    `docker compose exec app cargo run --bin abi_export -- ethereum/contracts/LearnToken.sol LearnToken ethereum/artifacts`

- Logs and diagnostics:

  - App logs: `docker compose logs -f app`
  - Worker logs: `docker compose logs -f worker`
  - Web logs: `docker compose logs -f web`
  - Anvil logs: `docker compose logs -f anvil`
  - Verify solc in image: `docker compose exec app solc --version`

- Generate RSA keys inside the container (keys will appear in the mounted repo root):

  - `docker compose exec app openssl genpkey -algorithm RSA -out private.key -pkeyopt rsa_keygen_bits:2048`
  - `docker compose exec app openssl rsa -pubout -in private.key -out public.key`


## Project-specific conventions & gotchas

- Contract compilation: code first tries committed artifacts, then `ethers_solc`, and finally falls back to the `solc` CLI (see `src/utils/eth/compiler.rs`). The repo includes a heavy multi-stage `Dockerfile` that builds `solc` and Z3; prefer using the Docker image or the helper functions rather than replicating the solc build steps locally.
  - Prefer `make test-integration` for Anvil-backed contract behavior and
    `docker compose exec app ...` only for commands that specifically need the
    application image toolchain.

- Persistent contract state: deployed contract addresses are stored in DB persistent state (see `deploy_startup` in `src/utils/eth/deployer.rs`). When modifying deployment logic, update the persistent state key handling.

- DB connection pattern: handlers acquire connections via `pool.get()` (see `main.rs` and middleware removal note). Avoid copying an old connection-middleware pattern — tests indicate middleware was removed intentionally.

- Git submodules: `ethereum/contracts/lib/openzeppelin-contracts` is a submodule. If contracts or OpenZeppelin fixtures are missing, run:
  - `docker compose exec app git submodule update --init --recursive`


## Integration points & important files to inspect

- src/utils/eth/ — compile/deploy helpers (use these when adding or testing contracts).
- src/bin/abi_export.rs — shows how to export ABI/bytecode with `cargo run --bin abi_export -- ethereum/contracts/LearnToken.sol LearnToken ethereum/artifacts`.
- src/main.rs — app startup: DB pool, S3 init, deploy_startup call, Actix server wiring.
- src/infra/postgres/operations/db_setup — DB version updater called on startup (keep migrations/`migrations/` in sync).
- docker-compose.yml, Dockerfile, and docker/test-runner.Dockerfile —
  development infra, test runner, and how `solc`/Z3 are produced; heavy builds
  exist in the Dockerfile (use cautiously).


## How to extend safely (handy rules for codegen)

- When adding endpoints, follow the existing pattern: use `web::Data` for shared pool/state, call `pool.get()` inside handlers, and return Actix `Responder` types.
- For changes touching contracts, prefer the fallible helpers in `src/utils/eth/`, especially `try_compile_contract(...)` and `try_deploy_contract(...)`, so tests and startup idempotency are preserved without panic-based failures.
- Keep database schema changes in `migrations/` and ensure `version_updater` semantics are preserved; tests and startup depend on these migrations running.

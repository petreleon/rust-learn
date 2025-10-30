## Getting started (Docker-only)

- Always run commands via `docker compose exec app ...` to ensure a consistent toolchain and environment.
- Bring up infra and the app container: `docker compose up -d db minio anvil app`
- Open a shell inside the app container: `docker compose exec app bash`

## Quick context for code-generating agents

- Language: Rust (edition 2021). Web server: Actix Web. DB: PostgreSQL via Diesel r2d2 pool.
- Smart contracts live under `ethereum/contracts/` and are compiled/deployed via `src/utils/eth_utils.rs` (functions: `compile_contract`, `deploy_contract`, `get_provider`, `load_wallet_from_env`, `deploy_startup`). Prefer using these helpers instead of invoking solc directly.
- On startup (`src/main.rs`) the app:
  - loads `.env`, establishes the DB pool (`db::establish_connection()`),
  - initializes MinIO state (`utils::minio_utils::MinioState::new_from_env()`),
  - runs `version_updater` to migrate DB versioning,
  - ensures the LearnToken contracts are deployed via `deploy_startup`, then launches the Actix server with DB pool and MinIO in `App::data()`.

## Useful developer workflows (Docker Compose only)

- Start services (Postgres, MinIO, Anvil, App container shellable):

  - Bring up infra: `docker compose up -d db minio anvil app`
  - The `app` service initializes git submodules and stays running; you exec into it to run commands.

- Run commands inside the app container (preferred for all dev work):

  - Open a shell: `docker compose exec app bash`
  - Run the backend: `docker compose exec app cargo run`
  - Unit tests: `docker compose exec app cargo test`
  - Blockchain integration tests (requires `ETH_MNEMONIC` in `.env` and Anvil up):
    `docker compose exec app cargo test --test blockchain_integration_tests -- --ignored`
  - Database migrations (diesel CLI is preinstalled in the image):
    `docker compose exec app diesel migration run`
  - Export ABI/bytecode example:
    `docker compose exec app cargo run --bin abi_export -- ethereum/contracts/LearnToken.sol LearnToken ethereum/artifacts`

- Logs and diagnostics:

  - App logs: `docker compose logs -f app`
  - Anvil logs: `docker compose logs -f anvil`
  - Verify solc in image: `docker compose exec app solc --version`

- Generate RSA keys inside the container (keys will appear in the mounted repo root):

  - `docker compose exec app openssl genpkey -algorithm RSA -out private.key -pkeyopt rsa_keygen_bits:2048`
  - `docker compose exec app openssl rsa -pubout -in private.key -out public.key`


## Project-specific conventions & gotchas

- Contract compilation: code first tries `ethers_solc` and falls back to the `solc` CLI (see `src/utils/eth_utils.rs`). The repo includes a heavy multi-stage `Dockerfile` that builds `solc` and Z3 — prefer using the Docker image or the helper functions rather than replicating the solc build steps locally.
  - Run all compile/deploy-related commands via `docker compose exec app ...` to ensure consistent toolchain.

- Persistent contract state: deployed contract addresses are stored in DB persistent state (see `deploy_startup` comments in `src/utils/eth_utils.rs`). When modifying deployment logic, update the persistent state key handling.

- DB connection pattern: handlers acquire connections via `pool.get()` (see `main.rs` and middleware removal note). Avoid copying an old connection-middleware pattern — tests indicate middleware was removed intentionally.

- Git submodules: `ethereum/contracts/lib/openzeppelin-contracts` is a submodule. If contracts or OpenZeppelin fixtures are missing, run:
  - `docker compose exec app git submodule update --init --recursive`


## Integration points & important files to inspect

- src/utils/eth_utils.rs — compile/deploy helpers (use these when adding or testing contracts).
- src/bin/abi_export.rs — shows how to export ABI/bytecode with `cargo run --bin abi_export -- ethereum/contracts/LearnToken.sol LearnToken ethereum/artifacts`.
- src/main.rs — app startup: DB pool, MinIO init, deploy_startup call, Actix server wiring.
- src/config/db_setup.rs — DB version updater called on startup (keep migrations/`migrations/` in sync).
- docker-compose.yml & Dockerfile — development infra and how `solc`/Z3 are produced; heavy builds exist in the Dockerfile (use cautiously).


## How to extend safely (handy rules for codegen)

- When adding endpoints, follow the existing pattern: use `web::Data` for shared pool/state, call `pool.get()` inside handlers, and return Actix `Responder` types.
- For changes touching contracts, prefer calling `compile_contract(...)` and `deploy_contract(...)` (from `src/utils/eth_utils.rs`) so tests and startup idempotency are preserved.
- Keep database schema changes in `migrations/` and ensure `version_updater` semantics are preserved; tests and startup depend on these migrations running.

If anything in these notes is unclear or you'd like more examples (small PR-ready edits, tests, or a checklist for preparing a dev environment), tell me which section to expand and I'll iterate. 

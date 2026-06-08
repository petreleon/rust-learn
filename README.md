# RustLearn

RustLearn is an incentivized e-learning platform built with Rust. It combines a modular Actix Web API, PostgreSQL/Diesel persistence, S3-compatible media storage, background video processing, Ethereum smart contracts, and a Next.js frontend to support learning experiences where achievement can be rewarded with LearnToken.

## Why RustLearn?

Traditional learning platforms often make progress feel abstract and delayed. RustLearn is designed around immediate, auditable progress: learners complete coursework, teachers and organizations manage educational programs, and reward workflows can connect learning achievements to token-backed incentives.

The long-term goal is to provide a trustworthy platform where:

- learners can discover courses, consume content, complete assessments, and receive progress notifications;
- teachers can create and publish content, manage enrollments, and moderate course activity;
- organizations can manage members, courses, reports, and scoped permissions;
- platform administrators can operate global roles, permissions, wallets, exports, and integrations;
- blockchain-backed reward flows can be tested and audited before broader deployment.

See [`VISION.md`](VISION.md) for the product and architecture direction, and [`TODO.md`](TODO.md) for the current roadmap.
The target LearnToken reward flow is defined in
[`REWARD_LIFECYCLE.md`](REWARD_LIFECYCLE.md).

## Repository layout

```text
.
├── src/                    # Rust API, models, middleware, services, repositories, utilities
│   ├── api/                # Actix route handlers and route scopes
│   ├── bin/worker.rs       # Background upload/video-processing worker
│   ├── config/             # DB setup and role/permission constants
│   ├── db/                 # Diesel schema and connection setup
│   ├── middlewares/        # JWT, hierarchy, and permission middleware
│   ├── models/             # Diesel/domain models
│   ├── repositories/       # Persistence helpers
│   ├── services/           # Business workflows
│   └── utils/              # JWT, S3, notifications, wallets, Ethereum helpers
├── ethereum/               # Solidity contracts and generated ABI/bin artifacts
├── migrations/             # Diesel migrations
├── tests/                  # Integration and permission tests
├── web/                    # Next.js frontend
├── k8s/                    # Kubernetes manifests
├── docker-compose.yml      # Local app, web, Postgres, RustFS, Anvil, worker stack
├── Makefile                # Common development, Docker, Kubernetes, and test commands
├── PERMISSIONS.md          # Current role/permission matrix
├── VISION.md               # Product and architecture vision
├── TODO.md                 # Prioritized project roadmap
└── AGENTS.md               # Contributor/AI-agent guidance
```

## Architecture flow

```mermaid
flowchart LR
    Browser[Browser] --> Web[Next.js frontend]
    Web --> API[Actix Web API]
    API --> Auth[JWT and permission middleware]
    Auth --> Services[Services and repositories]
    Services --> Postgres[(PostgreSQL / Diesel)]
    Services --> RustFS[(RustFS / S3 objects)]
    Services --> Ethereum[Anvil or Ethereum RPC]
    API --> Jobs[Upload and video jobs]
    Jobs --> Worker[Worker binary]
    Worker --> RustFS
    Worker --> Postgres
```

## Core components

### Backend API

- Rust 2021 with Actix Web.
- JWT-protected `/api` scope.
- Route modules for authentication, users, wallets, courses, organizations, and roles.
- Registration rejects weak passwords: passwords must be at least 12 characters and include lowercase, uppercase, numeric, and symbol characters.
- Diesel and Diesel Async with PostgreSQL.
- Repository and service layers for persistence/business logic.

### Permissions and roles

RustLearn models permissions at three scopes:

- platform roles and permissions;
- organization roles and permissions;
- course roles and permissions.

Middleware and hierarchy checks are used to protect sensitive actions. Keep [`PERMISSIONS.md`](PERMISSIONS.md) updated when role capabilities change.

### Storage and media worker

The project uses S3-compatible object storage through the AWS SDK. The Docker Compose stack runs RustFS for local development. A separate `worker` binary processes upload jobs, runs ffmpeg-related work, retries failures, and writes a heartbeat file used by the worker health check.

### Blockchain integration

The `ethereum/` directory contains LearnToken-related Solidity contracts and generated artifacts. Startup code can deploy LearnToken idempotently using persistent state, and integration tests cover important contract behavior.

Operational backup/restore expectations and the Ethereum artifact release
process are documented in [docs/operations.md](docs/operations.md).

### Frontend

The `web/` directory contains a Next.js app that keeps browser API calls on the same origin through `/api` by default. `API_URL` points the Next proxy at the Rust API in container/server contexts; `NEXT_PUBLIC_API_URL` can override the browser root when needed.
The first screen is a reward operations console for teacher applications,
course reward candidate decisions, platform amount review, student reward
history, organization reward reports, reward fraud blocks, delegated reward
permissions, and CSV exports. UI actions are shown or disabled from resolved
platform, organization, and course permission strings rather than role labels.

## Prerequisites

Recommended local tools:

- Rust stable toolchain and Cargo
- Docker and Docker Compose
- PostgreSQL client tooling if running migrations outside containers
- Diesel CLI if using local migration commands
- Node.js/npm for frontend development
- ffmpeg for local worker/media-processing scenarios
- OpenSSL for RSA key generation

On macOS, Colima or Docker Desktop can provide the Docker VM. The worker release build can be memory-intensive; 8GB+ allocated to the Docker VM is recommended when building worker images.

## Configuration

Create a local `.env` file from the example:

```bash
make setup
```

`make setup` copies `.env.example` to `.env` and, when OpenSSL and Python 3 are available, replaces the committed JWT placeholders with a freshly generated local RSA key pair. Then edit `.env` for your environment-specific database, object-storage, Ethereum, and bootstrap-admin values.

### RSA keys for JWT signing

`.env.example` intentionally contains only non-secret JWT key placeholders. Use `make setup` for local development, or generate a private/public RSA key pair manually:

```bash
openssl genpkey -algorithm RSA -out private.key -pkeyopt rsa_keygen_bits:2048
openssl rsa -pubout -in private.key -out public.key
```

Add the contents to `.env`:

```text
PRIVATE_KEY="-----BEGIN PRIVATE KEY-----
...
-----END PRIVATE KEY-----"

PUBLIC_KEY="-----BEGIN PUBLIC KEY-----
...
-----END PUBLIC KEY-----"
```

The API publishes the configured public key as JWKS at
`/.well-known/jwks.json` and `/api/.well-known/jwks.json` for external JWT
verification. Set `JWT_KEY_ID` when you need a stable `kid` value across key
rollout or multiple environments.

Do not commit `.env`, real private keys, or production secrets.

### Health and readiness

The API exposes `GET /health` for shallow liveness and `GET /ready` for
dependency readiness. Readiness checks PostgreSQL, S3-compatible storage, and
the configured Ethereum JSON-RPC endpoint before returning `ready`.

### Structured logging

The API and worker initialize structured terminal logging at startup. Logs use
the `RUST_LOG` filter and default to `info`; set values such as
`RUST_LOG=rust_learn=debug,info` when you need more detail from application code
without enabling verbose logs for every dependency.

The worker emits operational metric events to the same log stream:
`worker_queue_metrics` reports queue depth, ready and delayed queued jobs,
processing jobs, failed jobs, and in-flight tasks. Idle metric logging is
throttled by `WORKER_QUEUE_METRICS_INTERVAL_SECONDS` to keep Compose and
Kubernetes logs readable. Per-job events include `worker_job_claimed`,
`worker_job_started`, `worker_job_processed`, `worker_job_retry_scheduled`, and
`worker_job_terminal_failure` with attempt numbers and processing duration in
milliseconds.

### Mock email preview

Registration creates an email-verification token and prints a local-development
mock verification email to the API terminal or container logs. No email provider
is called yet. Set `APP_PUBLIC_URL` to control the base URL used in the printed
link; the Compose default points directly at the API verification endpoint.

Preview the mock email without registering a user:

```bash
cargo run --bin mock_email --features tool-bin -- learner@example.com "Demo Learner" mock-preview-token
```

Or through Docker Compose:

```bash
docker compose --profile test run --rm --no-deps test-runner cargo run --bin mock_email --features tool-bin -- learner@example.com "Demo Learner" mock-preview-token
```

### Wallet linking API

Authenticated users can create and read their own internal wallet link with
`POST /api/wallets/me/link` and `GET /api/wallets/me`. Platform wallet managers
can link or read another user wallet with `/api/wallets/users/{id}` routes.
Organization wallet managers can link and read organization wallets with
`/api/wallets/organizations/{id}` routes. Link endpoints are idempotent and
return the existing wallet on repeated calls.

Authenticated users can move LearnToken between their centralized platform
wallet and an Ethereum wallet with `POST /api/wallets/me/deposits` and
`POST /api/wallets/me/retirements`. Requests include `gas_payer` as `user` or
`platform`; when the platform pays Ethereum gas, the configured token tax is
applied as a separate wallet debit. Deposit requests create a pending deposit
intent and do not credit the internal wallet immediately; the worker deposit
indexer credits the wallet only after it observes the matching confirmed
Ethereum event. Transfer responses include
`wallet_provider`, `metamask_required`, and `wallet_action`: MetaMask is implied
for every transfer path except a platform-paid retirement, where the person is
only receiving tokens and the platform sends the transfer. Authenticated users
can read current token taxes with `GET /api/wallets/token-taxes`; operators
update them through `PUT /api/wallets/token-taxes/deposit` or
`PUT /api/wallets/token-taxes/retire`, gated by platform `SET_DEPOSIT_TAX`
and `SET_RETIRE_TAX` respectively.

### Student reward history API

Authenticated students can read `GET /api/reward-candidates/me/history` for
their own reward candidate timeline. The response includes candidate status,
approved amount, wallet credit details, and token transaction references when
available. Rows are included only for courses where the requester has
`VIEW_COURSE_REWARD_STATUS`.

### Reward fraud and delegation APIs

Platform reward-fraud operators can manage reward blocks through
`POST /api/reward-fraud-blocks`, `GET /api/reward-fraud-blocks`,
`PUT /api/reward-fraud-blocks/{id}/revoke`, and
`GET /api/reward-fraud-blocks/{id}/audit`. Teacher blocks require
`BLOCK_REWARD_TEACHER` or `MANAGE_REWARD_FRAUD_BLOCKS`; organization blocks
require `BLOCK_REWARD_ORGANIZATION` or `MANAGE_REWARD_FRAUD_BLOCKS`; course
and reward-policy blocks require `MANAGE_REWARD_FRAUD_BLOCKS`. Listing and
audit history require `VIEW_REWARD_AUDIT` or `MANAGE_REWARD_FRAUD_BLOCKS`.

Central administrators with `DELEGATE_REWARD_APPROVAL` can grant, list, and
revoke delegated reward permissions with `POST /api/delegated-permissions`,
`GET /api/delegated-permissions`, and
`PUT /api/delegated-permissions/{id}/revoke`.

### Course discovery API

`GET /api/courses` supports `search`, `organization_id`, `limit`, and `offset`
query parameters. The response includes `courses`, `total`, `limit`, `offset`,
`search`, and `organization_id` so learners and organization views can paginate
and filter discovery results consistently.

### Reporting exports

Platform administrators can read `GET /api/reports/platform/summary` and export
`GET /api/reports/platform/summary.csv`. Organization administrators can read
`GET /api/reports/organizations/{id}/summary` and export
`GET /api/reports/organizations/{id}/summary.csv`. CSV endpoints return
`text/csv` with attachment filenames. Organization reward operators with
`VIEW_ORG_REWARD_REPORTS` can read and export
`GET /api/reports/organizations/{id}/reward-dashboard` and
`GET /api/reports/organizations/{id}/reward-dashboard.csv` for sponsored
teacher applications, course reward volume, approved amounts, and organization
wallet balances.
Additional platform exports require `EXPORT_DATA`:
`GET /api/reports/platform/teacher-applications.csv`,
`GET /api/reports/platform/reward-approvals.csv`,
`GET /api/reports/platform/token-payouts.csv`,
`GET /api/reports/platform/wallet-credits.csv`, and
`GET /api/reports/platform/delegated-permissions.csv`.

### Notification events

Notifications are persisted for key product events: course enrollment, content
publication, platform/organization/course role assignment, terminal worker job
failures, and reward records. Existing upload-processing notifications continue
to use the `video:*` titles.

### PostgreSQL

Typical Docker Compose values look like:

```text
DATABASE_URL=postgres://your_username:your_password@db:5432/your_db_name
POSTGRES_DB=your_db_name
POSTGRES_USER=your_username
POSTGRES_PASSWORD=your_password
```

The Compose stack exposes PostgreSQL on host port `5433` and container port `5432`.

### S3-compatible storage

Local development uses RustFS through S3-compatible settings:

```text
S3_ACCESS_KEY=rustfsadmin
S3_SECRET_KEY=rustfsadmin
S3_INTERNAL_DOMAIN=rustfs
S3_INTERNAL_PORT=9000
S3_EXTERNAL_DOMAIN=localhost
S3_EXTERNAL_PORT=9000
S3_INTERNAL_SCHEME=http
S3_EXTERNAL_SCHEME=http
```

RustFS API is exposed on port `9000`; the console is exposed on port `9001`.

### Ethereum provider

The Compose stack runs Anvil on port `8545`. App and worker containers override
the provider URL to the Compose service:

```text
ETH_HOST=anvil
ETH_PORT=8545
ETH_RPC_URL=http://anvil:8545
```

Startup deploys and persists LearnToken plus the wallet transfer helper
contracts when they are missing. If you use pre-deployed contracts, configure
`LEARN_TOKEN_ADDRESS`, `WALLET_DEPOSIT_IMPORTER_ADDRESS`, and the treasury
receiver values in `.env`.

### Bootstrap admin

The database setup code reads admin bootstrap values from the environment. Configure these in `.env` before first startup:

```text
ADMIN_NAME=admin
ADMIN_EMAIL=admin@example.com
ADMIN_PASSWORD=supersecretpassword
ADMIN_DATE_OF_BIRTH=1990-01-01
```

## Running with Docker Compose

Start the complete local stack:

```bash
docker-compose up
```

Or use the Makefile:

```bash
make dev
```

`make dev` starts the current Compose images. After changing Rust API code,
frontend dependencies, or Dockerfiles, rebuild the app/web images before testing
the full userflow:

```bash
make dev-refresh
```

Set `COMPOSE_REFRESH_SERVICES='app web worker'` when worker image changes also
need to be rebuilt for the same local run.
The refresh target rebuilds only the selected services, so a web-only refresh
does not also rebuild the Rust API image through Compose dependencies.

Default local service ports:

| Service | URL/port |
| --- | --- |
| Rust API | <http://localhost:8080> |
| Next.js web app | <http://localhost:3000> |
| PostgreSQL | `localhost:5433` |
| RustFS S3 API | <http://localhost:9000> |
| RustFS console | <http://localhost:9001> |
| Anvil Ethereum RPC | <http://localhost:8545> |

The API container runs `scripts/app-entrypoint.sh`, which initializes submodules, waits/retries migrations, and starts the app when `PROD_MODE=TRUE`.

## Running locally without containers

Start only the API dependencies with Docker Compose:

```bash
make dev-deps
```

This starts PostgreSQL, RustFS, and Anvil without starting the API, web app, or
worker containers. It is the quickest path when you want to run the Rust API on
the host with Cargo:

```bash
cargo run --bin rust-learn --features app-bin
```

The equivalent raw Docker Compose command is:

```bash
docker-compose up -d db rustfs anvil
```

Run the worker locally:

```bash
cargo run --bin worker --features worker-bin
```

Run the frontend locally:

```bash
cd web
npm install
npm run dev
```

## Worker service

The `worker` service processes background upload jobs and media transformations. It is built from `docker/worker.Dockerfile` and runs `/usr/local/bin/worker` directly to avoid runtime compilation.

Useful configuration:

```text
WORKER_CONCURRENCY=1
WORKER_MAX_ATTEMPTS=5
WORKER_BASE_BACKOFF_SECONDS=60
WORKER_QUEUE_METRICS_INTERVAL_SECONDS=60
WALLET_DEPOSIT_INDEXER_ENABLED=true
WALLET_DEPOSIT_INDEXER_POLL_SECONDS=15
WALLET_DEPOSIT_INDEXER_IDLE_LOG_SECONDS=60
WALLET_DEPOSIT_INDEXER_CONFIRMATIONS=1
WALLET_DEPOSIT_INDEXER_BATCH_BLOCKS=500
WALLET_DEPOSIT_INDEXER_LOOKBACK_BLOCKS=100
LEARN_TOKEN_DECIMALS=18
```

The worker also runs the wallet deposit indexer. It scans LearnToken `Transfer`
events for user-paid deposits into the configured treasury and
PlatformImporter `Imported` events for platform-paid deposits. It advances its
cursor in persistent state as `wallet_deposit_indexer_next_block`; use
`WALLET_DEPOSIT_INDEXER_START_BLOCK` only for the first scan of a fresh
environment. Empty polls are logged at `WALLET_DEPOSIT_INDEXER_IDLE_LOG_SECONDS`
while polls that credit deposits are logged immediately. Poll failures log an
initial retry notice at info level, escalate to warning only after the same
interval has elapsed, and emit a recovery notice when polling succeeds again.

Recommended worker build/start flow:

```bash
make worker-build
docker compose up -d db rustfs anvil worker
```

If the worker build fails with an out-of-memory linker error, increase Docker VM memory and rebuild. With Colima, for example:

```bash
colima start --memory 8192
docker compose build worker
```

Inspect worker logs:

```bash
docker compose logs -f worker
```

The worker writes `/tmp/worker_alive`; Docker Compose and Kubernetes run
`/usr/local/bin/worker-healthcheck` against this heartbeat for health checks.
Use `docker compose logs -f worker` to watch heartbeat-adjacent metric events
for queue depth, attempts, processing duration, retries, and failed jobs.

## Testing and quality checks

Hosted GitHub Actions CI is intentionally disabled. Run the quality gates
locally, preferably through Docker Compose, to avoid spending hosted CI minutes.

Run all Rust tests on the host:

```bash
make test
```

Docker Compose equivalent:

```bash
make test-compose
docker compose --profile test run --rm test-runner cargo test
```

Business-flow verification through Docker Compose:

```bash
docker compose up -d db rustfs anvil

# Teacher application and central review queue.
docker compose --profile test run --rm test-runner cargo test --test teacher_applications

# Course enrollment and join-request reward prerequisites.
docker compose --profile test run --rm test-runner cargo test --test course_enrollment_api
docker compose --profile test run --rm test-runner cargo test --test course_join_requests

# Reward candidate submission, teacher approval, amount approval, fraud blocks,
# delegated permissions, student history, and reporting.
docker compose --profile test run --rm test-runner cargo test --test reward_candidates
docker compose --profile test run --rm test-runner cargo test --test reward_fraud_blocks
docker compose --profile test run --rm test-runner cargo test --test reward_management_api
docker compose --profile test run --rm test-runner cargo test --test delegated_permissions
docker compose --profile test run --rm test-runner cargo test --test student_reward_history
docker compose --profile test run --rm test-runner cargo test --test reporting_exports

# Wallet credit, notification idempotency, reconciliation, and transaction links.
docker compose --profile test run --rm test-runner cargo test --test reward_execution
docker compose --profile test run --rm test-runner cargo test --test wallet_linking
docker compose --profile test run --rm test-runner cargo test --test notification_events

# Anvil-backed token contract behavior.
docker compose --profile test run --rm test-runner cargo test --test blockchain_integration_tests -- --ignored
```

To pass a narrower test filter through the Make target:

```bash
make test-compose CARGO_TEST_ARGS='--lib'
make test-compose CARGO_TEST_ARGS='--test authentication_flow'
```

Run blockchain integration tests:

```bash
make test-integration
cargo test --test blockchain_integration_tests -- --ignored
```

Check formatting:

```bash
cargo fmt --all --check
```

Docker Compose equivalent:

```bash
docker compose --profile test run --rm --no-deps test-runner cargo fmt --all --check
```

Frontend checks:

```bash
cd web
npm run lint
npm run build
```

Docker Compose equivalent:

```bash
make web-lint-compose
make web-build-compose
```

`web-lint-compose` runs lint inside the bootstrapped Compose web service, so
the anonymous `node_modules` volume is populated before ESLint runs.
`web-build-compose` validates the production Dockerfile build path instead of
running `next build` inside the development service environment.

Build the Linux ARM64 Compose images:

```bash
DOCKER_DEFAULT_PLATFORM=linux/arm64 docker compose build app web worker
```

This build was last verified locally for `rust-learn-app`, `rust-learn-web`,
and `rust-learn-worker`. Kubernetes builds use separate `rust-app` and
`rust-worker` images so the API runtime does not carry ffmpeg. The web image may
report `npm audit` advisories during dependency installation; those advisories
do not fail the image build.

Makefile shortcuts:

```bash
make fmt
make clippy
make test
make test-integration
make web-lint
make web-build
make web-lint-compose
make web-build-compose
make health
make runtime-verify
make runtime-log-scan
make runtime-disk
make docker-prune-build-cache
```

Test dependency notes:

| Check | External requirements |
| --- | --- |
| `make clippy` | Local Rust toolchain plus native libraries such as `libpq`; runs Clippy across all targets and the app, worker, and tool feature flags with warnings denied. |
| `make test` | A valid `.env`; many integration tests open `DATABASE_URL`, so start PostgreSQL first with `make dev-deps` when running the full suite. The host wrapper maps Compose-only service names to localhost ports and uses `target/host-tests` so Docker and host artifacts do not collide. Host runs also need local native libraries such as `libpq`. |
| `make test-compose` | Docker plus a valid `.env`; starts PostgreSQL, RustFS, and Anvil, then runs Cargo in the `test-runner` profile so host native libraries are not required. |
| `make web-lint-compose` | Docker; runs ESLint in a one-shot Compose web container after `npm ci`, so stale anonymous `node_modules` volumes cannot hide missing dependencies. |
| `make web-build-compose` | Docker; builds the `web` image through the production Dockerfile, which is the supported Compose production-build check for the frontend. |
| `make runtime-log-scan` | Running Docker Compose stack and Kubernetes `rust-learn` namespace; scans recent app, worker, and web logs for warning/error patterns without matching routine `failed=0` worker metrics. Override the window with `LOG_SCAN_SINCE=10m`. Fails if a required log source is unreachable. |
| `cargo test --test s3` | RustFS/S3-compatible storage reachable through the `S3_*` settings. With Compose, run from the container network or set `S3_INTERNAL_DOMAIN`/`S3_EXTERNAL_DOMAIN` appropriately for the host. |
| `make test-integration` | Docker plus a valid `.env`; starts Anvil, then runs ignored blockchain tests in the `test-runner` profile. |
| `cargo test --test blockchain_integration_tests -- --ignored` | Anvil or another Ethereum JSON-RPC endpoint plus `ETH_MNEMONIC` and provider settings in `.env`; host runs also need local native libraries such as `libpq`. |
| `make runtime-verify` | Running Docker Compose stack and Kubernetes `rust-learn` namespace; fails if Compose endpoints, the worker heartbeat, K8s deployments/pods, or in-cluster web/API readiness are unhealthy. |
| Worker/media-processing checks | ffmpeg on `PATH`, PostgreSQL, and RustFS/S3. Keep `WORKER_CONCURRENCY=1` on small Docker VMs. |

Docker is the recommended way to provide PostgreSQL, RustFS, and Anvil for local test runs:

```bash
make dev-deps
```

## Database migrations

Run migrations with Diesel CLI:

```bash
diesel migration run
```

Redo the latest migration:

```bash
diesel migration redo
```

When adding migrations, include reversible `up.sql` and `down.sql` files whenever possible and update/check `src/db/schema.rs` when schema changes require it.

## Troubleshooting

Diesel migration failures usually mean the API cannot reach PostgreSQL, the
database credentials in `.env` do not match the Compose container, or a migration
failed partway through. Check the database service first:

```bash
docker-compose ps db
docker-compose logs -f db
```

When running the API on the host, `DATABASE_URL` should point at
`localhost:5433`; inside Compose it should point at `db:5432`. After fixing the
connection string or database state, rerun:

```bash
diesel migration run
```

S3 or RustFS connectivity errors usually come from using container-only hostnames
from the host, mismatched credentials, or RustFS not being ready. Check the
service and console:

```bash
docker-compose ps rustfs
docker-compose logs -f rustfs
```

Containers should use `S3_INTERNAL_DOMAIN=rustfs`. A host-run API or worker
should use `S3_INTERNAL_DOMAIN=localhost` with `S3_INTERNAL_PORT=9000` unless it
is attached to the Compose network.

Ethereum RPC startup issues usually mean Anvil is still starting, the wrong host
name is configured, or an old local state volume is being reused. Check Anvil and
query the chain ID:

```bash
docker-compose ps anvil
docker-compose logs -f anvil
curl -s -X POST -H 'Content-Type: application/json' \
  --data '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}' \
  http://localhost:8545
```

Containers should use `ETH_HOST=anvil` or `ETH_RPC_URL=http://anvil:8545`. A
host-run API should prefer `ETH_RPC_URL=http://localhost:8545`.

Worker builds and ffmpeg processing can be memory-heavy. Keep
`WORKER_CONCURRENCY=1` on small machines, increase the Docker VM memory when
release builds fail, and inspect worker logs before raising concurrency:

```bash
docker compose logs -f worker
colima start --memory 8192
docker compose build worker
```

Docker and Minikube can also run out of disk after repeated local image builds.
Check runtime storage before long Compose or Kubernetes verification runs:

```bash
make runtime-disk
```

If the build cache is the pressure source, prune only Docker build cache without
removing images, containers, or volumes:

```bash
make docker-prune-build-cache
```

## Kubernetes

Kubernetes manifests live under `k8s/`. Common commands:

```bash
make k8s-build
make k8s-apply
make k8s-status
make k8s-logs SERVICE=rust-app
make k8s-forward SERVICE=web PORT=3000
make k8s-delete
```

## Documentation

- [`VISION.md`](VISION.md) — mission, architecture direction, strategic pillars, and near-term outcomes.
- [`TODO.md`](TODO.md) — prioritized implementation roadmap.
- [`PERMISSIONS.md`](PERMISSIONS.md) — current permissions assigned to platform, organization, and course roles.
- [`THIRD_PARTY.md`](THIRD_PARTY.md) — third-party Rust crates, Docker images, and external tools.
- [`AGENTS.md`](AGENTS.md) — repository guidance for AI agents and contributors.

## Contributing notes

Before opening a pull request:

1. Keep patches focused and documented.
2. Run `cargo fmt --all --check` and relevant tests.
3. Update README/TODO/PERMISSIONS/environment docs when behavior, setup, or permissions change.
4. Do not commit `.env`, private keys, `target/`, or `web/node_modules/`.

If you need help or want to propose a larger direction change, open an issue with the problem statement, expected behavior, and any operational constraints.

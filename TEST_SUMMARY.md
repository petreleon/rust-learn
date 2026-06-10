# Test Summary — RustLearn

## By Layer

| Layer | Framework | Tests | Pass | Fail | Skip |
|---|---|---|---|---|---|
| **Rust inline** | `cargo test --lib` | **371** | 371 | 0 | 0 |
| **Rust integration** | `cargo test --tests` | **201** | 201 | 0 | 0 |
| └─ Repository tests | Docker Compose DB | 35 | 35 | 0 | 0 |
| └─ Model permission tests | Docker Compose DB | 11 | 11 | 0 | 0 |
| └─ Blockchain integration | Docker Compose Anvil | 2 | 2 | 0 | 0 |
| └─ Eth utils | Docker Compose Anvil | 4 | 4 | 0 | 0 |
| └─ Other integration | Various | 149 | 149 | 0 | 0 |
| **Solidity** | Foundry `forge test` | **44** | 44 | 0 | 0 |
| **Frontend Vitest** | `npm test` | **72** | 72 | 0 | 0 |
| **Frontend API helpers** | Node `--test` | **83** | 83 | 0 | 0 |
| **Frontend E2E** | Playwright | **38** | ready | — | — |
| **Total** | | **809** | | | |

## By Category

| Category | Tests |
|---|---|
| Validation/normalization | ~120 |
| Permission/hierarchy | ~50 |
| Reward lifecycle | ~60 |
| Wallet/transaction | ~40 |
| Authentication/session | ~25 |
| Course/content | ~35 |
| Fraud/audit/delegation | ~45 |
| CSV/export formatting | ~15 |
| Smart contract (mint/burn/permit/transfer) | 44 |
| Frontend lib utilities | 31 |
| Frontend component helpers | 41 |
| Frontend API contract | 83 |
| E2E smoke | 38 |
| Infrastructure (health, routing, K8s, Docker) | ~50 |
| Model persistence (CRUD, find, list, count) | ~80 |
| Blockchain (deploy, compile, provider) | 6 |

## Key Metrics

- **Inline test growth**: 69 → 371 (5.4x)
- **Repository tests added**: 0 → 35
- **Solidity tests added**: 0 → 44
- **Frontend tests added**: 0 → 193 (72 + 83 + 38)
- **Services with inline tests**: 2/18 → 18/18
- **Repositories with dedicated tests**: 1/18 → 9/18
- **Blockchain `#[ignore]`d tests**: 2 → 0 (all pass)

## How to Run

```bash
# Rust (host, needs Docker Compose db + anvil)
make test

# Rust integration (containerized)
make test-compose

# Blockchain integration
make test-integration

# Solidity
docker compose up -d anvil
docker run --rm -v $PWD/ethereum:/workspace -w /workspace --entrypoint forge ghcr.io/foundry-rs/foundry:nightly test

# Frontend unit + API helpers
cd web && npm test && npm run test:api-helpers

# Frontend E2E
cd web && npx playwright test

# Everything (preflight)
make preflight
```

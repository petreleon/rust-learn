# Testing Gaps — Final State

*Last updated: synced with `TEST_SUMMARY.md`. Canonical test totals live there.*

## All Resolved

| Layer | Status |
|---|---|
| Services (18/18) | 371 inline unit tests across all 18 services |
| Repositories | 35 integration tests across 5 files (reward, fraud, execution, audit, policy, delegation, course, org, platform, user) |
| Ethereum contracts | 44 Solidity tests + 2 Rust blockchain tests + 4 eth utils tests |
| Middlewares | 2 inline + 11 request_utils + covered by integration tests |
| Utils | 11/13 files tested (centralized_wallets, request_utils, eth/provider, s3, jwt, email, worker, notifications) |
| Config | 5 tests (updates list order/versions + bootstrap admin password validation) |
| Model permissions | 11 integration tests (Platform/Org/Course has_permission + hierarchy) |
| Frontend | 72 Vitest tests + 83 API helper tests + 38 Playwright E2E tests ready |
| DB setup | version_updater list tested, migrations verified |

## Remaining Gaps

### 1. Frontend Tests (HIGH -> resolved)
- [x] Vitest + React Testing Library installed and configured
- [x] **31 unit tests** across 4 lib modules (session, auth, access, organization)
- [x] **38 E2E smoke tests** via Playwright
- [x] React component tests — 41 tests for product-shell and learner-routes
      pure functions (`web/src/components/__tests__/`). teacher-routes, admin-routes,
      org-routes, and workspace-route remain.
- [x] Frontend API helper tests — 83 Node tests for success, text errors, `401`,
      `403`, timeout, network failure, and verification-token states.
- [ ] Page-level integration tests — still zero for `web/src/app/` pages
- [x] Run via: `npm test` (unit), `npm run test:e2e` (Playwright), `npm run test:api-helpers`

### 2. Worker Binary (MEDIUM)
- `src/bin/worker.rs` has `test = false` — full video-processing pipeline untested
- `src/bin/abi_export.rs` — untested
- `src/bin/mock_email.rs` — untested

### 3. HTTP Handlers (LOW)
- `src/http/access_control` role routes — no dedicated test file (exercised by integration tests)
- `src/http/identity` user routes — no dedicated test file (exercised by integration tests)

### 4. Config Migration Data (LOW)
- `src/config/db_setup/updates/update_v1.rs` — seed data insertion not verified
- `src/config/db_setup/updates/update_v2.rs` — seed data insertion not verified

### 5. Utility Edge (LOW)
- `src/utils/course_utils.rs` — all DB calls, no isolated test
- `src/utils/eth/deployer.rs` — tested via `tests/eth_utils_tests.rs`
- `src/utils/eth/wallet.rs` — tested via `tests/eth_utils_tests.rs`

---

## Totals

| Metric | Before | After |
|---|---|---|
| Inline unit tests | 69 | **374** |
| Service coverage | 2/18 | **18/18** |
| Repository integration tests | 0 | **35** (5 files) |
| Model permission integration tests | 0 | **11** (1 file) |
| Eth utils integration tests | 0 | **4** (1 file) |
| Solidity Foundry tests | 0 | **44** |
| Blockchain integration tests | 2 ignored | **2 passing** |
| Frontend Vitest | 0 | **72** |
| Frontend API helper tests | 0 | **83** |
| Frontend E2E smoke tests | 0 | **38 ready** |
| **Total test count** | ~110 | **813** |

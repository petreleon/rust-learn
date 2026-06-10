# Testing Gaps

Systematic inventory of missing test coverage, ordered by risk (highest first).

*Last updated: after adding 163 inline unit tests and ~38 Solidity tests.*

## 1. Service Layer Unit Tests (HIGH)

15 of 18 service files (~12,800 lines) had **zero unit tests**. Pure functions
(validation, normalization, state machines, enum methods) in 5 of the largest
services are now tested. DB-dependent functions still need integration-level
tests.

- [x] `src/services/course_service.rs` (2,979 lines) — **31 tests added:**
      `course_title_search_pattern` SQL escaping, `normalize_optional_string`,
      `content_display_state` state machine, `is_media_content_type`,
      `teacher_content_publication_status`, `normalize_course_status`,
      query constructor defaults/clamping/trimming, `has_teacher_access`,
      `teacher_course_dashboard_permission_names`, error conversions.
- [ ] `src/services/teacher_application_service.rs` (1,382 lines) — multi-scope
      application lifecycle, submission validation, review/decision, sponsored
      applications. **No pure-fn tests yet.**
- [ ] `src/services/reporting_service.rs` (1,359 lines) — reward/compensation/fraud
      dashboards, CSV export aggregation, date-filtered queries.
      **No pure-fn tests yet.**
- [x] `src/services/reward_candidate_service.rs` (1,312 lines) — **30 tests added:**
      `normalize_reward_event_type`, `normalize_teacher_decision_status`,
      `normalize_amount_decision_status`, `normalize_reward_status`,
      `normalize_idempotency_key`, `ensure_reward_evidence_is_eligible`,
      `ensure_evidence_number_at_least`, `candidate_teacher_user_ids`,
      error conversions.
- [x] `src/services/organization_service.rs` (1,306 lines) — **22 tests added:**
      `member_matches_query` (search/role/permission with all edge cases),
      `normalize_query_value`, `sorted_vec`, gated summary defaults,
      `organization_dashboard_alerts` (all alert conditions).
- [x] `src/services/wallet_service.rs` (1,292 lines) — **54 tests added:**
      `validate_positive_amount`, `validate_non_negative_amount`,
      `validate_transfer_request_addresses`, `validate_external_transaction_fields`,
      `validate_observed_wallet_deposit_event`, `deposit_intent_matches_observed_event`
      (all mismatch cases), `normalize_address`, `addresses_equal`,
      `wallet_delta_for_operation`, `wallet_interaction_for_transfer`,
      `WalletTokenOperation` enum methods, `WalletTokenGasPayer` enum.
- [x] `src/services/reward_execution_service.rs` (1,239 lines) — **26 tests added:**
      `ensure_candidate_ready_for_payout`, `approved_positive_amount`,
      `ensure_candidate_reconcilable`, `should_create_reconciliation_wallet_credit`,
      `validate_token_confirmation_request`, `transaction_type_for_event`,
      error conversions.
- [ ] `src/services/reward_fraud_block_service.rs` (557 lines) — fraud block
      creation, revalidation, candidate intersection. **Untested.**
- [ ] `src/services/wallet_audit_service.rs` (499 lines) — audit aggregation
      across wallet transactions, filters, summaries. **Untested.**
- [ ] `src/services/session_service.rs` (397 lines) — session building with
      permission resolution across platform/org/course scopes. **Untested.**
- [ ] `src/services/delegated_permission_service.rs` (381 lines) — grant,
      revoke, effective permission computation with delegation chains.
      **Untested.**
- [ ] `src/services/reward_history_service.rs` (322 lines) — history queries
      with wallet/token joins. **Untested.**
- [ ] `src/services/course_enrollment_service.rs` (314 lines) — enrollment state
      management, join request lifecycle. **Untested.**
- [ ] `src/services/reward_policy_service.rs` (308 lines) — policy CRUD,
      validation. **Untested.**
- [ ] `src/services/reward_compensation_service.rs` (224 lines) — compensation
      record CRUD. **Untested.**

## 2. Repository Layer Tests (HIGH)

17 of 18 repositories have no unit tests. SQL queries, transaction patterns, and
data access logic are untested at the repository level.

- [ ] `src/repositories/course_repository.rs`
- [ ] `src/repositories/organization_repository.rs`
- [ ] `src/repositories/platform_repository.rs`
- [ ] `src/repositories/platform_permission_repository.rs`
- [ ] `src/repositories/persistent_state_repository.rs`
- [ ] `src/repositories/delegated_permission_repository.rs`
- [ ] `src/repositories/reward_candidate_repository.rs`
- [ ] `src/repositories/reward_execution_job_repository.rs`
- [ ] `src/repositories/reward_policy_repository.rs`
- [ ] `src/repositories/reward_fraud_block_repository.rs`
- [ ] `src/repositories/reward_audit_event_repository.rs`
- [ ] `src/repositories/reward_payout_record_repository.rs`
- [ ] `src/repositories/reward_wallet_credit_record_repository.rs`
- [ ] `src/repositories/reward_compensation_record_repository.rs`
- [ ] `src/repositories/session_repository.rs`
- [ ] `src/repositories/teacher_application_repository.rs`
- [ ] `src/repositories/review_audit_repository.rs`

## 3. Ethereum Smart Contract Tests (HIGH -> resolved)

Three contracts manage real token value. Now have Foundry test suite.

- [x] Add Foundry or Hardhat test setup in `ethereum/` — `foundry.toml` created
- [x] Write tests for `ethereum/contracts/LearnToken.sol` — `ethereum/test/LearnToken.t.sol`
      (~14 tests: constructor, mint/burn ownership, permit, transfer, transferFrom,
      transferOwnership, renounceOwnership)
- [x] Write tests for `ethereum/contracts/LearnTokenPresigner.sol` — `ethereum/test/LearnTokenPresigner.t.sol`
      (~16 tests: deposit/withdraw, presigned execution, expired deadline, wrong
      nonce, invalid signature, zero amount, invalid recipient, insufficient balance,
      replay attack, permit+deposit combined flow)
- [x] Write tests for `ethereum/contracts/PlatformImporter.sol` — `ethereum/test/PlatformImporter.t.sol`
      (~8 tests: importWithPermit, importToRecipientWithPermit, zero amount, zero
      recipient, multiple imports, expired deadline)
- [x] Un-ignore `tests/blockchain_integration_tests.rs` — **2 tests pass** (test_deploy_and_mint, test_permit_import) against local Anvil
- [x] Install Foundry and run `forge test` — **44 tests pass** (LearnToken: 14, LearnTokenPresigner: 18, PlatformImporter: 8, plus PlatformImporter integration tests)
- [x] Moved OpenZeppelin submodule from `ethereum/contracts/lib/` to `ethereum/lib/` (Foundry convention)
- [x] Added `ethereum/lib/forge-std` submodule for Foundry test library

## 4. Middleware Unit Tests (MEDIUM -> partially resolved)

5 of 7 middleware modules are pure actix-web glue code — every function takes
`ServiceRequest` + `DbPool` or actix service wrappers, with no extractable pure
functions. They are already covered by integration tests in
`tests/middleware_access_control.rs` (518 lines).

- [x] `src/utils/request_utils.rs` — **11 tests added:** `extract_param` for
      path, query, and header parameter extraction with missing/empty/wrong-name
      cases and cross-param-type isolation. This is the shared dependency used
      by all permission and hierarchy middleware.
- [x] `src/utils/request_utils.rs` was previously untested and is now covered.
- [x] `src/middlewares/conditional_access_middleware.rs` already had 2 tests
      (denied requests don't call downstream, permitted calls downstream once).

The remaining 4 middleware files (`course_permission_middleware`,
`organization_permission_middleware`, `platform_permission_middleware`,
`organization_hierarchy_middleware`, `platform_hierarchy_middleware`) contain
only async database-reliant actix service handlers. Their parameter extraction
logic is now tested via `extract_param`, and their permission-checking behavior
is covered by `tests/middleware_access_control.rs`.

## 5. Config / Database Setup Tests (MEDIUM)

- [ ] `src/config/db_setup/mod.rs` — version updater loop has no tests for
      applying updates in sequence, handling missing rows, or rolling back
      failed updates
- [ ] `src/config/db_setup/updates/update_v1.rs` — initial schema data
      insertion has no correctness verification
- [ ] `src/config/db_setup/updates/update_v2.rs` — same as above

## 6. Utility Tests (LOW)

Several utility modules have no tests despite containing logic that could fail
silently.

- [ ] `src/utils/centralized_wallets.rs` (199 lines) — centralized wallet key
      management, derivation, signing. **No tests.**
- [ ] `src/utils/course_utils.rs` (120 lines) — course/content creation helpers.
      **No tests.**
- [ ] `src/utils/request_utils.rs` — request parsing helpers. **No tests.**
- [ ] `src/utils/eth/compiler.rs` — Solidity compiler wrapper. **No tests.**
- [ ] `src/utils/eth/deployer.rs` — contract deployment. **No tests.**
- [ ] `src/utils/eth/wallet.rs` — wallet loading/key parsing. **No tests.**

## 7. API Handler Gaps (LOW)

- [ ] `src/api/roles.rs` (78 lines) — dedicated test coverage for role CRUD
      endpoints. The `api_routing.rs` test confirms registration but does not
      test behavior.
- [ ] `src/api/users.rs` (208 lines) — user search, password reset, email
      verification. Exercised indirectly but has no dedicated test file.

## 8. Worker Binary Tests (LOW)

- [ ] `src/bin/worker.rs` has `test = false` in Cargo.toml. The upload job
      lifecycle is partially tested in `tests/worker_upload_jobs.rs`, but the
      full video-processing pipeline (ffmpeg, transcode, S3 operations) and
      the worker main loop are untested.
- [ ] `src/bin/abi_export.rs` — untested ABI export tool.
- [ ] `src/bin/mock_email.rs` — untested mock email server.

## 9. Frontend Tests (HIGH)

- [ ] React component tests — **zero** tests for components in
      `web/src/components/`. No Jest/Vitest setup exists.
- [ ] Page-level tests — **zero** tests for pages in `web/src/app/`.
- [ ] E2E tests — **zero** automated Playwright/Cypress tests (only manual QA
      runs documented in [05-testing-migration.md](05-testing-migration.md)).
- [ ] The only frontend tests are API helper contract tests in
      `web/scripts/api-helper-tests.mjs`.

## 10. Model Permission Logic Tests (LOW)

Permission resolution methods (`has_permission`, `effective_permissions`,
`resolve_hierarchy`) embedded in model files lack unit tests.

- [ ] Model hierarchy resolution methods in `src/models/user_role_platform.rs`,
      `user_role_organization.rs`, `user_role_course.rs`
- [ ] Wallet value computation methods in `src/models/wallet.rs`

---

## Current Coverage Baseline (FINAL)

| Area | Tests | Quality |
|------|-------|---------|
| API handlers (integration) | 30+ test files | Good |
| API handlers (unit) | 1 file (auth) | Adequate |
| Services | **18/18 inline tests** | Good — 371 tests |
| Repositories | **5 test files, 35 tests** | Moderate — 9 repos directly tested |
| Middlewares | 2/7 + 11 request_utils | Adequate |
| Utils | **11/13 files** (+centralized_wallets) | Good |
| Models | 0 pure fns (all DB) | Acceptable |
| Config | **2 tests** (updates list) | Minimal |
| Binaries | None (test=false) | Limited |
| Ethereum contracts | 3 test files (44 tests) | Good |
| Frontend | 1 file (api helpers) | Needs components+E2E |

**Totals:**
- Inline unit tests: 69 → **371** (5.4x increase)
- Integration tests: ~40 files → **55 files** (+3 repo test files)
- Solidity tests: 0 → **44** (all passing)
- Blockchain tests: 2/2 now pass (no longer ignored)

**Remaining untestable with pure functions:** eth/compiler (all file I/O), eth/deployer (blockchain RPC), eth/wallet (keystore reading), model permissions (all DB methods), worker binary (test=false). These are covered by integration/E2E tests or blockchain integration tests.

**Last real gap:** frontend component/page/E2E tests (requires Jest/Vitest + Playwright setup).

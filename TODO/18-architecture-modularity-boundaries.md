# Architecture Modularity And Firm Boundaries

This file tracks the backend architecture work needed to move RustLearn from a
technically layered monolith to a modular monolith with explicit business
boundaries. The goal is not "more files"; the goal is fewer reasons for an API
handler, Diesel query, permission rule, and domain workflow to change together.

## Sources Checked First

- Actix Web application docs: `App` registers routes and middleware, stores
  app state, and `Scope` acts as a path namespace for grouped routes.
  Reference: <https://actix.rs/docs/application/>
- Actix Web `web::Data`: shared app data is cheap to clone because it is backed
  by `Arc`, and data can be layered at `App`, `Scope`, or `Resource`.
  Reference: <https://docs.rs/actix-web/latest/actix_web/web/struct.Data.html>
- Actix Web errors: handlers can return `Result<T, E>` where `E` implements
  `ResponseError`, letting the HTTP layer own status-code mapping.
  Reference: <https://actix.rs/docs/errors/>
- Actix Web extractors: typed extractors such as `Json<T>` and configurable
  extractor errors keep request parsing at the HTTP boundary.
  Reference: <https://actix.rs/docs/extractors/>
- Rust Book module system: packages, crates, modules, paths, and visibility are
  the language tools for grouping related functionality, separating distinct
  features, and hiding implementation details.
  Reference: <https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html>
- Rust API Guidelines: use type safety, newtypes, private fields, predictable
  APIs, and stable public surfaces to make boundaries harder to misuse.
  Reference: <https://rust-lang.github.io/api-guidelines/checklist.html>

## Current Problems To Fix

- [ ] Stop adding new `include!`-based service modules. They split files but
      keep one shared module namespace, shared imports, and hidden coupling.
- [ ] Move direct Diesel usage out of API handlers. Examples to migrate include
      chapters, notification preferences, course assessments, user search, and
      content mutations.
- [ ] Separate DB records from API DTOs. Avoid returning Diesel models directly
      from handlers for user-facing contracts.
- [ ] Stop mixing Active Record and Repository patterns. DB methods currently
      live both on `models::*` and in `repositories::*`; converge on repository
      or infra modules.
- [ ] Centralize authorization policy. Permission checks currently live in
      middleware, services, repositories, and frontend helper lists.
- [x] Pull startup side effects out of `main.rs`. DB setup, S3 setup,
      notification state, and Ethereum startup deployment should be composed
      through a small bootstrap module.
- [ ] Give cross-cutting utilities a home based on responsibility. Some
      `utils::*` modules are infrastructure adapters, some are domain helpers,
      and some are application services.
- [ ] Make frontend capability checks consume backend-derived session
      capabilities instead of duplicating backend permission constants.

## Decisions After Self-Criticism

- Rejected: context-first folders such as `rewards/domain`,
  `rewards/application`, `rewards/infra`, `rewards/http`. They make each
  context self-contained, but repeat the same architectural folders everywhere.
- Rejected: top-level rings with only one file per context, such as
  `domain/rewards.rs` and `application/rewards.rs`. That is cleaner than today,
  but not granular enough for rewards, wallet, learning, and access control.
- Rejected: starting with multiple Cargo crates. Crates can enforce boundaries,
  but they add friction before the project has proven the module shape.
- Accepted: top-level rings, stable context names inside every ring, and
  Level 2 granularity inside complex contexts. This keeps navigation simple,
  avoids repeated ring folders, and still makes use cases and domain aggregates
  small enough to test.

## Target Shape

Prefer a modular monolith organized by architectural ring first, then bounded
context. This avoids repeating `domain/application/infra/http` under every
context while still keeping each business area easy to find.

```text
src/
  bootstrap/
    app_state.rs
    startup.rs
    routes.rs
  shared/
    ids.rs
    time.rs
    money.rs
    pagination.rs
  domain/
    access_control/
    identity/
    organizations/
    learning/
    content/
    teacher_applications/
    kyc/
    rewards/
    wallet/
    reporting/
    notifications/
    operations/
  application/
    access_control/
    identity/
    organizations/
    learning/
    content/
    teacher_applications/
    kyc/
    rewards/
    wallet/
    reporting/
    notifications/
    operations/
  infra/
    postgres/
      access_control/
      identity/
      organizations/
      learning/
      content/
      teacher_applications/
      kyc/
      rewards/
      wallet/
      reporting/
      notifications/
      operations/
    object_storage/
      content/
    ethereum/
      rewards/
      wallet/
    email/
      identity/
      teacher_applications/
      notifications/
    notifications/
      dispatcher/
  http/
    extractors/
    errors.rs
    routes.rs
    access_control/
    identity/
    organizations/
    learning/
    content/
    teacher_applications/
    kyc/
    rewards/
    wallet/
    reporting/
    notifications/
    operations/
```

Dependency rule:

```text
http -> application -> domain
infra -> application/domain ports
domain -> no Actix, Diesel, S3, Ethereum, env vars, or HTTP responses
```

Import rule:

```text
domain      may import std + shared + approved pure value crates
application may import std + domain + shared + application/access_control ports
infra       may import application ports + domain + shared + db/schema + external crates
http        may import application + domain ids/enums + shared + Actix
bootstrap   may import every ring because it wires the process together
```

Forbidden imports in new code:

```text
domain      -> actix_web, diesel, diesel_async, aws_*, ethers, std::env
application -> actix_web, diesel schema/query DSL, aws_*, ethers, std::env
http        -> diesel schema/query DSL, repositories, infra/postgres internals
infra       -> http DTOs, Actix handlers
shared      -> any business context module
```

Approved pure value crates are crates that do not perform I/O, runtime wiring,
database access, network access, or framework integration. Prefer wrapping them
in `shared` value types when the same concept crosses several contexts.

Navigation rule:

```text
Start with application/<context>/<use_case>.rs.
Use domain/<context> for pure rules and types.
Use infra/<adapter>/<context> for external systems.
Use http/<context> only for Actix routing, DTOs, extractors, and response mapping.
```

Context naming rule:

```text
If a context exists in more than one ring, keep the same context directory name
in every ring. Prefer application/rewards + domain/rewards + http/rewards over
reward_candidate_service + reward_execution_service + reward_history_service.
```

Canonical context list:

```text
identity              users, authentication, sessions, email verification, password reset
access_control        roles, permissions, delegation, hierarchy decisions
organizations         organizations, members, organization dashboard ownership
learning              courses, enrollment, progress, assessment attempts
content               chapters, course content, upload jobs, media processing
teacher_applications  teacher application nomination, review, audit
kyc                   KYC submission, review, audit
rewards               candidates, policies, fraud blocks, payouts, compensation, history
wallet                wallets, deposits, transfers, token ledger, wallet audit
reporting             read models, dashboard summaries, CSV exports, reconciliation views
notifications         preferences, notification records, delivery decisions
operations            health, readiness, startup checks, persistent runtime state
```

Completeness rule:

```text
Every canonical context should appear in every ring where it owns behavior.
If a context is omitted from a ring, the omission must be intentional. Example:
reporting may have little or no domain logic, but it still belongs in
application/reporting, infra/postgres/reporting, and http/reporting.
```

Granularity levels:

- Level 1: top-level rings only. Example: `domain/rewards.rs` and
  `application/rewards.rs`. This is better than the current service pile, but
  still becomes crowded as a context grows.
- Level 2, preferred: top-level rings, context folders, then aggregate or
  use-case folders. This gives short files and precise ownership without
  repeating the four architectural rings under every context.
- Level 3, optional later: split rings into Cargo crates or workspace packages
  when compiler-enforced boundaries become worth the extra build and dependency
  management cost.

Complete Level 2 shape for the application:

```text
src/
  domain/
    identity/
      user/
      credential/
      session/
    access_control/
      permission/
      role/
      delegation/
      hierarchy/
    organizations/
      organization/
      member/
      audit/
    learning/
      course/
      enrollment/
      progress/
      assessment/
    content/
      chapter/
      content_item/
      upload_job/
    teacher_applications/
      application/
      decision/
      audit/
    kyc/
      submission/
      decision/
      audit/
    rewards/
      candidate/
      policy/
      fraud_block/
      payout/
      compensation/
    wallet/
      wallet/
      deposit/
      transfer/
      audit/
    reporting/
      read_model/
    notifications/
      preference/
      message/
    operations/
      health/
      startup/
  application/
    identity/
      register_user/
      login_with_password/
      request_password_reset/
      verify_email/
      current_session/
      ports.rs
    access_control/
      authorize_action/
      grant_delegated_permission/
      revoke_delegated_permission/
      assign_role/
      ports.rs
    organizations/
      create_organization/
      update_organization/
      list_members/
      manage_member/
      organization_dashboard/
      ports.rs
    learning/
      discover_courses/
      manage_course/
      request_enrollment/
      decide_enrollment/
      save_progress/
      list_course_assessments/
      list_assessment_attempts/
      submit_assessment_attempt/
      ports.rs
    content/
      manage_chapter/
      manage_content_item/
      request_upload_url/
      process_upload_job/
      ports.rs
    teacher_applications/
      submit_application/
      nominate_application/
      list_applications/
      decide_application/
      ports.rs
    kyc/
      submit_kyc/
      review_kyc/
      list_kyc_audit/
      ports.rs
    rewards/
      submit_candidate/
      decide_teacher_candidate/
      decide_amount/
      manage_reward_policy/
      manage_fraud_block/
      plan_payout/
      confirm_token_payout/
      credit_wallet/
      reconcile_candidate/
      list_reward_history/
      ports.rs
    wallet/
      link_wallet/
      create_deposit_intent/
      index_deposit/
      transfer_tokens/
      audit_wallet/
      ports.rs
    reporting/
      platform_summary/
      organization_summary/
      reward_dashboard/
      fraud_dashboard/
      export_csv/
      ports.rs
    notifications/
      save_preferences/
      list_notifications/
      mark_notification_read/
      send_notification/
      ports.rs
    operations/
      readiness_check/
      startup_contracts/
      ports.rs
  infra/
    postgres/
      identity/
        user_store.rs
        auth_store.rs
        token_store.rs
        session_read_store.rs
        mappers.rs
      access_control/
        role_store.rs
        permission_store.rs
        delegation_store.rs
        hierarchy_store.rs
        mappers.rs
      organizations/
        organization_store.rs
        member_store.rs
        audit_store.rs
        dashboard_read_store.rs
        mappers.rs
      learning/
        course_store.rs
        enrollment_store.rs
        progress_store.rs
        assessment_read_store.rs
        assessment_submission_store.rs
        assessment_store.rs
        mappers.rs
      content/
        chapter_store.rs
        content_item_store.rs
        upload_scope_store.rs
        upload_job_store.rs
        mappers.rs
      teacher_applications/
        application_store.rs
        audit_store.rs
        mappers.rs
      kyc/
        submission_store.rs
        audit_store.rs
        mappers.rs
      rewards/
        candidate_store.rs
        policy_store.rs
        audit_store.rs
        fraud_block_store.rs
        execution_job_store.rs
        payout_record_store.rs
        compensation_store.rs
        mappers.rs
      wallet/
        wallet_store.rs
        transaction_store.rs
        deposit_intent_store.rs
        wallet_credit_store.rs
        mappers.rs
      reporting/
        platform_reports.rs
        organization_reports.rs
        csv_exports.rs
        reconciliation_queries.rs
      notifications/
        preference_store.rs
        notification_store.rs
        mappers.rs
      operations/
        persistent_state_store.rs
        readiness_check.rs
        readiness_queries.rs
    object_storage/
      content/
        upload_url_provider.rs
        media_object_store.rs
      operations/
        readiness_check.rs
    ethereum/
      rewards/
        reward_contract_gateway.rs
      wallet/
        wallet_transfer_gateway.rs
      operations/
        readiness_check.rs
        startup_contract_deployer.rs
    email/
      identity/
        verification_mailer.rs
        password_reset_mailer.rs
      teacher_applications/
        teacher_application_mailer.rs
      notifications/
        notification_mailer.rs
    notifications/
      dispatcher.rs
      channel_router.rs
  http/
    identity/
      routes.rs
      dto/
      handlers/
      error.rs
    access_control/
      routes.rs
      dto/
      handlers/
      error.rs
    organizations/
      routes.rs
      dto/
      handlers/
      error.rs
    learning/
      routes.rs
      dto/
      handlers/
      error.rs
    content/
      routes.rs
      dto/
      handlers/
      error.rs
    teacher_applications/
      routes.rs
      dto/
      handlers/
      error.rs
    kyc/
      routes.rs
      dto/
      handlers/
      error.rs
    rewards/
      routes.rs
      dto/
      handlers/
      error.rs
    wallet/
      routes.rs
      dto/
      handlers/
      error.rs
    reporting/
      routes.rs
      dto/
      handlers/
      error.rs
    notifications/
      routes.rs
      dto/
      handlers/
      error.rs
    operations/
      routes.rs
      dto/
      handlers/
      error.rs
```

Detailed Level 2 shape for one large context, using rewards as the pilot:

```text
src/
  domain/
    rewards/
      candidate/
        mod.rs
        id.rs
        status.rs
        event_type.rs
        transition.rs
        error.rs
      policy/
        mod.rs
        scope.rs
        payment_strategy.rs
      fraud_block/
        mod.rs
        scope.rs
        rule.rs
  application/
    rewards/
      submit_candidate/
        mod.rs
        command.rs
        handler.rs
        error.rs
      decide_teacher_candidate/
        mod.rs
        command.rs
        handler.rs
        error.rs
      decide_amount/
        mod.rs
        command.rs
        handler.rs
        error.rs
      reconcile_candidate/
        mod.rs
        command.rs
        handler.rs
        error.rs
      ports.rs
  infra/
    postgres/
      rewards/
        candidate_store.rs
        policy_store.rs
        audit_store.rs
        fraud_block_store.rs
        mappers.rs
  http/
    rewards/
      routes.rs
      dto/
        submit_candidate.rs
        decide_teacher_candidate.rs
        decide_amount.rs
      handlers/
        submit_candidate.rs
        decide_teacher_candidate.rs
        decide_amount.rs
      error.rs
```

Optional Level 3 shape if module boundaries still feel too soft:

```text
crates/
  rust_learn_domain/
  rust_learn_application/
  rust_learn_infra_postgres/
  rust_learn_infra_ethereum/
  rust_learn_http_actix/
  rust_learn_bootstrap/
```

Level 3 dependency graph, if adopted later:

```text
rust_learn_domain       -> shared primitives only
rust_learn_application  -> rust_learn_domain
rust_learn_infra_*      -> rust_learn_application + rust_learn_domain
rust_learn_http_actix   -> rust_learn_application + rust_learn_domain
rust_learn_bootstrap    -> all crates; owns wiring only
```

Do not start at Level 3. First prove Level 2 on rewards; move to crates only if
imports keep crossing boundaries accidentally after the module layout is clean.

Module boundary enforcement starts with review and `rg` checks, then can become
compiler-enforced crates later. A useful preflight once the layout exists:

```bash
rg "actix_web|diesel|diesel_async|aws_|ethers|std::env" src/domain src/application
rg "crate::repositories|crate::infra::postgres|crate::db::schema" src/http
rg "crate::http" src/domain src/application src/infra
```

Allowed responsibilities:

- `http`: Actix routes, extractors, request DTOs, response DTOs, `ResponseError`
  mappings, route configuration.
- `application`: use cases, transactions, orchestration, authorization calls,
  idempotency, audit event creation, notification decisions.
- `domain`: status enums, value objects, invariants, pure decisions, transition
  rules, calculation helpers.
- `infra`: Diesel repositories, S3/Ethereum/email implementations, persistent
  state adapters.
- `bootstrap`: application state construction, startup checks, top-level route
  assembly, background worker composition.
- `shared`: tiny cross-context primitives only. Do not make it a dumping ground
  for business rules.

Wiring rule:

- `bootstrap` owns concrete wiring. It builds application services from infra
  adapters and exposes those services through `web::Data`.
- `http` handlers receive application services from state. They should not
  construct Diesel repositories or Ethereum/S3 clients directly.
- `application` use cases should be generic over ports or receive small service
  structs that hide concrete infra types.

## Actix-Specific Refactor Rules

- [ ] Each HTTP context exposes one route configurator from `http/<context>`,
      e.g. `pub fn configure_routes(cfg: &mut web::ServiceConfig)`.
- [ ] `api::api_scope()` should only compose context route configurators and
      cross-cutting middleware.
- [ ] Build an `AppState` or context-specific state structs once outside the
      `HttpServer::new` closure, then pass them with `web::Data`.
- [ ] Prefer typed extractors over manual `HttpRequest` parsing where possible:
      `AuthUser`, `DbConn`, `Json<T>`, `Path<T>`, and `Query<T>`.
- [ ] API handlers return `Result<web::Json<T>, ApiError>` or equivalent,
      instead of manually matching every service error to `HttpResponse`.
- [ ] Domain/application errors do not implement Actix traits directly. The
      HTTP layer maps them into a local `ResponseError` type.
- [ ] Configure JSON limits and JSON parse errors centrally so every route has
      consistent bad-request behavior.

## First Low-Risk Building Blocks

- [x] Move or wrap the existing `src/utils/api_error.rs` helper into
      `http/errors.rs`, then evolve it into one Actix `ResponseError`
      implementation with consistent JSON error bodies.
- [x] Add an `http/extractors/auth_user.rs` extractor that reads the `UserJWT`
      extension once and removes repeated `authenticated_user(&req)` boilerplate.
- [ ] Add a DB connection extractor or helper so handlers do not repeat
      `pool.get().await` and 500 mapping.
- [x] Add `bootstrap/app_state.rs` to group `DbPool`, `S3State`,
      `NotificationsState`, and future infra handles.
- [ ] Replace `include!` in one small module with normal `mod` files and
      `pub(crate) use` re-exports to establish the house pattern.

## Initial Implementation Slices

Slice 1: HTTP primitives without route behavior changes.

- [x] Create `src/http/mod.rs`, `src/http/errors.rs`, and
      `src/http/extractors/auth_user.rs`.
- [x] Reuse the current `src/utils/api_error.rs` response envelope instead of
      creating a second envelope.
- [x] Keep old helpers available while new handlers migrate.
- [x] Add unit tests for status code, error code, JSON body, missing auth,
      malformed auth, and extension-based auth.

Slice 2: migrate one simple legacy API area.

- [x] Use chapters as the first API migration because it is small, route-scoped,
      and currently queries Diesel directly in the handler.
- [x] Create `application/content/manage_chapter` after deciding chapters belong
      to content ownership for this migration slice.
- [x] Move Diesel queries behind `infra/postgres/.../chapter_store.rs`.
- [x] Keep existing route paths and permission middleware unchanged for the
      first pass.
- [x] Prove no behavior change with the existing host test wrapper or focused
      integration tests.

Slice 3: extract pure reward domain before reward persistence.

- [x] Create `domain/rewards/candidate/status.rs`,
      `domain/rewards/candidate/event_type.rs`, and
      `domain/rewards/candidate/transition.rs`.
- [x] Cover legal and illegal candidate transitions with pure unit tests.
- [x] Do not move Diesel candidate queries until the domain transition tests
      make the target behavior explicit.
- [ ] Only after that, introduce `application/rewards/<use_case>` handlers.

Slice 4: migrate notification preferences.

- [x] Use `/api/me/preferences` as the next API migration because it is
      session-scoped, small, and directly queried Diesel in the handler.
- [x] Create separate `application/notifications/get_preferences` and
      `application/notifications/save_preferences` use cases.
- [x] Move preference Diesel queries behind
      `infra/postgres/notifications/notification_preference_store.rs`.
- [x] Move notification preference request/response contracts into
      `http/notifications/dto`.
- [x] Keep existing route paths and authentication behavior unchanged.
- [x] Prove default preferences and save/reload behavior with a focused
      route-level regression test.

Slice 5: migrate access-control role catalog reads.

- [x] Use `/roles`, `/roles/organization`, and `/roles/course` as the next API
      migration because they are small access-control catalog reads and queried
      Diesel directly in the handler.
- [x] Create `application/access_control/list_roles` and a `RoleCatalogStore`
      port.
- [x] Move platform, organization, and course role catalog Diesel queries
      behind `infra/postgres/access_control/role_catalog_store.rs`.
- [x] Move role catalog response contracts into `http/access_control/dto`.
- [x] Keep existing route paths and platform permission middleware unchanged.
- [x] Prove platform, organization, and course role list behavior with a
      focused route-level regression test.

Slice 6: migrate identity user profile reads and search.

- [x] Use `/user` and `/user/{id}` as the next API migration because user
      search was a known direct-Diesel hotspot and identity owns users.
- [x] Create `application/identity/list_users`,
      `application/identity/get_user_profile`, and a `UserProfileStore` port.
- [x] Move user list, search, and lookup Diesel queries behind
      `infra/postgres/identity/user_profile_store.rs`.
- [x] Move user profile response contracts into `http/identity/dto`.
- [x] Keep existing route paths and read-permission behavior unchanged.
- [x] Prove list, search, self-read, and forbidden read behavior with a focused
      route-level regression test.

Slice 7: modularize the Actix binary entrypoint.

- [x] Create `bootstrap/app_state.rs` to group the DB pool, S3 state, and
      notifications state.
- [x] Move startup side effects out of `main.rs` into `bootstrap/startup.rs`:
      DB pool creation, S3 setup, DB versioning, and Ethereum startup
      deployment.
- [x] Move top-level route composition and the root index route into
      `bootstrap/routes.rs`.
- [x] Keep `main.rs` as a thin process runner: env/logging, app state
      initialization, server factory, bind, run.
- [x] Prove bootstrap route composition with a focused routing test.

Slice 8: migrate learning assessment reads.

- [x] Use `GET /courses/{id}/assessments` and
      `GET /courses/{id}/assessments/{assessment_id}/attempts` as the next API
      migration because they are small learning-context reads and queried
      Diesel directly in the handlers.
- [x] Create `application/learning/list_course_assessments`,
      `application/learning/list_assessment_attempts`, and an
      `AssessmentReadStore` port.
- [x] Move published-assessment and user-attempt Diesel queries behind
      `infra/postgres/learning/assessment_read_store.rs`.
- [x] Move assessment read response contracts into `http/learning/dto`.
- [x] Keep existing route paths, authentication behavior, JSON field names, and
      attempt ordering unchanged.
- [x] Self-critique: leave `submit_assessment_attempt` as a separate migration
      slice because it owns scoring, max-attempt checks, writes, and likely
      transaction semantics. Mixing it into a read slice would blur the
      boundary instead of improving it.
- [x] Prove published-only assessment listing, user-scoped attempt history, and
      descending attempt order with a focused route-level regression test.

Slice 9: migrate learning assessment submission.

- [x] Use `POST /courses/{id}/assessments/{assessment_id}/submit` as the next
      migration because it was the remaining assessment route with scoring,
      max-attempt checks, question reads, and attempt writes inside the API
      handler.
- [x] Move scoring into pure `domain/learning/assessment` rules so answer
      trimming, unanswered questions, total points, and zero-point percentage
      behavior are testable without Actix or Diesel.
- [x] Create `application/learning/submit_assessment_attempt` and an
      `AssessmentSubmissionStore` port.
- [x] Move published assessment lookup, completed-attempt counting, question
      reads, and completed-attempt insertion behind
      `infra/postgres/learning/assessment_submission_store.rs`.
- [x] Move submit request and response contracts into `http/learning/dto`.
- [x] Preserve existing HTTP behavior: missing assessment returns 404,
      exhausted attempts return 403, lookup failures still say
      `Failed to load assessment`, save failures still say
      `Failed to save attempt`, and `started_at` remains DB-defaulted.
- [x] Self-critique: do not introduce a transaction or idempotency policy in
      this architecture-only slice because the legacy route did not have one.
      A later product/consistency slice can make that behavior explicit.
- [x] Prove scoring, persisted attempt shape, and max-attempt enforcement with a
      focused route-level regression test.

Slice 10: migrate operations health/readiness checks.

- [x] Use `/health` and `/ready` as the next migration because readiness was a
      small operations route with a direct Diesel ping and concrete S3/Ethereum
      checks inside the API handler.
- [x] Create `application/operations/readiness_check` and a
      `ReadinessDependency` port.
- [x] Move readiness response decisions into application code while keeping
      concrete dependency pings out of application.
- [x] Move the Postgres ping behind
      `infra/postgres/operations/readiness_check.rs`.
- [x] Move object-storage and Ethereum readiness adapters behind
      `infra/object_storage/operations/readiness_check.rs` and
      `infra/ethereum/operations/readiness_check.rs`.
- [x] Move liveness/readiness response DTOs into `http/operations/dto`.
- [x] Preserve existing route paths and JSON strings: `/health` returns
      `{"status":"ok"}`, `/ready` returns `ready`/`not_ready`, and dependency
      checks use `ok`/`failed` with optional messages.
- [x] Self-critique: keep concrete adapter construction in the legacy
      `api::health` wrapper for now because full bootstrap wiring of operation
      services is a larger route-composition cleanup. The handler no longer
      owns Diesel, timeout orchestration, S3 calls, or Ethereum provider calls.
- [x] Prove both liveness and full readiness behavior with the existing focused
      route-level regression test.

Slice 11: migrate content item lifecycle reads and mutations.

- [x] Use content item list/create/update/delete as the next migration because
      these route handlers directly queried `chapters`, `contents`, and
      `user_role_course` while returning Diesel `Content` records.
- [x] Create `application/content/manage_content_item` and a `ContentItemStore`
      port.
- [x] Move chapter scope validation, content scope validation, content
      list/create/update/delete persistence, and course recipient lookup behind
      `infra/postgres/content/content_item_store.rs`.
- [x] Move content item request/response contracts into `http/content/dto`.
- [x] Keep upload URL generation, media URL generation, and video processing as
      later slices because those involve object-storage and worker boundaries.
- [x] Preserve existing route paths, JSON fields, wrong-course chapter 404s,
      and optional content-published notification behavior.
- [x] Self-critique: notification delivery still happens in the legacy HTTP
      wrapper because the existing notification state is concrete app data.
      A later content/notifications slice should move delivery behind an
      application notification port.
- [x] Prove create/list/update/wrong-scope behavior with the existing focused
      course content lifecycle regression test.

Slice 12: migrate content upload URL requests.

- [x] Use `POST /courses/{id}/chapters/{chapter_id}/contents/upload_url` as
      the next migration because it combines chapter scope validation, object
      key construction, bucket preparation, and presigned upload URL generation
      inside the API handler.
- [x] Create `application/content/request_upload_url`,
      `ContentUploadScopeStore`, and `ContentUploadUrlProvider` ports.
- [x] Move chapter scope validation for upload URL requests behind
      `infra/postgres/content/upload_scope_store.rs`.
- [x] Move bucket preparation and presigned PUT URL generation behind
      `infra/object_storage/content/upload_url_provider.rs`.
- [x] Move upload URL request/response contracts into `http/content/dto`.
- [x] Preserve existing route path, `content_type is required` validation,
      object key format, `course-materials` bucket, one-hour expiry, S3 env
      fallback, and response fields `upload_url`/`object_key`.
- [x] Self-critique: media URL generation and video processing still have
      direct API coupling and remain separate slices because they include
      content lookup, presigned GETs, and upload-job/worker behavior.
- [x] Prove object key construction and chapter-before-content-type validation
      with pure use-case tests, and prove the real route with the video upload
      flow regression test.

Progress evidence from 2026-06-12 and 2026-06-13:

- `src/api/chapters.rs` no longer contains Diesel query builders; it delegates
  to `application/content/manage_chapter` and `infra/postgres/content/chapter_store`.
- Chapter HTTP request/response DTOs now live under `http/content/dto`, while
  application output remains a non-Actix, non-Serialize type.
- `src/api/session/notifications.rs` no longer contains direct Diesel usage for
  notification preferences; it delegates preference reads and writes through
  `application/notifications` and `infra/postgres/notifications`.
- Notification preference HTTP request/response DTOs now live under
  `http/notifications/dto`.
- `src/api/roles.rs` no longer contains direct Diesel usage for role catalog
  reads; it delegates platform, organization, and course role lists through
  `application/access_control/list_roles` and `infra/postgres/access_control`.
- Role catalog HTTP response DTOs now live under `http/access_control/dto`.
- `src/api/users.rs` no longer contains direct Diesel usage or imports the
  Diesel `User` record for user reads; it delegates list/search/profile reads
  through `application/identity` and `infra/postgres/identity`.
- User profile HTTP response DTOs now live under `http/identity/dto`.
- `src/main.rs` is now 35 lines and delegates app state initialization and
  route composition to `bootstrap/startup.rs`, `bootstrap/app_state.rs`, and
  `bootstrap/routes.rs`.
- `src/api/courses/list_assessment_attempts.rs` no longer contains direct
  Diesel usage; it delegates user attempt history through
  `application/learning/list_assessment_attempts` and
  `infra/postgres/learning/assessment_read_store.rs`.
- The read side of `src/api/courses/list_course_assessments.rs` no longer
  contains direct Diesel usage; it delegates published assessment listing
  through `application/learning/list_course_assessments` and
  `infra/postgres/learning/assessment_read_store.rs`.
- Assessment read HTTP response DTOs now live under `http/learning/dto`.
- `src/api/courses/list_course_assessments.rs` no longer contains direct Diesel
  usage for either assessment reads or assessment submission; it delegates
  submission through `application/learning/submit_assessment_attempt` and
  `infra/postgres/learning/assessment_submission_store.rs`.
- Assessment scoring rules now live under `domain/learning/assessment`.
- Assessment submit request/response DTOs now live under `http/learning/dto`.
- `src/api/health.rs` no longer contains direct Diesel, timeout orchestration,
  object-storage health calls, or Ethereum provider calls; it delegates
  readiness checks through `application/operations/readiness_check` and
  infra adapters.
- Operations liveness/readiness DTOs now live under `http/operations/dto`.
- Content item list/create/update/delete route handlers no longer contain
  direct Diesel calls or return Diesel content records; they delegate through
  `application/content/manage_content_item` and
  `infra/postgres/content/content_item_store.rs`.
- Content item HTTP request/response DTOs now live under `http/content/dto`.
- The content upload URL route no longer owns direct chapter-scope Diesel,
  bucket preparation, presigned PUT generation, or JSON response construction;
  it delegates through `application/content/request_upload_url`,
  `infra/postgres/content/upload_scope_store.rs`, and
  `infra/object_storage/content/upload_url_provider.rs`.
- Content upload URL HTTP request/response DTOs now live under
  `http/content/dto`.
- `rg "actix_web|diesel|diesel_async|aws_|ethers|std::env" src/domain src/application`
  returns no matches.
- `rg "crate::repositories|crate::infra::postgres|crate::db::schema" src/http`
  returns no matches.
- `rg "crate::http|crate::services" src/domain src/application src/infra`
  returns no matches.
- `rg "diesel|diesel_async|schema::|RunQueryDsl" src/api/roles.rs` returns no
  matches.
- `rg "diesel|diesel_async|schema::|RunQueryDsl|QueryDsl|ExpressionMethods|models::user::User" src/api/users.rs`
  returns no matches.
- `rg "diesel|diesel_async|RunQueryDsl|assessment_attempts::table|assessments::table" src/api/courses/list_assessment_attempts.rs`
  returns no matches.
- `rg "diesel|diesel_async|RunQueryDsl|assessment_attempts::table|assessment_questions::table|assessments::table|crate::db::schema" src/api/courses/list_course_assessments.rs src/api/courses/list_assessment_attempts.rs`
  returns no matches.
- `rg "diesel|diesel_async|RunQueryDsl|try_get_provider|provider|get_chainid|health_check\\(|timeout|crate::db::schema" src/api/health.rs`
  returns no matches.
- `rg "diesel|diesel_async|insert_into|update\\(|delete\\(|contents::table|user_role_course|NewContent\\b|UpdateContent\\b|models::content" src/api/contents/create_content.rs src/api/contents/get_upload_url.rs`
  returns no matches.
- `rg "ensure_bucket|presign_external_put|S3State::new_from_env|ensure_chapter_belongs_to_course|chapters::table|diesel|diesel_async|RunQueryDsl|serde_json::json" src/api/contents/get_upload_url.rs`
  returns no matches.
- `cargo fmt --all --check` passes.
- `git diff --check` passes.
- `./scripts/run-host-tests.sh cargo test http::` passes.
- `./scripts/run-host-tests.sh cargo test --lib learning` passes.
- `./scripts/run-host-tests.sh cargo test application::access_control` passes.
- `./scripts/run-host-tests.sh cargo test application::identity` passes.
- `./scripts/run-host-tests.sh cargo test domain::rewards::candidate` passes.
- `./scripts/run-host-tests.sh cargo test application::content` passes.
- `./scripts/run-host-tests.sh cargo test application::notifications` passes.
- `./scripts/run-host-tests.sh cargo test role_read_routes_require_view_role_assignments_permission --test middleware_access_control`
  passes.
- `./scripts/run-host-tests.sh cargo test read_user_routes_require_view_user_or_self --test middleware_access_control`
  passes.
- `./scripts/run-host-tests.sh cargo test --test api_routing` passes.
- `./scripts/run-host-tests.sh cargo test assessment_read_routes_are_published_and_user_scoped --test course_assessments`
  passes.
- `./scripts/run-host-tests.sh cargo test assessment_submit_attempt_scores_persists_and_enforces_max_attempts --test course_assessment_submission`
  passes.
- `./scripts/run-host-tests.sh cargo test --lib operations` passes.
- `./scripts/run-host-tests.sh cargo test --test health_readiness` passes.
- `./scripts/run-host-tests.sh cargo test test_course_content_lifecycle --test course_content_management`
  passes.
- `./scripts/run-host-tests.sh cargo test --lib request_upload_url` passes.
- `./scripts/run-host-tests.sh cargo test course_video_upload_can_be_queued_and_processed --test video_upload_flow`
  passes.
- `./scripts/run-host-tests.sh cargo test notification_preferences_default_and_save_round_trip --test current_session_api`
  passes.
- `./scripts/run-host-tests.sh cargo test --test current_session_api`
  passes.
- `./scripts/run-host-tests.sh cargo check --features app-bin --bin rust-learn`
  passes.

## Legacy Transition Rules

- [ ] New ring-based modules may be called from existing `api`, `services`, and
      `repositories` while migration is in progress.
- [ ] New ring-based modules must not call old `services::*` modules.
- [ ] New domain and application modules must not call old `models::*` async DB
      methods. Use a temporary infra adapter if a legacy query still lives
      there.
- [ ] Keep route URLs stable. Move implementation behind the route before
      moving the route file itself.
- [ ] Keep legacy wrappers thin and temporary. Each wrapper should have a TODO
      naming the new module it delegates to and the deletion condition.
- [ ] Do not move files just to satisfy the shape. Move a vertical slice only
      when tests prove behavior before and after.

## Granularity Guardrails

- [ ] Name application modules by use case, not by vague service buckets:
      `submit_candidate`, `decide_amount`, `reconcile_candidate`.
- [ ] Keep `mod.rs` files boring: declare child modules and re-export the
      intentional public surface only.
- [ ] No `imports.rs` files. Each module imports what it actually uses so
      coupling remains visible during review.
- [ ] Prefer `pub(crate)` or narrower visibility by default. Use `pub` only for
      APIs that another ring or binary must call.
- [ ] Start with one `application/<context>/ports.rs` when several use cases
      need the same stores; split ports next to a use case only when that use
      case owns a truly unique external contract.
- [ ] Keep mappers at boundaries: HTTP DTO mapping in `http`, Diesel record
      mapping in `infra/postgres`, domain validation in `domain`.
- [ ] Split a folder only when it names a real business concept or use case.
      Avoid one-file folders unless the module is expected to grow immediately.
- [ ] A use-case folder may start with only `mod.rs`, `command.rs`,
      `handler.rs`, and `error.rs`. Add more files only when the code demands
      it.
- [ ] Prefer explicit modules over clever generic abstractions. Granular means
      easy to locate and test, not abstract for its own sake.

## Pilot Context: Rewards

Use rewards as the first serious extraction because it currently crosses
candidate submission, fraud blocks, reward policy, audit events, wallet credits,
notifications, reporting, and platform review.

- [ ] Create the rewards context across the top-level rings:
      `domain/rewards`, `application/rewards`, `infra/postgres/rewards`, and
      `http/rewards`.
- [ ] Use Level 2 granularity inside rewards: split domain by aggregate
      (`candidate`, `policy`, `fraud_block`) and application by use case
      (`submit_candidate`, `decide_amount`, `reconcile_candidate`).
- [ ] Move reward statuses and event types into domain enums/newtypes. Keep
      database string conversion at the infra boundary.
- [ ] Move reward request/response structs out of service imports and into
      `http/rewards/dto.rs`.
- [ ] Move candidate transition rules into pure domain functions:
      submit, teacher approve/reject, amount approve/reject, token confirmed,
      wallet credited, notified, reconciliation needed.
- [ ] Define repository ports needed by reward use cases before moving Diesel
      code. Examples: `RewardCandidateStore`, `RewardPolicyStore`,
      `RewardAuditStore`, `RewardFraudBlockStore`.
- [ ] Move Diesel implementations behind `infra/postgres/rewards`.
- [ ] Move reward permission decisions through `application/access_control`
      instead of calling `user_permission_*_request` directly from reward use
      cases.
- [ ] Keep the old route paths stable while swapping internals.
- [ ] Add tests at three levels: pure domain transition tests, application
      use-case tests with fake ports, and API regression tests for existing
      routes.

## Access Control Context

- [ ] Create one access-control API for `can(actor, action, scope)` style
      decisions.
- [ ] Encode scope as types instead of loose strings where practical:
      `PlatformScope`, `OrganizationScope`, `CourseScope`, `DelegatedScope`.
- [ ] Make middleware call the same access-control service as application use
      cases.
- [ ] Keep middleware as an early rejection optimization; do not make it the
      only place that protects business actions.
- [ ] Return frontend capabilities from session endpoints so `web/src/lib`
      does not duplicate backend permission groupings.

## Data Boundary Rules

- [ ] Diesel schema types stay in `infra` or legacy repositories only.
- [ ] `models::*` should become persistence records, not business logic owners.
- [ ] New public API response types live in `http/<context>/dto.rs` or
      context-specific contract modules.
- [ ] Use newtypes for IDs and important string states when crossing domain
      boundaries: `CourseId`, `UserId`, `RewardCandidateId`, `PermissionKey`.
- [ ] Keep stringly-typed status conversion at edges: HTTP parsing and DB
      serialization.

## Migration Order

1. Add HTTP primitives: `ApiError`, `AuthUser`, DB connection helper, JSON
   config.
2. Move direct DB queries out of the smallest API modules first, starting with
   chapters and notification preferences.
3. Replace `include!` with normal modules in one context at a time.
4. Introduce `access_control` and route all permission checks through it.
5. Extract rewards as the pilot bounded context across `domain`,
   `application`, `infra/postgres`, and `http`.
6. Extract learning/content after rewards proves the pattern.
7. Extract wallet infrastructure adapters from wallet application use cases.
8. Convert reporting to explicit read-model/query modules rather than general
   business services.
9. Update frontend permission/capability helpers to consume backend capability
   contracts.

## Acceptance Criteria

- [ ] New routes can be added by touching one context plus shared contracts,
      not `api`, `services`, `repositories`, `models`, `utils`, and frontend
      permission lists at once.
- [ ] A domain test can run without Actix, Diesel, S3, Ethereum, env vars, or a
      database pool.
- [ ] A use-case test can run with fake ports for authorization, persistence,
      notifications, and time.
- [ ] API handlers mostly contain extraction, use-case call, and response
      mapping; no complex Diesel query builders.
- [ ] Permission behavior has one backend source of truth.
- [x] `main.rs` is mostly logging/env setup, bootstrap calls, and server start.
- [ ] Existing route URLs and response semantics remain backward compatible
      unless a migration note explicitly says otherwise.
- [ ] New `domain`, `application`, `http`, and `infra` modules pass the import
      checks listed in this file.
- [ ] Every legacy wrapper introduced during migration names its deletion
      condition.
- [ ] No new module uses `include!` or `imports.rs`.
- [ ] Public API DTOs and Diesel records are different types unless a migration
      note explicitly accepts a temporary leak.
- [ ] `cargo fmt --all --check` and relevant host tests pass after each
      context migration.

## Non-Goals

- [ ] Do not split into microservices yet. The current problem is internal
      boundaries, not deployment topology.
- [ ] Do not rewrite all services at once. Use one bounded context as proof,
      then repeat.
- [ ] Do not add abstractions only to hide Diesel. Add ports when they protect
      domain/application code from infrastructure churn or enable useful tests.
- [ ] Do not change product behavior during architecture migration unless the
      behavior is already covered by a separate TODO item.

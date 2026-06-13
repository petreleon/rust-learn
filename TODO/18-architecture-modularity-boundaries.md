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

Level 2 is complete only when the target is context-complete, ring-complete,
adapter-complete, use-case-complete, route-complete, contract-complete,
boundary-complete, wiring-complete, migration-complete, and test-complete:

- Context-complete means every canonical context has an explicit owner and a
  recorded reason for any omitted ring. Empty folders are not completion; owned
  behavior, ports, adapters, and routes are completion.
- Ring-complete means every behavior-owning context has a stable home under
  `domain`, `application`, `infra`, and `http`; no context should remain a
  scattered collection of `api/*`, `services/*`, `repositories/*`, and
  `utils/*` names.
- Adapter-complete means every external system is grouped first by adapter,
  then by context. `infra/postgres` must contain every DB-backed context, not
  only rewards. `infra/object_storage`, `infra/ethereum`, `infra/email`, and
  other adapters follow the same rule for the contexts they serve.
- Use-case-complete means application modules are named after user/business
  actions, not vague service buckets. Prefer `manage_reward_policy`,
  `submit_assessment_attempt`, and `notification_inbox` over
  `reward_service`, `course_service`, or `session_service`.
- Route-complete means every route is owned by `http/<context>`. Legacy `api/*`
  modules may exist only as temporary delegating shims with a deletion note.
- Contract-complete means every use case has a narrow command/query input,
  output type, error type, and port contract. Shared ports may live in
  `application/<context>/ports.rs`; unique ports may live beside the use case.
- Boundary-complete means HTTP DTOs, application command/output types, domain
  values, Diesel records, and provider payloads remain separate types unless a
  migration note explicitly accepts a temporary leak.
- Wiring-complete means `bootstrap` constructs concrete use cases and adapters.
  HTTP handlers receive application services; they do not create Diesel,
  Ethereum, S3, email, or notification adapters.
- Migration-complete means old `services/*`, `repositories/*`, `models/*`, and
  `utils/*` no longer own behavior. Anything left there is either a persistence
  record, a compatibility wrapper with a named deletion condition, or shared
  code waiting on an explicit migration slice.
- Test-complete means the slice has the smallest useful set of pure domain,
  application fake-port, adapter, route, or integration tests needed for its
  risk, plus boundary import scans when new modules are introduced.

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

Level 2 completion matrix:

| Context | Domain owns | Application owns | Postgres owns | Other infra owns | HTTP owns |
| --- | --- | --- | --- | --- | --- |
| `identity` | users, credentials, sessions, verification states | registration, login, password reset, email verification, current session | users, credentials, tokens, session read models | verification and password-reset mailers | auth, session, and user profile routes |
| `access_control` | permissions, roles, scopes, hierarchy, delegation rules | authorization decisions, role assignment, delegated permission grants/revokes | role catalogs, permission checks, hierarchy and delegation stores | none unless an external policy engine is introduced | role, permission, delegation, and capability routes |
| `organizations` | organization, membership, membership audit rules | organization CRUD, member management, dashboards, invitations | organizations, memberships, invites, member audit, dashboard read models | organization notification hooks when needed | organization, member, invite, and dashboard routes |
| `learning` | courses, enrollment, progress, assessment rules | course discovery, lifecycle, enrollment decisions, progress writes, assessment reads/submission | courses, enrollments, progress, assessments, attempts | none by default | course, enrollment, progress, and assessment routes |
| `content` | chapters, content items, upload jobs, media rules | chapter/content item management, upload URL requests, media URL requests, upload job processing | chapters, content items, upload jobs, media metadata | object storage upload/media providers, worker-facing media adapters | chapter, content, media, and upload routes |
| `teacher_applications` | application lifecycle, decisions, audit vocabulary | submit, nominate, list, review, audit application decisions | applications, review decisions, audit events | teacher application mailers | teacher application and audit routes |
| `kyc` | submission lifecycle, review decisions, audit vocabulary | submit KYC, review KYC, list KYC audit | KYC submissions, review decisions, audit events | external KYC provider adapter if added later | KYC submission, review, and audit routes |
| `rewards` | candidates, policies, fraud blocks, payouts, compensation, transition rules | candidate submission, teacher decision, amount decision, policy/fraud management, payout planning, token confirmation, wallet credit, reconciliation, reward history | candidates, policies, fraud blocks, audit, execution jobs, payout records, compensation, wallet credit records, reward read models | Ethereum reward contract gateway, reward notification hooks | reward candidate, policy, fraud block, history, and review routes; payout routes only if exposed |
| `wallet` | wallets, deposits, transfers, token ledger, wallet audit rules | wallet linking, deposit intents, deposit indexing, token transfers, wallet audit | wallets, transactions, deposit intents, wallet credit records | Ethereum wallet transfer gateway | wallet, deposit, transfer, and audit routes |
| `reporting` | report vocabulary and pure aggregation rules only when useful | platform summaries, organization summaries, reward/fraud dashboards, exports, reconciliation views | report queries, reconciliation queries, CSV export read models | file/export sinks if added later | report dashboard and export routes |
| `notifications` | preferences, notification records, delivery decisions | preference reads/writes, inbox actions, notification creation, dispatch planning | preferences, inbox records, outbox records, delivery attempts | dispatcher, channel router, email/push/SMS providers | preference, inbox, and notification routes |
| `operations` | health/startup state vocabulary | readiness checks, startup contract orchestration, operational status | persistent state, readiness queries, version checks | object storage, Ethereum, and runtime readiness adapters | health, readiness, and operational routes |

If a matrix cell is empty in the filesystem, that is a migration gap, not a
target-shape decision. If a context genuinely does not need a ring or adapter,
record that as `none` in this file before omitting the folder.

Minimum Level 2 unit:

```text
domain/<context>/<aggregate>/        pure vocabulary, invariants, transitions
application/<context>/<use_case>/    command/query, handler, output, error
application/<context>/ports.rs       shared external contracts for the context
infra/<adapter>/<context>/           concrete adapter implementation and mappers
http/<context>/dto/                  request/response contracts for route use cases
http/<context>/handlers/             Actix handlers that call application use cases
http/<context>/routes.rs             route composition for the context
bootstrap/                           concrete wiring for routes, workers, and startup
```

Use-case folders are mandatory for business actions. Aggregate folders are
mandatory once a context has more than one domain concept. HTTP folders are
mandatory only for route-backed use cases; worker-only and startup-only use
cases must instead be wired from `bootstrap` or `src/bin/*` and documented in
the context checklist.

Cross-context ownership rule:

- `rewards` owns reward candidates, policy, fraud blocks, payout planning,
  token confirmation, reward compensation, and reconciliation of reward
  execution state.
- `wallet` owns wallet linking, deposits, token transfers, ledger views, and
  wallet audit. Reward wallet credits may be produced by rewards, but wallet
  audit read models belong to wallet unless a reporting view explicitly owns
  the projection.
- `reporting` owns cross-context read models, dashboards, exports, and
  reconciliation views. It reads from other contexts through query adapters; it
  should not become a second home for command-side business rules.
- `notifications` owns notification preferences, inbox records, outbox records,
  dispatch planning, and delivery adapters. Other contexts request
  notifications through application ports instead of inserting provider payloads
  directly.
- `access_control` owns permission vocabulary and authorization decisions. Other
  contexts ask it questions; they do not duplicate permission matrices.

Self-critique gate before marking any Level 2 slice complete:

- Can a new contributor start at `application/<context>/<use_case>` and find
  the command, output, errors, ports, adapter, route, and tests without knowing
  legacy service names?
- Does every cross-context dependency go through a named application port or a
  documented read model?
- Can the domain and application modules compile without Actix, Diesel, S3,
  Ethereum, environment variables, or HTTP DTOs?
- Is `bootstrap` the only place that knows the concrete adapter graph for that
  slice?
- Is any remaining legacy wrapper thin, named, temporary, and covered by a
  deletion condition?

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
      token/
      wallet_credit/
      compensation/
      reconciliation/
    wallet/
      wallet/
      deposit/
      transfer/
      audit/
    reporting/
      read_model/
    notifications/
      preference/
      notification/
      delivery_channel/
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
      request_media_url/
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
      record_token_confirmation/
      record_compensation/
      credit_wallet/
      notify_wallet_credit/
      reconcile_candidate/
      list_reward_history/
      list_candidate_audit/
      list_course_candidates/
      list_platform_candidates/
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
      get_preferences/
      save_preferences/
      notification_inbox/
      send_notification/
      dispatch_notification/
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
        media_object_store.rs
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
        reward_candidate_submission_store.rs
        teacher_reward_candidate_decision_store.rs
        reward_amount_decision_store.rs
        reward_policy_store.rs
        reward_fraud_block_store.rs
        reward_payout_plan_store.rs
        reward_token_confirmation_store.rs
        reward_compensation_store.rs
        reward_wallet_credit_store.rs
        reward_wallet_credit_notification_store.rs
        reward_reconciliation_store.rs
        reward_history_store.rs
        reward_candidate_audit_store.rs
        course_reward_candidate_store.rs
        platform_reward_candidate_store.rs
        *_mappers.rs
        *_use_case.rs
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
        notification_preference_store.rs
        notification_inbox_store.rs
        notification_outbox_store.rs
        mappers.rs
      operations/
        persistent_state_store.rs
        readiness_check.rs
        readiness_queries.rs
    object_storage/
      content/
        upload_url_provider.rs
        media_url_provider.rs
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

Level 2 deep example: rewards.

This is an example of how one complex context becomes granular after the
complete Level 2 target is accepted. It must not be read as "Postgres only has
rewards." Every persistence-owning context in the matrix above gets its own
`infra/postgres/<context>` folder as its slices move. Rewards is merely the
deepest current extraction, not the architectural template for which contexts
exist.

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
        event_type.rs
        payment_strategy.rs
      fraud_block/
        mod.rs
        scope.rs
        rule.rs
      payout/
        mod.rs
        status.rs
      token/
        mod.rs
        event_type.rs
        transaction_type.rs
      wallet_credit/
        mod.rs
        transaction_type.rs
      compensation/
        mod.rs
        rule.rs
      reconciliation/
        mod.rs
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
      manage_reward_policy/
        mod.rs
        command.rs
        query.rs
        handler.rs
        validation.rs
        output.rs
        error.rs
        service.rs
      manage_fraud_block/
        mod.rs
        command.rs
        query.rs
        handler.rs
        error.rs
      plan_payout/
        mod.rs
        command.rs
        handler.rs
        error.rs
      record_token_confirmation/
        mod.rs
        command.rs
        handler.rs
        error.rs
      record_compensation/
        mod.rs
        command.rs
        handler.rs
        error.rs
      credit_wallet/
        mod.rs
        command.rs
        handler.rs
        error.rs
      notify_wallet_credit/
        mod.rs
        command.rs
        handler.rs
        error.rs
      reconcile_candidate/
        mod.rs
        command.rs
        handler.rs
        error.rs
      list_reward_history/
        mod.rs
        query.rs
        handler.rs
        output.rs
        error.rs
      list_candidate_audit/
        mod.rs
        handler.rs
        output.rs
        error.rs
      list_course_candidates/
        mod.rs
        query.rs
        handler.rs
        output.rs
        error.rs
      list_platform_candidates/
        mod.rs
        query.rs
        handler.rs
        output.rs
        error.rs
      ports.rs
  infra/
    postgres/
      rewards/
        reward_candidate_submission_store.rs
        teacher_reward_candidate_decision_store.rs
        reward_amount_decision_store.rs
        reward_policy_store.rs
        reward_policy_mappers.rs
        reward_policy_use_case.rs
        reward_fraud_block_store.rs
        reward_payout_plan_store.rs
        reward_token_confirmation_store.rs
        reward_compensation_store.rs
        reward_wallet_credit_store.rs
        reward_wallet_credit_notification_store.rs
        reward_reconciliation_store.rs
        reward_history_store.rs
        reward_candidate_audit_store.rs
        course_reward_candidate_store.rs
        platform_reward_candidate_store.rs
        *_mappers.rs
        *_use_case.rs
  http/
    rewards/
      routes.rs
      dto/
        submit_candidate.rs
        teacher_decision.rs
        amount_decision.rs
        reward_policy.rs
        fraud_block.rs
        reward_history.rs
        candidate_audit.rs
        course_candidates.rs
        platform_candidates.rs
      handlers/
        submit_candidate.rs
        teacher_decision.rs
        amount_decision.rs
        reward_policy.rs
        fraud_block.rs
        reward_history.rs
        candidate_audit.rs
        course_candidates.rs
        platform_candidates.rs
      error.rs
```

Rewards use cases that are worker-only or execution-only, such as wallet
credit, wallet-credit notification, token confirmation, compensation recording,
and reconciliation, do not need HTTP handlers unless a route is introduced.
They still need application contracts, Postgres adapters, bootstrap or worker
wiring, and focused tests.

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

Do not start at Level 3. First complete Level 2 across the active bounded
contexts; move to crates only if imports keep crossing boundaries accidentally
after the module layout is clean.

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
- [x] Replace `include!` in one small module with normal `mod` files and
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
- [x] Self-critique: the first operations slice kept concrete adapter
      construction in the legacy `api::health` wrapper. Slice 20 removes that
      temporary wiring and makes `api::health` a compatibility wrapper.
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

Slice 13: migrate content media URL requests.

- [x] Use `GET /courses/{id}/chapters/{chapter_id}/contents/{id}/media` as
      the next migration because it combines chapter scope validation, content
      lookup, object-key validation, S3 fallback setup, presigned media URL
      generation, and ad hoc JSON construction inside the API handler.
- [x] Create `application/content/request_media_url`,
      `ContentMediaStore`, and `ContentMediaUrlProvider` ports.
- [x] Move chapter/content lookup for media URL requests behind
      `infra/postgres/content/media_object_store.rs`.
- [x] Move presigned GET URL generation and lazy S3 env fallback behind
      `infra/object_storage/content/media_url_provider.rs`.
- [x] Move the media URL response contract into `http/content/dto`.
- [x] Preserve existing route path, `course-materials` bucket, one-hour expiry,
      missing data/object-key error, invalid object-key prefix error, S3 env
      fallback, and response field `url`.
- [x] Self-critique: video processing still has direct API coupling and remains
      a separate slice because it includes authenticated user extraction,
      video-only validation, upload-job insertion, and worker handoff behavior.
- [x] Prove chapter-before-content lookup, object-key prefix validation, and URL
      provider calls with pure use-case tests, and prove the real route with
      the S3-backed video upload flow regression test.

Slice 14: migrate content video-processing job queueing.

- [x] Use `POST /courses/{id}/chapters/{chapter_id}/contents/{id}/process`
      as the next migration because it combines authenticated actor extraction,
      chapter scope validation, content lookup, video-only validation,
      object-key validation, upload-job insertion, and HTTP response mapping
      inside the API handler.
- [x] Create `domain/content/content_item` for the pure video content-type rule.
- [x] Create `application/content/process_upload_job` and a
      `ContentProcessingJobStore` port.
- [x] Move chapter/content lookup and upload-job insertion behind
      `infra/postgres/content/upload_job_store.rs`.
- [x] Preserve existing route path, authentication behavior, `course-materials`
      bucket, accepted body `Video processing queued`, non-video error,
      missing data/object-key error, invalid object-key prefix error, and
      upload job `user_id`.
- [x] Self-critique: the background worker still calls concrete S3,
      notifications, and `models::upload_job` methods directly. That remains a
      separate worker-composition slice because this endpoint only queues work.
- [x] Prove validation order, video-type validation, object-key scoping, and
      queued job shape with pure use-case/domain tests, and prove the real
      route with the S3-backed video upload flow regression test.

Slice 15: replace content API include-based module assembly.

- [x] Use `src/api/contents.rs` as the first include cleanup because all
      content handlers now delegate to Level 2 use cases and adapters, making
      the wrapper small enough to convert safely.
- [x] Replace `include!("contents/...")` with normal sibling modules:
      `create_content`, `get_upload_url`, `get_media_url`, and
      `process_content`.
- [x] Delete `src/api/contents/imports.rs`; each content handler module now
      imports the dependencies it actually uses.
- [x] Move route configuration into the parent `src/api/contents.rs` module so
      route wiring no longer lives inside the media URL handler file.
- [x] Keep content handlers visible only to the parent module with
      `pub(super)` instead of widening them to public API.
- [x] Preserve all existing content route paths and permission middleware.
- [x] Self-critique: this establishes the explicit-module pattern for one
      route context only. Other legacy API/service modules still use
      `include!` and should be converted context by context after their
      behavior is behind use-case boundaries.
- [x] Prove the route configuration still compiles, contains no local
      include/imports bucket, and still passes the focused content lifecycle
      and video upload route tests.

Slice 16: move content HTTP routes and handlers into the HTTP ring.

- [x] Use content as the first HTTP-ring route migration because its handlers
      already delegate to `application/content` use cases and
      `infra/*/content` adapters.
- [x] Create `http/content/routes.rs` with
      `configure_routes(cfg: &mut web::ServiceConfig)`.
- [x] Move content handlers from `api/contents/*` into
      `http/content/handlers/*`.
- [x] Keep handler visibility limited to `crate::http::content` instead of
      exporting handlers crate-wide.
- [x] Leave `api::contents::config` as a thin compatibility wrapper that
      delegates to `http::content::routes::configure_content_item_routes`.
- [x] Preserve all existing content route paths and permission middleware.
- [x] Self-critique: `api::courses::course_scope` still calls the compatibility
      wrapper. A later course-route composition slice should call
      `http::content::routes::configure_routes` directly while untangling the
      remaining course include module. Content HTTP handlers also still
      construct concrete infra adapters; a later bootstrap/state wiring slice
      should move that adapter construction out of `http`.
- [x] Prove route composition and focused content flows still pass, and prove
      the old `api/contents` handler files no longer exist.

Slice 17: move chapter HTTP routes and handlers into the content HTTP ring.

- [x] Use chapter routes as the next HTTP-ring migration because chapters are
      part of the content context and already delegate to
      `application/content/manage_chapter` plus
      `infra/postgres/content/chapter_store.rs`.
- [x] Move chapter handlers from `src/api/chapters.rs` into
      `http/content/handlers/chapter.rs`.
- [x] Move chapter route wiring from `src/api/chapters/routes.rs` into
      `http/content/routes.rs`.
- [x] Split content route composition into `configure_chapter_routes`,
      `configure_content_item_routes`, and full `configure_routes` so current
      legacy wrappers do not register duplicate routes.
- [x] Leave `api::chapters::config` as a thin compatibility wrapper that
      delegates to `http::content::routes::configure_chapter_routes`.
- [x] Keep chapter handler visibility limited to `crate::http::content`.
- [x] Preserve existing chapter route paths, permission middleware, response
      bodies, and focused content lifecycle behavior.
- [x] Self-critique: `api::courses::course_scope` still composes
      `api::chapters::config` and `api::contents::config` separately. A later
      course-route cleanup should compose `http::content::routes::configure_routes`
      directly once the course module is also being untangled. The content HTTP
      handlers still import concrete infra adapters until content app-service
      state is wired through bootstrap.
- [x] Prove route composition and chapter/content lifecycle tests still pass,
      and prove the chapter HTTP handler no longer imports Diesel/schema types.

Slice 18: inject chapter use cases into HTTP instead of constructing Postgres.

- [x] Use chapter handlers as the first content wiring cleanup because they
      have one Postgres adapter and no object-storage or notification side
      effects.
- [x] Add an application-facing `ChapterUseCases` trait under
      `application/content/manage_chapter`.
- [x] Add `infra/postgres/content/chapter_use_cases.rs` as the concrete
      DbPool-backed implementation.
- [x] Add the chapter use-case trait object to `bootstrap::AppState` and
      production Actix app data.
- [x] Update focused route tests to provide the same chapter use-case app data
      when constructing `api::courses::course_scope()` directly.
- [x] Remove direct `DbPool`, `PostgresChapterStore`, and
      `infra/postgres/content/chapter_store` imports from
      `http/content/handlers/chapter.rs`.
- [x] Preserve existing DB connection failure response body:
      `Failed to get DB connection`.
- [x] Self-critique: other content HTTP handlers still construct concrete
      Postgres/S3/notification adapters. Repeat this injected-use-case pattern
      for content item lifecycle, upload URLs, media URLs, and processing jobs.
- [x] Prove chapter/content lifecycle and video upload flows still pass, and
      prove the chapter handler no longer imports concrete infra or `DbPool`.

Slice 19: complete content HTTP use-case injection.

- [x] Add application-facing service traits for content item lifecycle, upload
      URL requests, media URL requests, and video-processing queueing.
- [x] Add concrete DbPool/S3-backed implementations under
      `infra/postgres/content` so adapter construction happens outside HTTP.
- [x] Wire those implementations through `bootstrap::AppState` and production
      Actix app data.
- [x] Update content HTTP handlers to extract application-facing use cases
      instead of `DbPool`, Postgres stores, or S3 providers.
- [x] Preserve existing DB connection and S3 init failure response bodies.
- [x] Update focused route tests to provide the same use-case app data when
      constructing `api::courses::course_scope()` directly.
- [x] Self-critique: content HTTP still owns notification dispatch for
      `content:published`, and video-processing HTTP still owns auth extraction.
      Later slices should move those behind notification and actor/extractor
      ports, but the concrete Postgres/S3 boundary is now out of content HTTP.
- [x] Prove the content handler folder no longer imports concrete Postgres,
      object-storage providers, or `DbPool`, and prove the binary still checks.

Slice 20: move operations routes and readiness wiring behind HTTP/bootstrap.

- [x] Add an application-facing `ReadinessUseCase` trait under
      `application/operations/readiness_check`.
- [x] Add `bootstrap/readiness.rs` as the runtime implementation that wires
      Postgres, S3, and Ethereum readiness dependencies.
- [x] Move `/health` and `/ready` handlers and route configuration into
      `http/operations`.
- [x] Make `src/api/health.rs` a thin compatibility wrapper around
      `http::operations`.
- [x] Wire the readiness use-case trait object through `bootstrap::AppState`
      and production Actix app data.
- [x] Preserve `/ready` fallback behavior when readiness app data is missing:
      it returns `503` with `not_ready` instead of an extractor failure.
- [x] Update routing tests to compose `http::operations` directly.
- [x] Self-critique: operations readiness is now properly routed and wired, but
      several other legacy `api` modules still compose services/repositories
      directly and should be migrated one context at a time.
- [x] Prove liveness, missing-readiness fallback, full readiness, route
      composition, and operations unit behavior with focused tests.

Slice 21: move role catalog routes into the access-control HTTP ring.

- [x] Add an application-facing `RoleCatalogUseCase` trait under
      `application/access_control/list_roles`.
- [x] Add `infra/postgres/access_control/role_catalog_use_case.rs` as the
      DbPool-backed implementation.
- [x] Move `/roles`, `/roles/organization`, and `/roles/course` handlers and
      scope composition into `http/access_control`.
- [x] Make `src/api/roles.rs` a thin compatibility wrapper around
      `http::access_control`.
- [x] Wire the role catalog use-case trait object through `bootstrap::AppState`
      and production Actix app data.
- [x] Update the role-read permission route test to compose
      `http::access_control` directly and provide the injected use case.
- [x] Preserve DB connection and catalog load failure response bodies.
- [x] Self-critique: this cleans up role catalog reads only. Role assignment
      still lives in legacy user/organization/course API modules and should be
      migrated with the broader access-control authorization cleanup.
- [x] Prove role route permissions, catalog reads, application helper behavior,
      and route composition with focused tests.

Slice 22: move notification preference routes into the notifications HTTP ring.

- [x] Add an application-facing `NotificationPreferencesUseCase` trait under
      `application/notifications`.
- [x] Add `infra/postgres/notifications/notification_preferences_use_case.rs`
      as the DbPool-backed implementation.
- [x] Move `/me/preferences` handlers and route configuration into
      `http/notifications`.
- [x] Keep `api::session` as a compatibility re-export for preference handlers.
- [x] Wire the notification preference use-case trait object through
      `bootstrap::AppState` and production Actix app data.
- [x] Update the current-session API test app to compose `http::notifications`
      directly and provide the injected use case.
- [x] Preserve existing preference DB-unavailable and load/save failure
      response bodies.
- [x] Self-critique: `/me/notifications` list/read/clear still use the legacy
      session notification handlers and `NotificationsState::from(pool)` after
      this slice. Resolved by Slice 23 with a dedicated notification inbox use
      case and HTTP route owner.
- [x] Prove default preferences and save/reload behavior, application default
      behavior, route composition, and binary wiring with focused tests.

Slice 23: move notification inbox routes into the notifications HTTP ring.

- [x] Add an application-facing `notification_inbox` use case under
      `application/notifications` for list, mark-read, and clear behavior.
- [x] Add a `NotificationInboxStore` port so notification read-management no
      longer depends on `NotificationsState` from HTTP.
- [x] Add `infra/postgres/notifications/notification_inbox_store.rs` and
      `notification_inbox_use_case.rs` as the DbPool-backed implementation.
- [x] Move `/me/notifications` GET/DELETE and
      `/me/notifications/{id}/read` PUT handlers into `http/notifications`.
- [x] Add notification response DTO mapping in `http/notifications/dto` so the
      public JSON shape is separate from the Diesel record.
- [x] Remove `src/api/session/notifications.rs`; keep `api::session`
      compatibility re-exports for existing handler imports.
- [x] Wire the notification inbox use-case trait object through
      `bootstrap::AppState` and production Actix app data.
- [x] Preserve existing response bodies:
      `Notification marked as read`, `Notifications cleared`,
      `Failed to load notifications`, `Failed to mark notification as read`,
      and `Failed to clear notifications`.
- [x] Preserve notification list JSON fields:
      `id`, `user_id`, `title`, `body`, `created_at`, and `read`.
- [x] Self-critique: notification creation and delivery still live in
      `utils::notifications::NotificationsState` and remain injected for
      producer paths. Move send/dispatch behind `application/notifications`
      ports and `infra/notifications` in a later slice.
- [x] Prove ownership filtering, mark-read, clear, route composition,
      application notification behavior, and binary wiring with focused tests.

Slice 24: move current-session routes into the identity HTTP ring.

- [x] Add an application-facing `CurrentSessionUseCase` trait under
      `application/identity/current_session`.
- [x] Add a `CurrentSessionStore` port plus application output/error types so
      the current-session contract is not a Diesel record or HTTP DTO.
- [x] Add `infra/postgres/identity/current_session_store.rs` and
      `current_session_use_case.rs` as the DbPool-backed implementation.
- [x] Move current-session scope assembly behind the Postgres identity adapter,
      split into session load, delegation application, and scope-builder
      modules.
- [x] Move `/me` handler and route configuration into `http/identity`.
- [x] Add current-session response DTO mapping in `http/identity/dto` while
      preserving the existing JSON field names.
- [x] Make `src/api/session.rs` a thin compatibility re-export for identity and
      notification handlers.
- [x] Wire the current-session use-case trait object through
      `bootstrap::AppState`, production Actix app data, and focused test app
      data.
- [x] Preserve existing current-session error codes/messages for
      unauthorized, missing user, unverified email, DB-unavailable, and
      session-load failures.
- [x] Self-critique: the old include-based `services/session_service` module
      still exists for legacy unit coverage after this slice. Resolved by
      Slice 25 by moving the builder tests to the new identity infra module and
      deleting the legacy service module.
- [x] Prove profile/scope/delegation output, authorization rejection, missing
      user, unverified email, application fake-port behavior, route
      composition, and binary wiring with focused tests.

Slice 25: retire the legacy current-session service module.

- [x] Move the remaining current-session scope-builder tests from
      `services/session_service/tests.rs` to
      `infra/postgres/identity/current_session_scope_builder/tests.rs`.
- [x] Keep the moved tests beside the Postgres identity mapper/builder code
      that now owns this behavior.
- [x] Remove `pub mod session_service` from `src/services/mod.rs`.
- [x] Delete the include-based `src/services/session_service.rs` wrapper and
      the `src/services/session_service/` folder.
- [x] Preserve current-session API behavior by relying on the already migrated
      `http/identity` + `application/identity/current_session` +
      `infra/postgres/identity` path.
- [x] Prove the old module is gone, the moved builder tests pass, the
      current-session API tests still pass, route composition still passes, and
      the app binary still checks.

Slice 26: move reward policy routes into the rewards HTTP ring.

- [x] Use `/reward-policies` as the first rewards HTTP-ring migration because
      policy management is small enough to prove the rewards boundary without
      moving payout execution, candidate review, fraud blocking, and wallet
      crediting in the same slice.
- [x] Move reward policy scope/event/payment normalization into
      `domain/rewards/policy`.
- [x] Create `application/rewards/manage_reward_policy` with explicit
      command/query/output/error/service modules and a `RewardPolicyStore`
      port under `application/rewards/ports.rs`.
- [x] Move reward policy validation and versioning orchestration out of the
      legacy include-based service module.
- [x] Move reward policy Diesel access behind
      `infra/postgres/rewards/reward_policy_store.rs`, with DbPool wiring in
      `reward_policy_use_case.rs` and Diesel/application mapping in
      `reward_policy_mappers.rs`.
- [x] Move `/api/reward-policies` request/response DTOs, handlers, and route
      composition into `http/rewards`.
- [x] Make `src/api/reward_policies.rs` a thin compatibility wrapper around
      `http::rewards::reward_policy_scope`.
- [x] Wire the reward policy use-case trait object through
      `bootstrap::AppState`, production Actix app data, and focused tests.
- [x] Delete the legacy `services/reward_policy_service` include module and
      move its pure normalization/amount validation coverage beside the new
      domain/application owners.
- [x] Preserve existing route path, JSON field names, policy versioning
      behavior, active-policy deactivation, permission denial body,
      DB-unavailable body, and generic processing-error body.
- [x] Self-critique: this moves reward policy management only. Reward
      candidates, fraud blocks, payout execution, compensation, history, and
      reporting still use legacy API/service wiring and should move through
      separate rewards slices. Reward policy permission checks still call the
      existing platform permission repository from the Postgres adapter until a
      shared `application/access_control` authorization port becomes the single
      backend source of truth.
- [x] Prove the reward policy service module is gone, reward policy HTTP has no
      Diesel/repository/service imports, pure validation tests pass, focused
      reward policy integration behavior passes, route composition still
      passes, and the app binary still checks.

Slice 27: move reward fraud-block routes into the rewards HTTP ring.

- [x] Use `/reward-fraud-blocks` as the next rewards migration because fraud
      blocks are a bounded management surface with create, list, revoke, audit,
      permission checks, and notification fan-out.
- [x] Move fraud-block scope vocabulary and target matching into
      `domain/rewards/fraud_block`.
- [x] Create `application/rewards/manage_fraud_block` with explicit
      command/query/output/error/service modules and a `RewardFraudBlockStore`
      port under `application/rewards/ports.rs`.
- [x] Move fraud-block request normalization, exact-target validation, list
      pagination shaping, revoke idempotency, and audit-event assembly out of
      the legacy include-based service module.
- [x] Move fraud-block Diesel persistence behind
      `infra/postgres/rewards/reward_fraud_block_store.rs`, with DbPool wiring
      in `reward_fraud_block_use_case.rs` and Diesel/application mapping in
      `reward_fraud_block_mappers.rs`.
- [x] Move fraud-block permission checks and notification recipient queries
      behind dedicated Postgres rewards modules so application and HTTP do not
      import permission constants, Diesel schema, or notification writes.
- [x] Move `/api/reward-fraud-blocks` request/response DTOs, handlers, and
      route composition into `http/rewards`.
- [x] Make `src/api/reward_fraud_blocks.rs` a thin compatibility wrapper
      around `http::rewards::reward_fraud_block_scope`.
- [x] Wire the reward fraud-block use-case trait object through
      `bootstrap::AppState`, production Actix app data, route tests, and
      focused API tests.
- [x] Delete the legacy `services/reward_fraud_block_service` include module
      and move its pure scope/target/permission coverage beside the new
      domain/application/infra owners.
- [x] Preserve existing route paths, JSON field names, create/revoke
      notification titles and bodies, permission denial body, DB-unavailable
      body, not-found body, and generic processing-error body.
- [x] Self-critique: this moves reward fraud-block management only. Reward
      candidates still call legacy reward-candidate services, and payout
      execution, compensation, history, and reporting still need their own
      rewards slices. Fraud-block permission checks still call the existing
      platform permission repository from the Postgres adapter until
      `application/access_control` becomes the shared authorization source.
- [x] Prove the fraud-block service module is gone, reward fraud-block HTTP has
      no Diesel/repository/service imports, pure domain/application/infra tests
      pass, focused fraud-block integration behavior passes, route composition
      still passes, reward-candidate fraud-block regressions still pass, and
      the app binary still checks.

Slice 28: move student reward history into the rewards HTTP ring.

- [x] Use `GET /reward-candidates/me/history` as the next rewards migration
      because it is a read-only student-facing reward history slice with
      existing route-level coverage and no candidate mutation behavior.
- [x] Move reward candidate status normalization into
      `domain/rewards/candidate/status`.
- [x] Create `application/rewards/list_reward_history` with explicit
      query/output/error/service modules and a `StudentRewardHistoryStore` port
      under `application/rewards/ports.rs`.
- [x] Move history status validation, pagination clamping, course-status
      permission filtering, and financial-reference assembly out of the legacy
      include-based `services/reward_history_service`.
- [x] Move student reward history Diesel reads behind
      `infra/postgres/rewards/reward_history_store.rs`, with DbPool wiring in
      `reward_history_use_case.rs` and wallet/token financial lookups split
      into `reward_history_financials.rs`.
- [x] Move `/api/reward-candidates/me/history` request/response DTOs, handler,
      and route composition into `http/rewards` while preserving the legacy URL.
- [x] Remove the student-history route from the legacy reward-candidate API
      configurator so only `http/rewards` owns that route.
- [x] Wire the student reward history use-case trait object through
      `bootstrap::AppState`, production Actix app data, route tests, and the
      focused student reward history API test.
- [x] Delete the legacy `services/reward_history_service` include module.
- [x] Preserve existing JSON field names, status filter normalization, invalid
      status bad-request body, DB-unavailable body, and generic history-load
      body.
- [x] Self-critique: this moves student reward history only. Candidate
      submission, teacher review, platform amount review, candidate audit,
      payout execution, compensation, and reporting remain legacy reward
      surfaces. Course reward-status permission still goes through the existing
      course repository from the Postgres adapter until access-control is
      centralized.
- [x] Prove the reward history service module is gone, student history route
      ownership moved to `http/rewards`, reward HTTP has no
      Diesel/repository/service imports, pure status/history validation tests
      pass, the focused student history route test passes, route composition
      still passes, and the app binary still checks.

Slice 29: move reward candidate audit reads into the rewards HTTP ring.

- [x] Use `GET /reward-candidates/{candidate_id}/audit` as the next rewards
      migration because it is a read-only platform audit slice with one
      permission gate and no candidate state mutation.
- [x] Create `application/rewards/list_candidate_audit` with explicit
      output/error/service modules and a `RewardCandidateAuditStore` port under
      `application/rewards/ports.rs`.
- [x] Move the candidate existence check, platform audit permission gate, and
      audit-event list operation out of the legacy include-based
      `services/reward_candidate_service`.
- [x] Move reward candidate audit Diesel reads behind
      `infra/postgres/rewards/reward_candidate_audit_store.rs`, with DbPool
      wiring in `reward_candidate_audit_use_case.rs` and model-to-application
      mapping in `reward_candidate_audit_mappers.rs`.
- [x] Move `/api/reward-candidates/{candidate_id}/audit` response DTO,
      handler, and route composition into `http/rewards` while preserving the
      legacy URL and response field names.
- [x] Remove the candidate-audit route from the legacy reward-candidate API
      configurator so only `http/rewards` owns that route.
- [x] Wire the reward candidate audit use-case trait object through
      `bootstrap::AppState`, production Actix app data, route tests, and a
      focused reward candidate audit API test.
- [x] Preserve the legacy forbidden body, missing-candidate body,
      DB-unavailable body, generic processing-error body, and audit-event JSON
      field names.
- [x] Self-critique: this moves candidate audit reads only. Candidate
      submission, course candidate listing, platform review listing, teacher
      decisions, amount decisions, payout execution, compensation, and
      reporting remain legacy reward surfaces. Platform audit permission still
      goes through the existing platform permission repository from the
      Postgres adapter until access-control is centralized.
- [x] Prove the legacy candidate-audit handler is gone, the audit route
      ownership moved to `http/rewards`, reward HTTP has no
      Diesel/repository/service imports, application fake-port tests pass, the
      focused audit route test passes, route composition still passes, and the
      app binary still checks.

Slice 30: move course reward-candidate listing into the rewards HTTP ring.

- [x] Use `GET /courses/{course_id}/reward-candidates` as the next rewards
      migration because it is a read-only course-scoped candidate list with
      existing route coverage and a clear permission boundary.
- [x] Create `application/rewards/list_course_candidates` with explicit
      query/output/error/service modules and a `CourseRewardCandidateStore`
      port under `application/rewards/ports.rs`.
- [x] Move course existence checks, course reward-candidate management
      permission probes, student self-filter narrowing, status normalization,
      and pagination inputs out of the legacy include-based
      `services/reward_candidate_service`.
- [x] Move course reward-candidate Diesel reads behind
      `infra/postgres/rewards/course_reward_candidate_store.rs`, with DbPool
      wiring in `course_reward_candidate_use_case.rs` and model-to-application
      mapping in `course_reward_candidate_mappers.rs`.
- [x] Move `/api/courses/{course_id}/reward-candidates` list request/response
      DTOs, handler, and GET route composition into `http/rewards` while
      preserving the legacy URL and response field names.
- [x] Remove the course-candidate list route from the legacy reward-candidate
      API configurators while keeping legacy POST and teacher-decision routes
      stable.
- [x] Wire the course reward-candidate list use-case trait object through
      `bootstrap::AppState`, production Actix app data, route tests, and a
      focused course reward-candidate list API test.
- [x] Preserve the legacy forbidden body, invalid-status conflict body,
      missing-course/candidate body, DB-unavailable body, generic
      processing-error body, manager visibility, and student self-filtering.
- [x] Self-critique: this moves course candidate listing only. Course candidate
      submission on the same path, teacher decisions, platform review listing,
      amount decisions, payout execution, compensation, and reporting remain
      legacy reward surfaces. Course permission checks still go through the
      existing course repository from the Postgres adapter until
      access-control is centralized.
- [x] Prove the legacy course-candidate list service is gone, GET ownership
      moved to `http/rewards`, reward HTTP has no Diesel/repository/service
      imports, application fake-port tests pass, the focused list route test
      passes, route composition still passes, broader reward-candidate
      regressions still pass, and the app binary still checks.

Slice 31: move platform reward-candidate review listing into the rewards HTTP ring.

- [x] Use `GET /reward-candidates/review` as the next rewards migration because
      it is a read-only platform review queue with existing enriched-response
      coverage and no candidate mutation behavior.
- [x] Create `application/rewards/list_platform_candidates` with explicit
      query/output/error/service modules plus a module-local
      `PlatformRewardCandidateStore` because the platform review enrichment
      contract is unique and `application/rewards/ports.rs` is already near the
      manual line limit.
- [x] Move platform audit permission checks, amount-approval capability reads,
      status normalization, search normalization, in-memory search,
      pagination, user/course enrichment, and fallback summaries out of the
      legacy include-based `services/reward_candidate_service`.
- [x] Move platform reward-candidate list/count reads and user/course summary
      Diesel lookups behind
      `infra/postgres/rewards/platform_reward_candidate_store.rs`, with DbPool
      wiring in `platform_reward_candidate_use_case.rs`.
- [x] Move `/api/reward-candidates/review` request/response DTOs, handler, and
      GET route composition into `http/rewards` while preserving the legacy URL
      and response field names.
- [x] Remove the platform review route, service function, enrichment helper,
      and service-owned request/response DTOs from the legacy reward-candidate
      API/service.
- [x] Wire the platform review use-case trait object through
      `bootstrap::AppState`, production Actix app data, route tests, and the
      existing platform review regression test.
- [x] Preserve the legacy forbidden body, invalid-status conflict body,
      DB-unavailable body, generic processing-error body, enriched user/course
      response shape, search behavior, pagination behavior, and operator
      permission flags.
- [x] Self-critique: this moves platform candidate review listing only. Course
      candidate submission, organization candidate submission, teacher
      decisions, amount decisions, payout execution, compensation, and
      reporting remain legacy reward surfaces. Platform permission checks still
      go through the existing platform permission repository from the Postgres
      adapter until access-control is centralized.
- [x] Prove the legacy platform review route/service/enrichment helpers are
      gone, route ownership moved to `http/rewards`, reward HTTP has no
      Diesel/repository/service imports, application fake-port tests pass, the
      platform review regression passes through HTTP, route composition still
      passes, legacy reward-candidate service tests still pass, and the app
      binary still checks.

Slice 32: move teacher reward-candidate decisions into the rewards HTTP ring.

- [x] Use `PUT /courses/{course_id}/reward-candidates/{candidate_id}/teacher-decision`
      as the next rewards mutation migration because it has a clear boundary:
      course approval permission, teacher decision status normalization,
      candidate transition, fraud-block guard, candidate update, and audit
      event creation.
- [x] Create `application/rewards/decide_teacher_candidate` with explicit
      command/output/error/service modules and a module-local
      `TeacherRewardCandidateDecisionStore` because the transactional mutation
      contract is unique to this use case.
- [x] Preserve legacy teacher decision status inputs exactly: `approved` and
      `teacher_approved` map to `teacher_approved`; `rejected` and
      `teacher_rejected` map to `teacher_rejected`; hyphenated values remain
      invalid.
- [x] Reuse the pure domain candidate transition rule for teacher
      approve/reject validation while keeping DB string conversion at the
      Postgres boundary.
- [x] Move teacher-decision permission probing, candidate load/update,
      active fraud-block checks, active reward-policy lookup, and audit-event
      insertion behind `infra/postgres/rewards`.
- [x] Keep the candidate read, transition check, fraud-block check, update, and
      audit insert inside one Postgres transaction. Use an infra-only
      transaction error wrapper so the application error type remains
      Diesel-free.
- [x] Move the teacher-decision request/response DTO and handler into
      `http/rewards` while preserving the legacy URL, response field names, and
      legacy HTTP error bodies.
- [x] Remove teacher-decision route ownership from
      `api/reward_candidates`. Register the `http/rewards` resource inside the
      temporary legacy course scope so the existing `/api/courses/...` prefix
      does not shadow the new route during migration.
- [x] Wire the teacher-decision use-case trait object through
      `bootstrap::AppState`, production Actix app data, and route smoke-test
      app data.
- [x] Self-critique: the legacy direct service function remains temporarily
      because existing integration tests still call reward-candidate mutations
      as service helpers. Later mutation slices should move submit-candidate
      and any remaining teacher-decision helpers to application use cases, then
      delete the include-based reward-candidate service.
- [x] Prove application fake-port tests pass, route composition reaches the
      teacher-decision URL, application/domain import scans stay clean, touched
      non-generated Rust files stay under the manual line limit, and the app
      binary still checks.

Slice 33: move reward amount decisions into the rewards HTTP ring.

- [x] Use `PUT /reward-candidates/{candidate_id}/amount-decision` as the next
      rewards mutation migration because it completes the teacher-to-platform
      approval path and owns platform permission, amount validation, candidate
      transition, fraud-block guard, audit creation, and execution-job enqueue.
- [x] Create `application/rewards/decide_amount` with explicit
      command/output/error/service modules and a module-local
      `RewardAmountDecisionStore` because the transaction contract is unique to
      this mutation.
- [x] Preserve legacy amount decision status inputs exactly: `approved` and
      `amount_approved` map to `amount_approved`; `rejected` and
      `amount_rejected` map to `amount_rejected`; unsupported values remain
      conflicts.
- [x] Preserve legacy validation order: status is normalized first, platform
      amount-approval permission is checked second, and approved-amount
      required/non-negative validation runs only after permission succeeds.
- [x] Reuse the pure domain candidate transition rule for amount approve/reject
      validation while keeping DB string conversion at the Postgres boundary.
- [x] Move amount-decision platform permission probing, candidate load/update,
      active fraud-block checks, active reward-policy lookup, audit-event
      insertion, and execution-job enqueueing behind `infra/postgres/rewards`.
- [x] Extract shared reward-candidate fraud-block and active-policy lookup
      helpers under `infra/postgres/rewards` so teacher and amount decisions do
      not duplicate the same adapter queries.
- [x] Keep the candidate read, transition check, fraud-block check, update,
      audit insert, and execution-job enqueue inside one Postgres transaction.
      Use an infra-only transaction error wrapper so the application error type
      remains Diesel-free.
- [x] Move the amount-decision request/response DTO and handler into
      `http/rewards` while preserving the legacy URL, response field names, and
      legacy HTTP error bodies.
- [x] Remove amount-decision route ownership from `api/reward_candidates`.
- [x] Wire the amount-decision use-case trait object through
      `bootstrap::AppState`, production Actix app data, and route smoke-test
      app data.
- [x] Replace the service-owned `RewardAmountDecisionRequest` with a
      compatibility alias to the application command. Reward-candidate and
      delegated-permission amount-decision test helpers now call the
      application-facing Postgres use case instead of the legacy service
      function.
- [x] Self-critique: at the end of Slice 33, candidate submission routes still
      lived in the legacy reward-candidate API/service. Slice 34 resolves that
      route ownership; reward execution/payout/compensation flows still need
      their own Level 2 slices before the include-based reward-candidate service
      can be deleted.
- [x] Prove application fake-port tests pass, route composition reaches the
      amount-decision URL, the existing teacher-to-amount approval regression
      still enqueues exactly one execution job, application/domain and HTTP
      import scans stay clean, touched non-generated Rust files stay under the
      manual line limit, and the app binary still checks.

Slice 34: move reward candidate submission into the rewards HTTP ring.

- [x] Use `POST /courses/{course_id}/reward-candidates` and
      `POST /organizations/{organization_id}/courses/{course_id}/reward-candidates`
      as the next rewards mutation migration because submission still owned
      candidate creation, idempotency replay, evidence validation, policy
      eligibility, fraud-block gating, source-scope selection, and audit
      creation from the legacy API/service layer.
- [x] Create `application/rewards/submit_candidate` with explicit
      command/output/error/service/store modules. The application boundary owns
      course-vs-organization source selection, course existence, attachment
      validation, and permission ordering, while persistence details stay out of
      the use-case tests.
- [x] Move reward candidate event-type normalization into
      `domain/rewards/candidate/event_type` and evidence threshold validation
      into `domain/rewards/candidate/evidence`.
- [x] Preserve legacy submission behavior: course submit accepts
      `SUBMIT_COURSE_REWARD_EVENT` or `CREATE_REWARDABLE_COURSE_EVENT`,
      organization submit requires `SUBMIT_ORG_COURSE_REWARD_EVENT`, blank
      idempotency keys are rejected, same-key/same-target replays return the
      existing candidate, mismatched key reuse is a bad request, and candidate
      creation still writes the submitted audit event atomically.
- [x] Move candidate submission Postgres behavior behind
      `infra/postgres/rewards`, split into validation, eligibility, mutation,
      audit insert, mapper, store, and use-case adapter modules so the adapter
      stays granular and under the manual line limit.
- [x] Move submission request/response DTOs and handlers into `http/rewards`;
      the legacy URLs remain stable, including bridge resources composed inside
      the existing course and organization scopes until those broader scopes
      move fully to the HTTP ring.
- [x] Remove legacy `api/reward_candidates` route ownership entirely. Reward
      candidate history, audit, list, review, teacher decision, amount decision,
      and submission routes are now composed by `http/rewards`.
- [x] Wire `RewardCandidateSubmissionUseCase` through `bootstrap::AppState`,
      production Actix app data, and route smoke-test app data.
- [x] Move reward-candidate and delegated-permission submission regression
      helpers onto `PostgresRewardCandidateSubmissionUseCase` so the existing
      DB-backed behavior tests exercise the new Level 2 path.
- [x] Self-critique: this completes reward candidate submission route
      ownership, but reward execution, payout, compensation, reconciliation,
      and broader permission-source unification still need Level 2 slices before
      the old reward candidate service and repository helpers can be retired.
- [x] Prove application fake-port tests pass, domain event/evidence tests pass,
      route composition reaches both submission URLs, course/org/delegated
      submission regressions pass through the new use case, application/domain
      and HTTP import scans stay clean, touched non-generated Rust files stay
      under the manual line limit, and the app binary still checks.

Slice 35: move reward compensation recording into the rewards application and
Postgres rings.

- [x] Use reward compensation as the next rewards migration because it is a
      compact but real mutation: platform permission, command validation,
      idempotency replay, reward candidate lookup, wallet linking, guarded
      wallet adjustment, transaction linking, and compensation record creation
      previously lived in one include-based service.
- [x] Create `application/rewards/record_compensation` with explicit
      command/output/error/service/store/validation modules. Preserve the
      legacy order: permission is checked before request validation, and the
      store is not called when either step fails.
- [x] Move the compensation transaction type vocabulary into
      `domain/rewards/compensation`.
- [x] Move Postgres compensation behavior behind `infra/postgres/rewards`,
      split into mapper, wallet, transaction, transaction-body, store, and
      use-case adapter modules. The new infra module reimplements the small
      user-wallet link query locally instead of calling the legacy wallet
      service from a new ring.
- [x] Preserve legacy compensation semantics: `RECONCILE_WALLETS` or
      `MANAGE_WALLETS` can record compensation, zero amount is invalid,
      negative compensation is allowed as long as the guarded wallet update does
      not go below zero, reason/idempotency key must be nonblank, same
      idempotency key returns the existing record without reapplying the wallet
      adjustment, and compensation does not mutate reward candidate decision
      fields.
- [x] Remove `src/services/reward_compensation_service.rs` and
      `src/services/reward_compensation_service/`.
- [x] Move the reward compensation regression helper onto
      `PostgresRewardCompensationUseCase` so the DB-backed compensation test
      exercises the new Level 2 path.
- [x] Self-critique: this removes one include-based reward service, but reward
      execution, token confirmation, wallet credit notification, reconciliation,
      wallet audit, and reporting read models still need their own Level 2
      slices.
- [x] Prove application fake-port and validation tests pass, the existing
      compensation regression passes through the new Postgres use case,
      application/domain import scans stay clean, new infra does not call the
      legacy compensation or wallet services, touched non-generated Rust files
      stay under the manual line limit, and the app binary still checks.

Slice 36: move reward payout planning into the rewards application and
Postgres rings.

- [x] Use payout planning as the next rewards migration because it is the
      decision point between approved reward candidates, active reward
      policies, configured token infrastructure, and execution strategy.
- [x] Create `application/rewards/plan_payout` with explicit
      output/error/service/store/validation modules. The application use case
      owns candidate readiness checks, approved-amount validation, active
      policy requirement, payout-method selection, and actor permission gating.
- [x] Move payout-method vocabulary into `domain/rewards/payout` while keeping
      payment-strategy normalization in `domain/rewards/policy`.
- [x] Move Postgres payout-planning behavior behind `infra/postgres/rewards`,
      split into mapper, policy lookup, store, and use-case adapter modules.
      Policy lookup preserves the legacy priority: course policy, linked
      organization policy, then platform policy.
- [x] Preserve legacy payout-planning semantics: only amount-approved
      candidates can be planned, approved amount must be present and positive,
      `EXECUTE_REWARD_PAYOUT` gates actor-triggered planning, treasury
      strategies use the presigner contract when configured, mint and
      treasury strategies require token confirmation, and off-chain payouts do
      not.
- [x] Remove the legacy `select_payout_method.rs` include. The legacy
      `plan_reward_payout` service entry points now delegate through the
      application handler and Postgres store while the surrounding wallet
      credit flow is migrated separately.
- [x] Self-critique: payout planning now has a Level 2 boundary, but reward
      token confirmation, wallet credit, wallet credit notification,
      reconciliation, wallet audit, and reporting read models still need their
      own Level 2 slices before reward execution can be retired cleanly.
- [x] Prove application fake-port and validation tests pass, the existing
      reward-execution payout-planning regressions pass through
      `PostgresRewardPayoutPlanUseCase`, application/domain import scans stay
      clean, new infra does not call legacy reward execution or wallet
      services, touched non-generated Rust files stay under the manual line
      limit, and the app binary still checks.

Slice 37: move reward token confirmation into the rewards application and
Postgres rings.

- [x] Use token confirmation as the next reward-execution migration because it
      is the token-side mutation between payout planning and wallet credit:
      command validation, actor permission gating, external transaction
      idempotency, payout-record creation, candidate status transition, and
      audit insertion previously lived in the include-based reward execution
      service.
- [x] Create `application/rewards/record_token_confirmation` with explicit
      command/output/error/service/store/validation modules. Preserve legacy
      ordering: direct calls validate before persistence, while actor-triggered
      calls check `EXECUTE_REWARD_PAYOUT` before command validation.
- [x] Move token event to transaction-type vocabulary into
      `domain/rewards/token`.
- [x] Move Postgres token-confirmation behavior behind
      `infra/postgres/rewards`, split into mapper, external-transaction,
      transaction, store, and use-case adapter modules. The transaction uses
      the domain candidate transition rule for `TokenPending -> TokenConfirmed`
      while preserving the legacy invalid-status message.
- [x] Preserve legacy token-confirmation semantics: existing payout records are
      returned idempotently without a second audit event, candidates must be
      token pending before confirmation, external transactions are reused by
      `(chain_id, transaction_hash, log_index)`, missing transaction links are
      repaired, the candidate is marked token confirmed, and audit metadata
      includes transaction, external transaction, payout record, and insert
      status.
- [x] Remove stale token-confirmation logic from the legacy reward execution
      service. Its public token-confirmation entry points now delegate through
      the application handler and Postgres store; the old external transaction
      include only keeps the reconciliation link-repair helper needed by the
      remaining legacy reconciliation slice.
- [x] Remove orphaned `src/services/reward_execution_service/tests*` files
      whose included tests were no longer compiled and still referenced moved
      legacy helpers.
- [x] Self-critique: token confirmation now has a Level 2 boundary, but wallet
      credit, wallet credit notification, reconciliation, wallet audit, and
      reporting read models still need their own Level 2 slices before reward
      execution can be retired cleanly.
- [x] Prove domain token mapping tests, application fake-port and validation
      tests, and the existing DB-backed reward-execution regressions pass with
      token-confirmation helpers routed through
      `PostgresRewardTokenConfirmationUseCase`; application/domain import
      scans stay clean, new infra does not call legacy reward execution or
      wallet services, touched non-generated Rust files stay under the manual
      line limit, and the app binary still checks.

Slice 38: move reward wallet credit into the rewards application and Postgres
rings.

- [x] Use wallet credit as the next reward-execution migration because it is
      the off-chain/token-confirmed side effect after payout planning:
      permission gating, approved-amount validation, active policy checks,
      idempotent reward-wallet-credit records, wallet linking, guarded wallet
      balance updates, internal/generic transaction creation, candidate status
      transition, and audit insertion previously lived in the include-based
      reward execution service.
- [x] Create `application/rewards/credit_wallet` with explicit
      output/error/service/store/handler modules. Preserve legacy ordering:
      direct calls credit without actor context, while actor-triggered calls
      check `EXECUTE_REWARD_PAYOUT` before persistence.
- [x] Move reward wallet-credit transaction vocabulary into
      `domain/rewards/wallet_credit`.
- [x] Move Postgres wallet-credit behavior behind `infra/postgres/rewards`,
      split into mapper, policy, wallet, transaction, validation, store, and
      use-case adapter modules. The rewards adapter links/creates wallets
      locally instead of calling the legacy wallet service from a new ring.
- [x] Preserve legacy wallet-credit semantics: existing credit records replay
      idempotently, wallet-credited/notified/completed candidates without a
      record are rejected, token-confirmed candidates can be credited, amount
      approved candidates can be credited only for active off-chain reward
      policies, reconciliation credit remains explicitly gated, wallet balance
      is adjusted before the internal/generic transaction and credit record,
      and audit metadata records the wallet, credit record, transaction, and
      internal transaction IDs.
- [x] Move legacy public wallet-credit entry points to delegate through the
      application handler and `PostgresRewardWalletCreditStore`. The remaining
      reconciliation bridge temporarily calls the new infra transaction helper
      until reconciliation gets its own Level 2 slice.
- [x] Self-critique: wallet credit now has a Level 2 boundary, but wallet
      credit notification, reconciliation, wallet audit, reward execution route
      wiring, and reporting read models still need their own slices before the
      legacy reward execution service can be retired cleanly.
- [x] Prove application fake-port tests and the existing DB-backed
      reward-execution regressions pass with wallet-credit helpers routed
      through `PostgresRewardWalletCreditUseCase`; application/domain import
      scans stay clean, new infra does not call legacy reward execution or
      wallet services, touched non-generated Rust files stay under the manual
      line limit, and the app binary still checks.

Slice 39: move reward wallet-credit notification into the rewards application
and Postgres rings.

- [x] Use wallet-credit notification as the next reward-execution migration
      because it is the user-facing side effect after wallet credit:
      permission gating, notification idempotency, reward-wallet-credit record
      updates, candidate notified status transition, and audit insertion
      previously lived in the include-based reward execution service.
- [x] Create `application/rewards/notify_wallet_credit` with explicit
      output/error/service/store/handler modules. Preserve legacy ordering:
      direct calls notify without actor context, while actor-triggered calls
      check `EXECUTE_REWARD_PAYOUT` before persistence.
- [x] Move reward wallet-credit notification title/body vocabulary into
      `domain/rewards/wallet_credit` so the new reward adapter does not depend
      on legacy `utils::notifications`.
- [x] Move Postgres wallet-credit notification behavior behind
      `infra/postgres/rewards`, split into mapper, notification insertion,
      validation, transaction, store, and use-case adapter modules.
- [x] Preserve legacy wallet-credit notification semantics: only
      wallet-credited candidates can create a normal notification, existing
      notification IDs replay idempotently without a second notification or
      audit event, notified/completed/reconciliation states with missing
      notification references are rejected, reconciliation repairs can create
      the missing notification only from explicitly allowed confirmed states,
      and audit metadata records the wallet, notification, and transaction IDs.
- [x] Move legacy public wallet-credit notification entry points to delegate
      through the application handler and
      `PostgresRewardWalletCreditNotificationStore`. The remaining
      reconciliation bridge temporarily calls the new infra transaction helper
      until reconciliation gets its own Level 2 slice.
- [x] Self-critique: wallet-credit notification now has a Level 2 boundary,
      but reconciliation, wallet audit, reward execution route wiring, and
      reporting read models still need their own slices before the legacy
      reward execution service can be retired cleanly.
- [x] Prove the domain notification message test, application fake-port tests,
      and the existing DB-backed reward-execution regressions pass with the
      wallet-credit notification regression routed through
      `PostgresRewardWalletCreditNotificationUseCase`; application/domain
      import scans stay clean, new infra does not call legacy reward execution
      or notification utilities, touched non-generated Rust files stay under
      the manual line limit, and the app binary still checks.

Slice 40: move reward reconciliation into the rewards application and Postgres
rings.

- [x] Use reconciliation as the next reward-execution migration because it is
      the orchestration that repairs missing payout links, wallet-credit side
      effects, notification side effects, internal transaction links, and the
      reconciliation audit event.
- [x] Create `application/rewards/reconcile_candidate` with explicit
      output/error/service/store/handler modules. Preserve legacy ordering:
      direct calls reconcile without actor context, while actor-triggered calls
      check `EXECUTE_REWARD_PAYOUT` before persistence.
- [x] Move Postgres reconciliation behavior behind `infra/postgres/rewards`,
      split into mapper, validation, link repair, audit, transaction, store,
      and use-case adapter modules.
- [x] Preserve legacy reconciliation semantics: only confirmed/reconcilable
      reward candidate states can reconcile, missing external transaction links
      are repaired idempotently, missing wallet credit is created only for
      eligible confirmed states and only when payout evidence exists where
      required, missing internal transaction links are repaired idempotently,
      missing wallet-credit notifications are repaired through the
      wallet-credit notification transaction, and a reconciliation audit event
      is inserted only when at least one repair happened.
- [x] Move legacy public reconciliation entry points to delegate through the
      application handler and `PostgresRewardReconciliationStore`.
- [x] Remove stale private reward-execution include fragments:
      `reconcile_reward_candidate_with_actor.rs`,
      `record_external_reward_transaction.rs`,
      `ensure_wallet_credit_allowed.rs`,
      `credit_reward_wallet_for_candidate.rs`, and
      `notify_reward_wallet_credit_for_candidate.rs`.
- [x] Self-critique: reconciliation now has a Level 2 boundary and the legacy
      reward execution service is much thinner, but reward execution route
      wiring, wallet audit, reporting read models, and access-control
      unification still need their own slices before the rewards context is
      complete.
- [x] Prove application fake-port tests and the existing DB-backed
      reward-execution regressions pass with reconciliation routed through
      `PostgresRewardReconciliationUseCase`; application import scans stay
      clean, new infra does not call legacy reward execution or wallet
      services, touched non-generated Rust files stay under the manual line
      limit, and the app binary still checks.

Progress evidence from 2026-06-12 and 2026-06-13:

- `src/api/chapters.rs` is now a thin compatibility wrapper around
  `http::content::routes::configure_chapter_routes`.
- Chapter HTTP handlers now live under `http/content/handlers/chapter.rs` and
  delegate through the injected `ChapterUseCases` application-facing service.
- Chapter HTTP handlers now receive an injected `ChapterUseCases` application
  service; the concrete DbPool/Postgres wiring lives in
  `infra/postgres/content/chapter_use_cases.rs` and `bootstrap::AppState`.
- Content HTTP handlers now receive injected application-facing content use
  cases; concrete DbPool, Postgres store, and S3 provider wiring lives in
  `infra/postgres/content/*_use_case*.rs` and `bootstrap::AppState`.
- Chapter HTTP request/response DTOs now live under `http/content/dto`, while
  application output remains a non-Actix, non-Serialize type.
- `src/api/session` re-exports notification preference handlers from
  `http::notifications`; `/me/preferences` route composition now lives in
  `http/notifications`.
- `src/api/session` also re-exports notification inbox handlers from
  `http::notifications`; `/me/notifications` list/read/clear route
  composition now lives in `http/notifications`.
- `src/api/session` re-exports the current-session handler from
  `http::identity`; `/me` route composition now lives in `http/identity`.
- Current-session HTTP handlers and response DTOs now live under
  `http/identity`.
- Current-session reads are injected as an application-facing
  `CurrentSessionUseCase`; concrete DbPool/Postgres wiring lives in
  `infra/postgres/identity/current_session_use_case.rs` and
  `bootstrap::AppState`.
- Legacy `src/services/session_service.rs` and
  `src/services/session_service/` have been removed; the remaining
  current-session builder tests live beside
  `infra/postgres/identity/current_session_scope_builder.rs`.
- `src/api/reward_policies.rs` is now a thin compatibility wrapper around
  `http::rewards`.
- Reward policy HTTP handlers, routes, and request/response DTOs now live under
  `http/rewards`.
- Reward policy management is injected as an application-facing
  `RewardPolicyUseCase`; concrete DbPool/Postgres wiring lives in
  `infra/postgres/rewards/reward_policy_use_case.rs` and
  `bootstrap::AppState`.
- Reward policy scope/event/payment normalization now lives under
  `domain/rewards/policy`; amount and scope-reference validation lives under
  `application/rewards/manage_reward_policy`.
- Legacy `src/services/reward_policy_service.rs` and
  `src/services/reward_policy_service/` have been removed.
- `src/api/reward_fraud_blocks.rs` is now a thin compatibility wrapper around
  `http::rewards`.
- Reward fraud-block HTTP handlers, routes, and request/response DTOs now live
  under `http/rewards`.
- Reward fraud-block management is injected as an application-facing
  `RewardFraudBlockUseCase`; concrete DbPool/Postgres wiring lives in
  `infra/postgres/rewards/reward_fraud_block_use_case.rs` and
  `bootstrap::AppState`.
- Reward fraud-block scope vocabulary now lives under
  `domain/rewards/fraud_block`; request target validation lives under
  `application/rewards/manage_fraud_block`.
- Reward fraud-block notification fan-out and recipient lookup now live under
  `infra/postgres/rewards/reward_fraud_block_notifications.rs` and
  `reward_fraud_block_notification_recipients.rs`.
- Legacy `src/services/reward_fraud_block_service.rs` and
  `src/services/reward_fraud_block_service/` have been removed.
- Student reward history HTTP handlers and request/response DTOs now live under
  `http/rewards`, while preserving the legacy `/reward-candidates/me/history`
  URL.
- Student reward history is injected as an application-facing
  `StudentRewardHistoryUseCase`; concrete DbPool/Postgres wiring lives in
  `infra/postgres/rewards/reward_history_use_case.rs` and
  `bootstrap::AppState`.
- Student reward history candidate, wallet-credit, and token-transaction reads
  now live under `infra/postgres/rewards/reward_history_store.rs` and
  `reward_history_financials.rs`.
- Legacy `src/services/reward_history_service.rs` and
  `src/services/reward_history_service/` have been removed.
- Reward candidate audit HTTP handler and response DTO now live under
  `http/rewards`, while preserving the legacy
  `/reward-candidates/{candidate_id}/audit` URL.
- Reward candidate audit reads are injected as an application-facing
  `RewardCandidateAuditUseCase`; concrete DbPool/Postgres wiring lives in
  `infra/postgres/rewards/reward_candidate_audit_use_case.rs` and
  `bootstrap::AppState`.
- Reward candidate existence, platform audit permission, and reward audit-event
  reads now live behind `RewardCandidateAuditStore` in
  `infra/postgres/rewards/reward_candidate_audit_store.rs`.
- Course reward-candidate list HTTP handler and request/response DTO now live
  under `http/rewards`, while preserving the legacy
  `/courses/{course_id}/reward-candidates` GET URL.
- Course reward-candidate list reads are injected as an application-facing
  `CourseRewardCandidatesUseCase`; concrete DbPool/Postgres wiring lives in
  `infra/postgres/rewards/course_reward_candidate_use_case.rs` and
  `bootstrap::AppState`.
- Course reward-candidate list course existence, course permission probes, and
  candidate reads now live behind `CourseRewardCandidateStore` in
  `infra/postgres/rewards/course_reward_candidate_store.rs`.
- Platform reward-candidate review HTTP handler and request/response DTO now
  live under `http/rewards`, while preserving the legacy
  `/reward-candidates/review` URL.
- Platform reward-candidate review is injected as an application-facing
  `PlatformRewardCandidatesUseCase`; concrete DbPool/Postgres wiring lives in
  `infra/postgres/rewards/platform_reward_candidate_use_case.rs` and
  `bootstrap::AppState`.
- Platform reward-candidate review list/count, platform permission probes, and
  user/course summary reads now live behind the module-local
  `PlatformRewardCandidateStore` in
  `infra/postgres/rewards/platform_reward_candidate_store.rs`.
- Teacher reward-candidate decision HTTP handler and request/response DTO now
  live under `http/rewards`, while preserving the legacy
  `/courses/{course_id}/reward-candidates/{candidate_id}/teacher-decision`
  PUT URL.
- Teacher reward-candidate decisions are injected as an application-facing
  `TeacherRewardCandidateDecisionUseCase`; concrete DbPool/Postgres wiring
  lives in
  `infra/postgres/rewards/teacher_reward_candidate_decision_use_case.rs` and
  `bootstrap::AppState`.
- Teacher reward-candidate permission probing, transactional candidate
  update/audit insertion, fraud-block checks, and active policy lookups now
  live behind the module-local `TeacherRewardCandidateDecisionStore` in
  `infra/postgres/rewards/teacher_reward_candidate_decision_store.rs`.
- `api/reward_candidates` no longer owns the teacher-decision route; the
  temporary legacy course scope composes the `http/rewards` resource so the
  `/api/courses/...` prefix remains reachable until course routing moves fully
  into the HTTP ring.
- Reward amount-decision HTTP handler and request/response DTO now live under
  `http/rewards`, while preserving the legacy
  `/reward-candidates/{candidate_id}/amount-decision` PUT URL.
- Reward amount decisions are injected as an application-facing
  `RewardAmountDecisionUseCase`; concrete DbPool/Postgres wiring lives in
  `infra/postgres/rewards/reward_amount_decision_use_case.rs` and
  `bootstrap::AppState`.
- Reward amount-decision platform permission probing, transactional candidate
  update/audit insertion, execution-job enqueueing, fraud-block checks, and
  active policy lookups now live behind the module-local
  `RewardAmountDecisionStore` in
  `infra/postgres/rewards/reward_amount_decision_store.rs`.
- Shared reward-candidate fraud-block and active-policy lookup helpers now live
  under `infra/postgres/rewards/reward_candidate_fraud_blocks.rs` and
  `reward_candidate_policy_lookup.rs`.
- `api/reward_candidates` no longer owns the amount-decision route.
- Reward candidate submission HTTP handlers and request/response DTOs now live
  under `http/rewards`, while preserving the legacy
  `/courses/{course_id}/reward-candidates` and
  `/organizations/{organization_id}/courses/{course_id}/reward-candidates`
  POST URLs.
- Reward candidate submission is injected as an application-facing
  `RewardCandidateSubmissionUseCase`; concrete DbPool/Postgres wiring lives in
  `infra/postgres/rewards/reward_candidate_submission_use_case.rs` and
  `bootstrap::AppState`.
- Reward candidate submission permission probing, idempotency replay,
  fraud-block checks, target eligibility, evidence validation, candidate insert,
  and audit insertion now live behind the module-local
  `RewardCandidateSubmissionStore` plus granular Postgres helpers under
  `infra/postgres/rewards/reward_candidate_submission_*`.
- `src/api/reward_candidates.rs` and `src/api/reward_candidates/` have been
  removed; reward candidate routes are composed by `http/rewards`, with bridge
  resources in the legacy course and organization scopes until those broader
  scopes move.
- Reward compensation recording is now an application-facing
  `RewardCompensationUseCase`; concrete DbPool/Postgres wiring lives in
  `infra/postgres/rewards/reward_compensation_use_case.rs`.
- Reward compensation permission probing, idempotency replay, wallet linking,
  guarded wallet adjustment, transaction creation, and record insertion now live
  behind `RewardCompensationStore` plus granular Postgres helpers under
  `infra/postgres/rewards/reward_compensation_*`.
- `src/services/reward_compensation_service.rs` and
  `src/services/reward_compensation_service/` have been removed.
- Reward payout planning is now an application-facing `RewardPayoutPlanUseCase`;
  concrete DbPool/Postgres wiring lives in
  `infra/postgres/rewards/reward_payout_plan_use_case.rs`.
- Reward payout candidate loading, active policy lookup, presigner availability,
  and permission probing now live behind `RewardPayoutPlanStore` plus granular
  Postgres helpers under `infra/postgres/rewards/reward_payout_plan_*`.
- Reward payout method names now live in `domain/rewards/payout`, and
  reward-policy scope/payment-strategy callers import the canonical
  `domain/rewards/policy` constants instead of model-layer duplicate literals.
- `src/services/reward_execution_service/select_payout_method.rs` has been
  removed. The legacy reward-execution payout-planning entry points delegate
  through the new application handler and Postgres store while wallet credit,
  token confirmation, reconciliation, and notification side effects remain the
  next reward-execution migration surface.
- Reward token confirmation is now an application-facing
  `RewardTokenConfirmationUseCase`; concrete DbPool/Postgres wiring lives in
  `infra/postgres/rewards/reward_token_confirmation_use_case.rs`.
- Reward token confirmation permission probing, external transaction
  idempotency, payout record creation, candidate status transition, and audit
  insertion now live behind `RewardTokenConfirmationStore` plus granular
  Postgres helpers under `infra/postgres/rewards/reward_token_confirmation_*`.
- Token event to transaction-type vocabulary now lives in
  `domain/rewards/token`.
- The legacy reward-execution token-confirmation entry points now delegate
  through the new application handler and Postgres store. The old
  `record_external_reward_transaction.rs` include was reduced to the external
  transaction link-repair helper still needed by reconciliation, and orphaned
  uncompiled `src/services/reward_execution_service/tests*` files were removed.
- Reward wallet credit is now an application-facing
  `RewardWalletCreditUseCase`; concrete DbPool/Postgres wiring lives in
  `infra/postgres/rewards/reward_wallet_credit_use_case.rs`.
- Reward wallet credit permission probing, wallet linking, active off-chain
  policy eligibility, guarded/idempotent credit-record creation, internal and
  generic transaction creation, candidate status transition, and audit
  insertion now live behind `RewardWalletCreditStore` plus granular Postgres
  helpers under `infra/postgres/rewards/reward_wallet_credit_*`.
- Reward wallet-credit transaction vocabulary now lives in
  `domain/rewards/wallet_credit`.
- The legacy reward-execution wallet-credit entry points now delegate through
  the new application handler and Postgres store. Reconciliation temporarily
  calls the new wallet-credit infra transaction bridge until the reconciliation
  slice moves fully into `application/rewards/reconcile_candidate`.
- Reward wallet-credit notification is now an application-facing
  `RewardWalletCreditNotificationUseCase`; concrete DbPool/Postgres wiring
  lives in
  `infra/postgres/rewards/reward_wallet_credit_notification_use_case.rs`.
- Reward wallet-credit notification permission probing, notification
  idempotency, credit-record notification updates, candidate notified status
  transition, and audit insertion now live behind
  `RewardWalletCreditNotificationStore` plus granular Postgres helpers under
  `infra/postgres/rewards/reward_wallet_credit_notification_*`.
- Reward wallet-credit notification title/body vocabulary now lives in
  `domain/rewards/wallet_credit`, and the new reward adapter no longer calls
  legacy `utils::notifications`.
- The legacy reward-execution wallet-credit notification entry points now
  delegate through the new application handler and Postgres store.
  Reconciliation temporarily calls the new notification infra transaction
  bridge until the reconciliation slice moves fully into
  `application/rewards/reconcile_candidate`.
- Reward reconciliation is now an application-facing
  `RewardReconciliationUseCase`; concrete DbPool/Postgres wiring lives in
  `infra/postgres/rewards/reward_reconciliation_use_case.rs`.
- Reward reconciliation candidate eligibility, external/internal transaction
  link repair, wallet-credit repair, wallet-credit notification repair, and
  reconciliation audit insertion now live behind `RewardReconciliationStore`
  plus granular Postgres helpers under
  `infra/postgres/rewards/reward_reconciliation_*`.
- The legacy reward-execution reconciliation entry points now delegate through
  the new application handler and Postgres store, and stale private
  reward-execution include fragments for wallet-credit/notification candidate
  bridges plus external/internal link repair were removed.
- Notification preference and inbox HTTP handlers plus request/response DTOs
  now live under `http/notifications`.
- Notification preference reads/writes are injected as an application-facing
  `NotificationPreferencesUseCase`; concrete DbPool/Postgres wiring lives in
  `infra/postgres/notifications/notification_preferences_use_case.rs` and
  `bootstrap::AppState`.
- Notification inbox read-management is injected as an application-facing
  `NotificationInboxUseCase`; concrete DbPool/Postgres wiring lives in
  `infra/postgres/notifications/notification_inbox_use_case.rs` and
  `bootstrap::AppState`.
- `src/api/session/notifications.rs` has been removed.
- `src/api/roles.rs` is now a thin compatibility wrapper around
  `http::access_control`.
- Role catalog HTTP handlers, routes, and response DTOs now live under
  `http/access_control`.
- Role catalog reads are injected as an application-facing `RoleCatalogUseCase`;
  concrete DbPool/Postgres wiring lives in
  `infra/postgres/access_control/role_catalog_use_case.rs` and
  `bootstrap::AppState`.
- `src/api/users.rs` no longer contains direct Diesel usage or imports the
  Diesel `User` record for user reads; it delegates list/search/profile reads
  through `application/identity` and `infra/postgres/identity`.
- User profile HTTP response DTOs now live under `http/identity/dto`.
- `src/main.rs` is now 49 lines and delegates app state initialization and
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
- `src/api/health.rs` is now a thin compatibility wrapper around
  `http::operations`.
- Operations liveness/readiness handlers, routes, and DTOs now live under
  `http/operations`.
- Operations readiness is injected as an application-facing `ReadinessUseCase`;
  concrete Postgres/S3/Ethereum dependency wiring lives in
  `bootstrap/readiness.rs`.
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
- The content media URL route no longer owns direct chapter/content Diesel,
  object-key validation, presigned GET generation, S3 env fallback setup, or
  JSON response construction; it delegates through
  `application/content/request_media_url`,
  `infra/postgres/content/media_object_store.rs`, and
  `infra/object_storage/content/media_url_provider.rs`.
- Content media URL HTTP response DTOs now live under `http/content/dto`.
- The content video-processing route no longer owns direct chapter/content
  Diesel, video-type validation, object-key validation, upload-job insertion,
  or `NewUploadJob` construction; it delegates through
  `domain/content/content_item`, `application/content/process_upload_job`, and
  `infra/postgres/content/upload_job_store.rs`.
- `src/api/contents.rs` no longer uses `include!`; it is now a thin
  compatibility wrapper around `http::content::routes::configure_routes`.
- Chapter and content-item route configuration now lives in
  `http/content/routes.rs`.
- Content API handler files now live under `http/content/handlers`, and
  `src/api/contents/imports.rs` has been removed.
- `rg "actix_web|diesel|diesel_async|aws_|ethers|std::env" src/domain src/application`
  returns no matches.
- `rg "crate::repositories|crate::db::schema" src/http` returns no matches.
- Content HTTP wiring debt cleared: `rg "crate::infra::postgres" src/http` and
  the broader content-handler concrete-adapter scan now return no matches.
  Legacy `src/api` modules still contain direct service/repository/infra usage
  and remain the next Level 2 migration surface.
- `rg "crate::http|crate::services" src/domain src/application src/infra`
  returns no matches.
- `rg "diesel|diesel_async|schema::|RunQueryDsl|chapters::table|models::chapter|NewChapter\\b|UpdateChapter\\b" src/http/content/handlers/chapter.rs src/http/content/routes.rs`
  returns no matches.
- `rg "crate::infra::postgres|crate::db::DbPool|PostgresChapterStore" src/http/content/handlers/chapter.rs`
  returns no matches.
- `rg "crate::infra::postgres|crate::db::DbPool|crate::infra::object_storage|S3Content|PostgresContent|PostgresChapterStore" src/http/content/handlers`
  returns no matches.
- `rg "diesel|diesel_async|schema::|RunQueryDsl" src/api/roles.rs` returns no
  matches.
- `rg "crate::infra::postgres|crate::db::DbPool|PostgresRoleCatalogStore|crate::db" src/api/roles.rs src/http/access_control`
  returns no matches.
- `rg "api::roles|crate::api::roles" src/api/mod.rs tests/middleware_access_control/role_read_routes_require_view_role_assignments_permission.rs`
  returns no matches.
- `rg "NotificationsState::from|PostgresNotificationPreferenceStore|PostgresNotificationInboxStore|crate::infra::postgres|crate::db::DbPool" src/http/notifications`
  returns no matches.
- `rg "mod notifications|api/session/notifications" src/api/session.rs src/api`
  returns no matches.
- `rg "actix_web|diesel|diesel_async|aws_|ethers|std::env" src/application/rewards/decide_teacher_candidate src/domain/rewards/candidate`
  returns no matches.
- `./scripts/run-host-tests.sh cargo test --lib application::rewards::decide_teacher_candidate`
  passes.
- `./scripts/run-host-tests.sh cargo test api_scope_and_following_routes_are_reachable --test api_routing`
  passes.
- `./scripts/run-host-tests.sh cargo check --features app-bin --bin rust-learn`
  passes.
- The manual line-limit scan only reports generated `src/db/schema.rs`; touched
  non-generated Rust files stay under 180 lines.
- `./scripts/run-host-tests.sh cargo test --lib application::rewards::decide_amount`
  passes.
- `./scripts/run-host-tests.sh cargo test teacher_submits_and_approves_then_platform_reviewer_sets_amount --test reward_candidates`
  passes.
- `./scripts/run-host-tests.sh cargo test delegated_platform_amount_reviewer_can_set_amount_after_teacher_approval --test delegated_permissions`
  passes.
- `rg "api::session::get_notification_preferences|api::session::save_notification_preferences|session::get_notification_preferences|session::save_notification_preferences" src/api/mod.rs tests/current_session_api/current_session_test_app.rs`
  returns no matches.
- `rg "session::get_current_session|api::session::get_current_session|crate::services::session_service|crate::db::DbPool|diesel|diesel_async|schema::|RunQueryDsl" src/api/session.rs src/http/identity src/api/mod.rs`
  returns no matches.
- `rg "api::session::get_current_session|session::get_current_session" src/api/mod.rs tests/current_session_api/current_session_test_app.rs`
  returns no matches.
- `rg "session_service|services::session_service|pub mod session_service|include!\\(\"session_service" src tests`
  returns no matches.
- `rg "reward_policy_service|services::reward_policy_service|include!\\(\"reward_policy_service" src tests`
  returns no matches.
- `rg "reward_fraud_block_service|services::reward_fraud_block_service|include!\\(\"reward_fraud_block_service" src tests`
  returns no matches.
- `rg "reward_history_service|services::reward_history_service|include!\\(\"reward_history_service" src tests`
  returns no matches.
- `rg "list_reward_candidate_audit|reward-candidates/\\{candidate_id\\}/audit" src/services/reward_candidate_service`
  returns no matches.
- `rg "list_course_reward_candidates|ListRewardCandidatesRequest|reward_candidate_service/list_course_reward_candidates" src/services/reward_candidate_service`
  returns no matches.
- `rg "list_platform_reward_candidates|PlatformRewardCandidatesRequest|PlatformRewardCandidateItem|build_platform_reward_candidate_items|reward-candidates/review" src/services/reward_candidate_service`
  returns no matches.
- `rg "crate::db|DbPool|diesel|diesel_async|RunQueryDsl|schema::|crate::services|crate::repositories" src/api/reward_policies.rs src/http/rewards`
  returns no matches.
- `rg "crate::db|DbPool|diesel|diesel_async|RunQueryDsl|schema::|crate::services|crate::repositories" src/api/reward_fraud_blocks.rs src/http/rewards`
  returns no matches.
- `rg "crate::repositories|crate::db::schema|diesel|diesel_async" src/http/rewards`
  returns no matches.
- `rg "reward-candidates/me/history" src/http/rewards/routes.rs`
  returns only the `http/rewards/routes.rs` owner.
- `rg "reward-candidates/\\{candidate_id\\}/audit" src/http/rewards/routes.rs`
  returns only the `http/rewards/routes.rs` owner.
- `rg "courses/\\{course_id\\}/reward-candidates" src/http/rewards/routes.rs`
  returns only the `http/rewards/routes.rs` owner for reward candidate course
  list/submission and teacher-decision resources.
- `rg "organizations/\\{organization_id\\}/courses/\\{course_id\\}/reward-candidates" src/http/rewards/routes.rs`
  returns only the `http/rewards/routes.rs` owner for organization-scoped
  reward candidate submission.
- `rg "reward-candidates/review" src/http/rewards/routes.rs`
  returns only the `http/rewards/routes.rs` owner.
- `rg "reward_candidates" src/api` returns no matches.
- `./scripts/run-host-tests.sh cargo test --lib application::rewards::submit_candidate`
  passes.
- `./scripts/run-host-tests.sh cargo test --lib domain::rewards::candidate`
  passes.
- `./scripts/run-host-tests.sh cargo test api_scope_and_following_routes_are_reachable --test api_routing`
  passes with route-only submission app data.
- `./scripts/run-host-tests.sh cargo test teacher_submits_and_approves_then_platform_reviewer_sets_amount --test reward_candidates`
  passes through `PostgresRewardCandidateSubmissionUseCase`.
- `./scripts/run-host-tests.sh cargo test organization_submission_requires_linked_course_and_still_waits_for_teacher --test reward_candidates`
  passes through `PostgresRewardCandidateSubmissionUseCase`.
- `./scripts/run-host-tests.sh cargo test delegated_course_permission_submits_candidate_without_course_role --test delegated_permissions`
  passes through `PostgresRewardCandidateSubmissionUseCase`.
- `./scripts/run-host-tests.sh cargo test --lib application::rewards::record_compensation`
  passes.
- `./scripts/run-host-tests.sh cargo test --test reward_compensations` passes
  through `PostgresRewardCompensationUseCase`.
- `./scripts/run-host-tests.sh cargo test --lib application::rewards::plan_payout`
  passes.
- `./scripts/run-host-tests.sh cargo test --test reward_execution` passes with
  payout-planning regressions routed through `PostgresRewardPayoutPlanUseCase`.
- `./scripts/run-host-tests.sh cargo test --lib services::reward_execution_service`
  passes.
- `./scripts/run-host-tests.sh cargo test --lib domain::rewards::token` passes.
- `./scripts/run-host-tests.sh cargo test --lib application::rewards::record_token_confirmation`
  passes.
- `./scripts/run-host-tests.sh cargo test --test reward_execution` passes with
  token-confirmation regressions routed through
  `PostgresRewardTokenConfirmationUseCase`.
- `./scripts/run-host-tests.sh cargo test --tests -- --list > /tmp/rust-learn-integration-tests.list`
  compiles and lists the integration test suite.
- `rg "actix_web|diesel|diesel_async|aws_|ethers|std::env|crate::services" src/application/rewards/record_compensation src/domain/rewards/compensation.rs`
  returns no matches.
- `rg "crate::services::wallet_service|crate::services::reward_compensation_service|services::reward_compensation_service" src/infra/postgres/rewards/reward_compensation_* src/application/rewards/record_compensation`
  returns no matches.
- `rg "actix_web|diesel|diesel_async|aws_|ethers|std::env|crate::services" src/application/rewards/plan_payout src/domain/rewards/payout.rs`
  returns no matches.
- `rg "crate::services::reward_execution_service|crate::services::wallet_service" src/infra/postgres/rewards/reward_payout_plan_* src/application/rewards/plan_payout`
  returns no matches.
- `rg "actix_web|diesel|diesel_async|aws_|ethers|std::env|crate::services" src/application/rewards/record_token_confirmation src/domain/rewards/token.rs`
  returns no matches.
- `rg "crate::services::reward_execution_service|crate::services::wallet_service" src/infra/postgres/rewards/reward_token_confirmation_* src/application/rewards/record_token_confirmation`
  returns no matches.
- `rg "reward_policies::reward_policy_scope|api::reward_policies|crate::api::reward_policies" src/api/mod.rs tests/api_routing.rs`
  returns no matches.
- `rg "reward_fraud_blocks::reward_fraud_block_scope|api::reward_fraud_blocks|crate::api::reward_fraud_blocks" src/api/mod.rs tests/api_routing.rs`
  returns no matches.
- `rg "diesel|diesel_async|schema::|RunQueryDsl|QueryDsl|ExpressionMethods|models::user::User" src/api/users.rs`
  returns no matches.
- `rg "diesel|diesel_async|RunQueryDsl|assessment_attempts::table|assessments::table" src/api/courses/list_assessment_attempts.rs`
  returns no matches.
- `rg "diesel|diesel_async|RunQueryDsl|assessment_attempts::table|assessment_questions::table|assessments::table|crate::db::schema" src/api/courses/list_course_assessments.rs src/api/courses/list_assessment_attempts.rs`
  returns no matches.
- `rg "diesel|diesel_async|RunQueryDsl|try_get_provider|provider|get_chainid|health_check\\(|timeout|crate::db::schema" src/api/health.rs`
  returns no matches.
- `rg "crate::infra::postgres|crate::db::DbPool|crate::infra::object_storage|crate::infra::ethereum" src/api/health.rs src/http/operations`
  returns no matches.
- `rg "crate::api::health|api::health" src/bootstrap/routes.rs tests/api_routing.rs`
  returns no matches.
- `rg "diesel|diesel_async|insert_into|update\\(|delete\\(|contents::table|user_role_course|NewContent\\b|UpdateContent\\b|models::content" src/http/content/handlers/create_content.rs src/http/content/handlers/get_upload_url.rs`
  returns no matches.
- `rg "ensure_bucket|presign_external_put|S3State::new_from_env|ensure_chapter_belongs_to_course|chapters::table|diesel|diesel_async|RunQueryDsl|serde_json::json" src/http/content/handlers/get_upload_url.rs`
  returns no matches.
- `rg "ensure_chapter_belongs_to_course|contents::table|diesel|diesel_async|RunQueryDsl|presign_external_get|S3State::new_from_env|serde_json::json|models::content::Content" src/http/content/handlers/get_media_url.rs`
  returns no matches.
- `rg "chapters::table|contents::table|upload_jobs::table|diesel|diesel_async|RunQueryDsl|insert_into|NewUploadJob|models::content::Content|is_video_content_type" src/http/content/handlers/process_content.rs`
  returns no matches.
- `rg "include!|imports\\.rs" src/api/contents.rs src/http/content`
  returns no matches.
- `cargo fmt --all --check` passes.
- `git diff --check` passes.
- Manual source file length guard returns no non-generated files over 180
  lines; generated `src/db/schema.rs` remains exempt.
- `./scripts/run-host-tests.sh cargo check --features app-bin --bin rust-learn`
  passes.
- `./scripts/run-host-tests.sh cargo test domain::rewards::policy --lib`
  passes.
- `./scripts/run-host-tests.sh cargo test application::rewards::manage_reward_policy --lib`
  passes.
- `./scripts/run-host-tests.sh cargo test --test reward_policies` passes.
- `./scripts/run-host-tests.sh cargo test domain::rewards::fraud_block --lib`
  passes.
- `./scripts/run-host-tests.sh cargo test application::rewards::manage_fraud_block --lib`
  passes.
- `./scripts/run-host-tests.sh cargo test reward_fraud_block_permissions --lib`
  passes.
- `./scripts/run-host-tests.sh cargo test reward_fraud_block_notification_permissions --lib`
  passes.
- `./scripts/run-host-tests.sh cargo test --test reward_fraud_blocks` passes.
- `./scripts/run-host-tests.sh cargo test fraud_block_api_separates_read_audit_from_block_management --test reward_management_api`
  passes.
- `./scripts/run-host-tests.sh cargo test --test reward_candidates` passes.
- `./scripts/run-host-tests.sh cargo test domain::rewards::candidate::status --lib`
  passes.
- `./scripts/run-host-tests.sh cargo test application::rewards::list_reward_history --lib`
  passes.
- `./scripts/run-host-tests.sh cargo test --test student_reward_history` passes.
- `./scripts/run-host-tests.sh cargo test application::rewards::list_candidate_audit --lib`
  passes.
- `./scripts/run-host-tests.sh cargo test --test reward_candidate_audit`
  passes.
- `./scripts/run-host-tests.sh cargo test application::rewards::list_course_candidates --lib`
  passes.
- `./scripts/run-host-tests.sh cargo test --test reward_course_candidates`
  passes.
- `./scripts/run-host-tests.sh cargo test application::rewards::list_platform_candidates --lib`
  passes.
- `./scripts/run-host-tests.sh cargo test services::reward_candidate_service --lib`
  passes.
- `./scripts/run-host-tests.sh cargo test --test course_content_management`
  passes.
- `./scripts/run-host-tests.sh cargo test course_video_upload_can_be_queued_and_processed --test video_upload_flow`
  passes.
- `./scripts/run-host-tests.sh cargo test --test api_routing` passes.
- `./scripts/run-host-tests.sh cargo test --lib content` passes.
- `./scripts/run-host-tests.sh cargo test --test health_readiness` passes.
- `./scripts/run-host-tests.sh cargo test --lib operations` passes.
- `./scripts/run-host-tests.sh cargo test notification_preferences_default_and_save_round_trip --test current_session_api`
  passes.
- `./scripts/run-host-tests.sh cargo test application::notifications` passes.
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
- `./scripts/run-host-tests.sh cargo test --test course_content_management`
  passes.
- `./scripts/run-host-tests.sh cargo test --lib request_upload_url` passes.
- `./scripts/run-host-tests.sh cargo test --lib request_media_url` passes.
- `./scripts/run-host-tests.sh cargo test --lib process_upload_job` passes.
- `./scripts/run-host-tests.sh cargo test --lib domain::content` passes.
- `./scripts/run-host-tests.sh cargo test course_video_upload_can_be_queued_and_processed --test video_upload_flow`
  passes, including the route-level media URL and processing queue assertions.
- `./scripts/run-host-tests.sh cargo test notification_preferences_default_and_save_round_trip --test current_session_api`
  passes.
- `./scripts/run-host-tests.sh cargo test --test current_session_api`
  passes.
- `./scripts/run-host-tests.sh cargo test notification_inbox_routes_list_mark_read_and_clear --test current_session_api`
  passes.
- `./scripts/run-host-tests.sh cargo test application::notifications` passes
  with fake-port coverage for notification inbox list, mark-read, and clear.
- `./scripts/run-host-tests.sh cargo test current_session_ --test current_session_api`
  passes.
- `./scripts/run-host-tests.sh cargo test application::identity::current_session`
  passes.
- `./scripts/run-host-tests.sh cargo test current_session_scope_builder --lib`
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

## Current Deep Extraction: Rewards

Rewards is the current deep extraction because it crosses candidate submission,
fraud blocks, reward policy, audit events, wallet credits, notifications,
reporting, and platform review. This section tracks rewards work only; it does
not narrow the complete Level 2 target. Apply the same ring, adapter, route, and
boundary checks from the matrix above to every canonical context.

- [x] Create the rewards context across the top-level rings:
      `domain/rewards`, `application/rewards`, `infra/postgres/rewards`, and
      `http/rewards`.
- [x] Use Level 2 granularity inside rewards: split domain by aggregate
      (`candidate`, `policy`, `fraud_block`, `payout`, `token`,
      `wallet_credit`, `compensation`) and application by use case
      (`submit_candidate`, `decide_teacher_candidate`, `decide_amount`,
      `manage_reward_policy`, `manage_fraud_block`, `plan_payout`,
      `record_token_confirmation`, `record_compensation`, `credit_wallet`,
      `notify_wallet_credit`, `reconcile_candidate`, reward history, and
      candidate review read models).
- [x] Move reward policy scope, event, and payment-strategy normalization into
      `domain/rewards/policy`.
- [x] Move reward fraud-block scope vocabulary and target matching into
      `domain/rewards/fraud_block`.
- [x] Move reward candidate status normalization into
      `domain/rewards/candidate/status`.
- [ ] Move remaining reward statuses and event types into domain enums/newtypes.
      Keep database string conversion at the infra boundary.
- [x] Move reward policy request/response structs out of service imports and
      into `http/rewards/dto`.
- [x] Move reward fraud-block request/response structs out of service imports
      and into `http/rewards/dto`.
- [x] Move student reward history request/response structs out of service
      imports and into `http/rewards/dto`.
- [x] Move reward candidate audit response structs out of service imports and
      into `http/rewards/dto`.
- [x] Move course reward-candidate list request/response structs out of
      service imports and into `http/rewards/dto`.
- [x] Move platform reward-candidate review request/response structs out of
      service imports and into `http/rewards/dto`.
- [x] Move teacher reward-candidate decision request/response structs out of
      service imports and into `http/rewards/dto`.
- [x] Move reward amount decision request/response structs out of service
      imports and into `http/rewards/dto`.
- [x] Move reward candidate submission request/response structs out of service
      imports and into `http/rewards/dto`.
- [ ] Move remaining reward request/response structs out of service imports and
      into `http/rewards/dto`.
- [ ] Move candidate transition rules into pure domain functions:
      submit, teacher approve/reject, amount approve/reject, token confirmed,
      wallet credited, notified, reconciliation needed.
- [x] Define the first reward repository port:
      `RewardPolicyStore` for `manage_reward_policy`.
- [x] Define `RewardFraudBlockStore` for `manage_fraud_block`.
- [x] Define `StudentRewardHistoryStore` for `list_reward_history`.
- [x] Define `RewardCandidateAuditStore` for `list_candidate_audit`.
- [x] Define `CourseRewardCandidateStore` for `list_course_candidates`.
- [x] Define module-local `PlatformRewardCandidateStore` for
      `list_platform_candidates`.
- [x] Define module-local `TeacherRewardCandidateDecisionStore` for
      `decide_teacher_candidate`.
- [x] Define module-local `RewardAmountDecisionStore` for `decide_amount`.
- [x] Define module-local `RewardCandidateSubmissionStore` for
      `submit_candidate`.
- [x] Define module-local `RewardCompensationStore` for `record_compensation`.
- [x] Define module-local `RewardPayoutPlanStore` for `plan_payout`.
- [x] Define module-local `RewardTokenConfirmationStore` for
      `record_token_confirmation`.
- [x] Define module-local `RewardWalletCreditStore` for `credit_wallet`.
- [x] Define module-local `RewardWalletCreditNotificationStore` for
      `notify_wallet_credit`.
- [x] Define module-local `RewardReconciliationStore` for
      `reconcile_candidate`.
- [ ] Define remaining context-owned repository ports before moving Diesel code.
      Wallet audit belongs to `application/wallet/audit_wallet`; reward and
      fraud dashboards belong to `application/reporting/*`; only reward command
      and reward read-model ports stay in `application/rewards`.
- [x] Move reward policy Diesel implementation behind `infra/postgres/rewards`.
- [x] Move reward fraud-block Diesel implementation behind
      `infra/postgres/rewards`.
- [x] Move student reward history Diesel implementation behind
      `infra/postgres/rewards`.
- [x] Move reward candidate audit Diesel implementation behind
      `infra/postgres/rewards`.
- [x] Move course reward-candidate list Diesel implementation behind
      `infra/postgres/rewards`.
- [x] Move platform reward-candidate review Diesel implementation behind
      `infra/postgres/rewards`.
- [x] Move teacher reward-candidate decision Diesel implementation behind
      `infra/postgres/rewards`.
- [x] Move reward amount decision Diesel implementation behind
      `infra/postgres/rewards`.
- [x] Move reward candidate submission Diesel implementation behind
      `infra/postgres/rewards`.
- [x] Move reward compensation Diesel implementation behind
      `infra/postgres/rewards`.
- [x] Move reward payout planning Diesel implementation behind
      `infra/postgres/rewards`.
- [x] Move reward token confirmation Diesel implementation behind
      `infra/postgres/rewards`.
- [x] Move reward wallet credit Diesel implementation behind
      `infra/postgres/rewards`.
- [x] Move reward wallet-credit notification Diesel implementation behind
      `infra/postgres/rewards`.
- [x] Move reward reconciliation Diesel implementation behind
      `infra/postgres/rewards`.
- [ ] Move remaining reward Diesel implementations behind
      `infra/postgres/rewards`.
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
5. Finish the active rewards extraction across `domain`, `application`,
   `infra/postgres`, `infra/ethereum`, `http`, and `notifications`.
6. Complete already-started contexts next: identity, learning, content,
   notifications, operations, access control, reporting, and wallet.
7. Extract organizations, teacher applications, and KYC with the same Level 2
   matrix instead of leaving them in legacy services.
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

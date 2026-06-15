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

- [x] Stop adding new `include!`-based service modules. They split files but
      keep one shared module namespace, shared imports, and hidden coupling.
- [x] Move direct Diesel usage out of API handlers. Examples to migrate include
      chapters, notification preferences, course assessments, user search, and
      content mutations.
- [x] Separate DB records from API DTOs. Avoid returning Diesel models directly
      from handlers for user-facing contracts.
- [x] Stop mixing Active Record and Repository patterns. DB methods currently
      live both on `models::*` and in `repositories::*`; converge on repository
      or infra modules.
- [ ] Centralize authorization policy. Permission checks currently live in
      middleware, services, repositories, and frontend helper lists.
- [x] Pull startup side effects out of `main.rs`. DB setup, S3 setup,
      notification state, and Ethereum startup deployment should be composed
      through a small bootstrap module.
- [x] Give cross-cutting utilities a home based on responsibility. Some
      `utils::*` modules are infrastructure adapters, some are domain helpers,
      and some are application services.
- [x] Make frontend capability checks consume backend-derived session
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
| `learning` | courses, enrollment, progress, assessment rules | course discovery/listing, course detail reads, lifecycle, deletion, enrollment decisions, progress writes, assessment reads/submission, course-organization reads | courses, course-organization links, enrollments, progress, assessments, attempts | none by default | course, course-organization, enrollment, progress, and assessment routes |
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
      get_course/
      create_course/
      manage_course/
      update_course/
      delete_course/
      update_course_lifecycle/
      assign_course_role/
      course_enrollment/
      learner_progress/
      list_course_organizations/
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
        course_creation_store.rs
        course_discovery_store.rs
        course_read_store.rs
        course_deletion_store.rs
        course_update_store.rs
        course_lifecycle_store.rs
        course_role_assignment_store.rs
        course_enrollment_store.rs
        course_enrollment_*_queries.rs
        permission checks via infra/postgres/access_control/
        course_organization_store.rs
        learner_progress_store.rs
        assessment_read_store.rs
        assessment_read_use_case.rs
        assessment_submission_store.rs
        assessment_submission_use_case.rs
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

- [x] Each HTTP context exposes one route configurator from `http/<context>`,
      e.g. `pub fn configure_routes(cfg: &mut web::ServiceConfig)`.
- [x] `http::api_scope()` composes context route configurators and
      cross-cutting middleware.
- [x] Build an `AppState` or context-specific state structs once outside the
      `HttpServer::new` closure, then pass them with `web::Data`.
- [x] Prefer typed extractors over manual `HttpRequest` parsing where possible:
      `AuthUser`, `DbConn`, `Json<T>`, `Path<T>`, and `Query<T>`.
- [x] API handlers return `Result<web::Json<T>, ApiError>` or equivalent,
      instead of manually matching every service error to `HttpResponse`.
      Checked contexts: access control, notifications, operations, identity,
      KYC, teacher applications, content, organizations, wallet, reporting,
      rewards, and learning. Manual `HttpResponse` construction is now limited
      to the shared `ApiError` `ResponseError` and the intentional
      identity-auth plain-text `ResponseError`.
- [x] Domain/application errors do not implement Actix traits directly. The
      HTTP layer maps them into a local `ResponseError` type.
- [x] Configure JSON limits and JSON parse errors centrally so every route has
      consistent bad-request behavior.

## First Low-Risk Building Blocks

- [x] Move or wrap the existing `src/utils/api_error.rs` helper into
      `http/errors.rs`, then evolve it into one Actix `ResponseError`
      implementation with consistent JSON error bodies.
- [x] Add an `http/extractors/auth_user.rs` extractor that reads the `UserJWT`
      extension once and removes repeated `authenticated_user(&req)` boilerplate.
- [x] Add a DB connection extractor or helper so handlers do not repeat
      `pool.get().await` and 500 mapping.
- [x] Add `bootstrap/app_state.rs` to group `DbPool`, `S3State`,
      `NotificationsState`, and future infra handles.
- [x] Replace `include!` in one small module with normal `mod` files and
      `pub(crate) use` re-exports to establish the house pattern.

## Completed / Verified Snapshot

This section replaces the long per-slice audit log. The detailed implementation
history is in git; this file keeps only the architectural outcomes, proof style,
and active gaps needed to continue Level 2 without re-reading hundreds of
completed entries.

### Completed Architecture Outcomes

| Area | Completed / checked outcome |
| --- | --- |
| Route ownership | Legacy `src/api` route wrappers were removed. `/api` is assembled from `http/<context>` route configurators, and `main.rs` is reduced to process startup and bootstrap calls. |
| HTTP boundary | Handlers now mostly do extraction, use-case calls, DTO mapping, and local HTTP error mapping. JSON limits and JSON parse errors are centralized. Manual response construction is confined to shared `ApiError` and the intentional identity-auth plain-text contract. |
| Typed auth | Route handlers use typed `AuthUser`/`AuthUserId` extractors. The final request-auth helper was replaced by a current-session extractor that preserves `/api/me` JSON unauthorized responses. |
| Bootstrap wiring | App state and app-data registration moved into bootstrap/context wiring. Concrete infra construction is owned by bootstrap, not handlers. |
| Context rings | Rewards, wallet, reporting, learning, organizations, KYC, teacher applications, identity, content, notifications, operations, and access-control now have Level 2 ring ownership for their migrated flows. |
| Services/repositories/utils cleanup | Legacy `src/api`, `src/services`, `src/repositories`, and `src/utils` modules were deleted after callers moved to ring owners or direct infra/application replacements. |
| Persistence ownership | Async DB behavior was moved out of `models::*`; model files are persistence row/change-set shapes. Diesel query logic lives in `infra/postgres/<context>` or legacy-free test fixtures. |
| Access control | Shared `AccessActor`, `AccessAction`, and `AccessScope` vocabulary plus `can(...)`/`can_any(...)` Postgres decisions are used across middleware and migrated application authorization stores. Scope-specific Postgres permission wrappers were removed. |
| Frontend gates | Platform, organization, learner, teacher-application, and ops frontend action gates now consume backend-derived current-session capabilities instead of local permission matrices. |
| Test harness shape | Production source no longer uses `include!`. Many integration-test harnesses have been converted from `include!`/`imports.rs` to explicit modules plus `support.rs` files; remaining harnesses are tracked as active work. |

### Recent Verified Batches

| Batch | Commit | Compact proof |
| --- | --- | --- |
| 287 | `581f1be3` | Removed final request-auth helper; proved no `authenticated_user*`, no stray `HttpRequest` handler parsing, and no direct `pool.get().await` in `src/http`. |
| 288-302 | `7cdddb44`..`8b3717e5` | Centralized JSON extractor errors and moved access-control, notifications, operations, identity, KYC, teacher-applications, content, organizations, wallet, reporting, rewards, and learning handlers to typed HTTP results/error mappers. |
| 303-312 | `19f31541`..`d0d0f32f` | Introduced typed access decisions, moved reward/wallet/learning/teacher-application/identity/KYC/organization authorization ports to that contract, consolidated Postgres decision adapters, and removed scope-specific wrappers. |
| 313-315 | `8ac98ee2`..`4f8eb696` | Repointed frontend platform, organization, ops, learner, and teacher-application gates to backend current-session capability facts; proved with frontend tests, TypeScript, backend current-session tests, scans, and Cargo gates. |
| 316 | `88308a68` | Converted `course_permissions`, `organization_permissions`, and `middleware_access_control` test harnesses to modules/support and routed test wiring through `AccessControlUseCases`. |
| 317 | `6868fe74` | Converted reward/repository reward test harnesses to explicit modules/support; refreshed stale reward API error-envelope assertions. |
| 318 | `7415aebb` | Converted small permission/management harnesses: `model_permission_tests`, `repository_core_tests`, `repository_delegation_tests`, `platform_permissions`, `organization_management`, `course_enrollment_api`. |
| 319 | `dc6b11ea` | Converted `course_content_management` and `student_reward_history` harnesses; kept touched files under the 180-line cap. |
| 320 | `ca1c80c9` | Converted `course_join_requests`, `organization_dashboard`, and `worker_upload_jobs` harnesses to explicit modules/support; proved with focused tests, Cargo gates, boundary scans, and touched-file size checks. |
| 321 | `61edcae1` | Converted `current_session_api` and `organization_members` harnesses to explicit modules/support; proved with focused tests, Cargo gates, boundary scans, and touched-file size checks. |
| 322 | `f3cb4bb1` | Converted `authentication_flow` to explicit modules/support, split the hidden registration test out of shared setup, and proved with focused auth tests, Cargo gates, boundary scans, and touched-file size checks. |
| 323 | this batch | Converted `teacher_applications` and `organization_teacher_applications` to explicit modules/support; preserved the shared nomination/decision helpers across both harnesses and proved with focused tests, Cargo gates, boundary scans, and touched-file size checks. |

### Repeated Verification Already Used

Completed batches were checked with the relevant subset of:

- `cargo fmt --all --check`
- focused host tests for touched contexts
- `./scripts/run-host-tests.sh cargo check --lib`
- `./scripts/run-host-tests.sh cargo check --bin rust-learn --features app-bin`
- `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`
- `git diff --check`
- touched-file size checks for manually maintained non-Markdown files
- source-boundary scans for `src/api`, `src/services`, `src/repositories`,
  `src/utils`, production `include!`, HTTP DB-pool access, request-auth helper
  usage, and ring import leaks
- frontend tests, TypeScript checks, and permission/capability scans for the
  frontend capability batches

### Active Remaining Work

Keep these visible; do not mark complete without fresh code inspection, tests,
and boundary scans.

- Finish global integration-test harness cleanup: remaining test crates still
  using `include!`/`imports.rs` need explicit modules and `support.rs` files.
  Current scan after Batch 323: 9 `tests/*/imports.rs` files and 86 test
  `include!` occurrences remain.
- Finish authorization hardening: middleware should stay an early rejection
  optimization, while application use cases remain the real business guard.
- Finish reward boundary hardening: remaining statuses/events/newtypes,
  candidate transition rules, DTO leaks, Diesel adapters, and fake-port
  use-case tests need direct evidence.
- Keep auditing data boundaries: Diesel schema/model leaks must stay in infra or
  persistence records, and public API DTOs must remain HTTP-owned.
- Keep route URLs and response semantics backward compatible unless a migration
  note explicitly records a behavior change.
- Keep running the full pre-push gate before every commit that advances this
  architecture objective.

## Legacy Transition Rules

- [x] Legacy `api` wrappers are gone; new ring-based modules may still be
      called from existing `services` and `repositories` while deeper
      extraction is in progress.
- [x] New ring-based modules must not call old `services::*` modules.
- [x] New domain and application modules must not call old `models::*` async DB
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

## Context Status Snapshot

The context details below are intentionally compact. Full route-by-route history
is in git and in the completed batch evidence above.

| Context | Completed / checked Level 2 status |
| --- | --- |
| Rewards | Ring folders, core vocabulary, HTTP DTOs, typed handlers/errors, command/read use cases, Postgres adapters, and access-control-backed permission checks exist for the migrated policy, fraud-block, history, audit, candidate, decision, compensation, payout, token, wallet-credit, notification, and reconciliation flows. |
| Reporting | Platform summary, fraud dashboard, reward dashboard, wallet reconciliation, CSV exports, organization summary, and organization reward dashboard live in `application/reporting`, `infra/postgres/reporting`, and `http/reporting`; legacy report URLs are preserved. |
| Wallet | Wallet audit/read/link/token-tax/deposit/retirement/observed-deposit flows have application, Postgres, HTTP/worker ownership, typed HTTP errors, and access-control-backed checks. |
| Operations | Health/readiness route composition lives in `http/operations`; the legacy health API wrapper is gone. |
| Content | Chapter/content/upload/media/processing handlers use content application use cases, Postgres adapters, typed HTTP errors, and one `http/content` route configurator. |
| Access control | Role/delegation routes, delegated-permission use cases, typed access scopes/actions, shared `can(...)` decisions, middleware wiring, and backend-derived session capabilities are in place. |
| Identity | Current session, user list/profile, platform role assignment, login, verification, registration, password reset, JWKS, and auth helper routes have identity HTTP/application/infra ownership. |
| Learning | Course catalog/detail/management/lifecycle/creation/update/progress/roles/enrollment/assessment/teaching routes use learning application contracts, Postgres adapters, HTTP DTOs, bootstrap wiring, and typed HTTP errors. |
| Organizations | Organization CRUD, courses, members, audit, dashboard, invitations, role assignment, removal, and teacher-application tracking use organization application contracts, Postgres adapters, HTTP DTOs, bootstrap wiring, and typed HTTP errors. |
| KYC | Status/submission/review/audit flows have domain rules, application contracts, Postgres adapter/use case, HTTP DTOs, bootstrap wiring, and typed HTTP errors. |
| Teacher applications | Self-read, audit, list, platform review, submission, decision, organization nomination, and notification fan-out have domain/application/Postgres/HTTP ownership; the legacy service was deleted. |

### Open Context Work

- Rewards still needs direct evidence for any remaining status/event newtypes,
  candidate transition rules, request/response DTO leaks, Diesel adapter leaks,
  and fake-port use-case tests.
- Access control still needs continued evidence that middleware is only an early
  rejection optimization and that use cases protect business actions.
- Integration-test harnesses still need global `include!`/`imports.rs` cleanup.
- Data boundary rules below remain active until current scans prove each item at
  repository scope, not only in recently touched files.

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
6. Deepen already-started contexts next: identity, learning, content,
   notifications, operations, access control, reporting, and wallet.
7. Deepen organizations, teacher applications, and KYC with the same Level 2
   matrix instead of leaving orchestration in legacy services.
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
- [x] API handlers mostly contain extraction, use-case call, and response
      mapping; no complex Diesel query builders.
- [ ] Permission behavior has one backend source of truth.
- [x] `main.rs` is mostly logging/env setup, bootstrap calls, and server start.
- [x] Route ownership is complete: `src/api` has been removed and the `/api`
      scope is assembled from `http` context route configurators.
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

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
        course_permission_checks.rs
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

## Completed Slice Ledger

This ledger compresses the detailed completed-slice audit trail so TODO/18 stays
usable as a working plan. The exact implementation history is preserved in git;
this file keeps only the architectural outcome, the current proof style, and the
remaining gaps.

| Slices | Completed architectural outcome |
| --- | --- |
| 1-7 | Created HTTP primitives, auth extractor, bootstrap app state, normal module assembly, first API migration, initial reward domain extraction, and a modular Actix entrypoint. |
| 8-19 | Moved learning assessment reads/submission and content/chapter upload/media/job flows toward application, infra, HTTP, and injected-use-case ownership. |
| 20-25 | Moved operations, role catalog, notification preferences/inbox, and current-session route ownership into context HTTP/bootstrap modules; removed the legacy session service. |
| 26-34 | Moved reward policy, fraud block, reward history, candidate audit, course/platform candidate lists, teacher decisions, amount decisions, and candidate submission request/response contracts into `http/rewards` DTOs and handlers. |
| 35-40 | Extracted reward compensation, payout planning, token confirmation, wallet credit, wallet-credit notification, and reconciliation into `application/rewards` use cases with Postgres adapters. |
| 41-48 | Extracted wallet audit, wallet reads, wallet linking, token tax, deposit intents, retirements, and observed deposit indexing into wallet application/HTTP/Postgres ownership. |
| 49-55 | Moved wallet and reward authorization decisions through `application/access_control`, including payout, platform, course, organization, fraud-block, and notification-recipient policies. |
| 56 | Moved production Actix app-data registration out of `main.rs`; `main.rs` is now process orchestration while bootstrap owns concrete state and route wiring. |
| 57-64 | Typed reward audit, execution job, candidate, fraud-block, policy, payout-method, token, wallet-credit, and compensation transaction vocabulary in the rewards domain while keeping compatibility aliases where legacy callers still need them. |
| 65-71 | Moved platform summary, platform fraud dashboard, organization summary, organization reward dashboard, platform reward dashboard, platform wallet reconciliation, and platform CSV export behavior into `application/reporting`, `infra/postgres/reporting`, and `http/reporting`; legacy report URLs still flow through the existing reports scope while matching old service queries/DTOs/CSV helpers were removed. |
| 72-84 | Finished the legacy HTTP route cleanup: request params/reporting, wallet, access-control roles/delegation, reward-policy/fraud-block wrappers, operations/session/content, platform users, KYC, teacher applications, authentication, course/organization route ownership, and `/api` composition all moved into `http/<context>` modules; the legacy `src/api` module was retired. |
| 85 | Split `http/organizations` away from `include!` and `imports.rs` into explicit modules for DTOs, CRUD handlers, course lists, dashboard, member list/invite/audit/role/removal flows, teacher-application tracking, and route composition. |
| 86 | Split `http/learning/course_routes` away from `include!` and `imports.rs` into explicit modules for DTOs, support/error mapping, catalog, teaching dashboard, management, lifecycle, organizations, roles, progress, enrollment, assessments, and routes. |
| 87 | Moved `/courses/{id}/organizations` behind `application/learning/list_course_organizations`, `infra/postgres/learning` adapters, an HTTP-owned response DTO, and bootstrap app-data wiring; route and permission tests now cover the injected use case. |
| 88 | Moved `DELETE /courses/{id}` behind `application/learning/delete_course`, a Postgres delete adapter/use case, and bootstrap app-data wiring; the HTTP management handler no longer owns Diesel deletion. |
| 89 | Moved `GET /courses/{id}` behind `application/learning/get_course`, a Postgres read adapter/use case, and an HTTP-owned `CourseResponse`; the catalog handler no longer returns the Diesel `Course` record directly. |
| 90 | Moved `GET /courses` behind `application/learning/discover_courses`, a Postgres discovery adapter/use case, and an HTTP-owned `CourseDiscoveryResponse`; literal wildcard search behavior is covered by course discovery tests. |
| 91 | Moved `PUT /courses/{id}/lifecycle` behind `application/learning/update_course_lifecycle`, a Postgres lifecycle adapter/use case, HTTP request DTO mapping, and bootstrap app-data wiring; lifecycle status normalization and permission selection now live outside the route. |
| 92 | Moved `PUT /courses/{id}` behind `application/learning/update_course`, a Postgres update adapter/use case, HTTP request DTO mapping, and bootstrap app-data wiring; course settings permission checks now use a shared learning Postgres permission adapter. |
| 93 | Moved `POST /courses` behind `application/learning/create_course`, a Postgres creation adapter/use case, HTTP request DTO mapping, and bootstrap app-data wiring; course creation permissions, owner organization linking, and pending organization invites now live outside the route. |
| 94 | Moved `GET/POST /courses/{id}/progress` behind `application/learning/learner_progress`, a Postgres progress adapter/use case, HTTP request/response DTOs, and bootstrap app-data wiring; visibility, enrollment, and content-membership checks now live outside the route. |
| 95 | Moved `POST /courses/{id}/users/{user_id}/roles` behind `application/learning/assign_course_role`, a Postgres role-assignment adapter/use case, HTTP request DTO mapping, and bootstrap app-data wiring; course role hierarchy checks now live outside the route and repository. |
| 96 | Moved course join request, join decision, waitlist approval, enrollment notification payload, and enrollment removal behind `application/learning/course_enrollment`, Postgres adapters, HTTP DTOs, and bootstrap wiring; the include-based `course_enrollment_service` was deleted. |
| 97 | Finished assessment route wiring for `GET /courses/{id}/assessments`, `GET /courses/{id}/assessments/{assessment_id}/attempts`, and `POST /courses/{id}/assessments/{assessment_id}/submit`; HTTP now receives injected assessment use cases instead of DB pools or Postgres stores. |
| 98 | Moved `GET /courses/catalog/{id}/learn` behind `application/learning/get_learner_course_learning`, learner-course Postgres read helpers, an HTTP-owned learning response DTO, and bootstrap app-data wiring; the route no longer opens the DB pool or calls `course_service::get_learner_course_learning`. |
| 99 | Moved `GET /courses/catalog/{id}` behind `application/learning/get_learner_course_detail`, shared learner catalog read vocabulary, Postgres detail/chapter adapters, an HTTP-owned detail response DTO, and bootstrap app-data wiring; app-state construction moved into `bootstrap/use_case_wiring.rs` so `startup.rs` stays focused on environment and startup tasks. |
| 100 | Moved `GET /courses/catalog` behind `application/learning/list_learner_course_catalog`, a Postgres catalog list adapter/use case, an HTTP-owned catalog list response DTO, and bootstrap app-data wiring; the learner catalog HTTP module no longer imports `DbPool` or `course_service`. |

## Recent Slice Evidence

Slice 100: move learner course catalog list into an injected use case.

- [x] Add `application/learning/list_learner_course_catalog` with a normalized
      query, output, store port, handler, use-case trait, and unit coverage for
      trim/default/clamp behavior.
- [x] Add `infra/postgres/learning/learner_course_catalog_list_*` so Diesel
      search, organization, lifecycle, visibility, reward, enrollment, paging,
      and literal wildcard escaping live in Postgres infra rather than HTTP or
      legacy services.
- [x] Add `LearnerCourseCatalogResponse` under HTTP DTO ownership and keep the
      existing JSON shape for `courses`, `total`, `limit`, `offset`, `search`,
      `organization_id`, `lifecycle_status`, `enrollment_status`, and
      `reward_available`.
- [x] Wire the concrete catalog list use case through `bootstrap/app_state`,
      `bootstrap/use_case_wiring`, and `bootstrap/app_data`.
- [x] Update catalog route tests to inject the production Postgres catalog list
      use case while preserving published summary, pending enrollment,
      reward-available filtering, own-draft visibility, and enrolled filtering.
- [x] Self-critique: learner catalog HTTP is now clean, but teacher dashboard
      reads still call legacy `course_service` from `http/learning`.
- [x] Prove behavior with binary compile, full `course_discovery` test target,
      API route reachability, query normalization unit test, formatting,
      line-count checks, `git diff --check`, and boundary scans proving the
      learner catalog HTTP/application DTO path no longer imports DB/Diesel,
      Postgres stores, or legacy services.

## Legacy Transition Rules

- [x] Legacy `api` wrappers are gone; new ring-based modules may still be
      called from existing `services` and `repositories` while deeper
      extraction is in progress.
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

- [x] Rewards now has Level 2 ring folders with aggregate/use-case granularity
      across candidate, policy, fraud block, payout, token, wallet credit,
      compensation, review, history, and reconciliation flows.
- [x] Core rewards vocabulary now lives in `domain/rewards` for policy,
      fraud-block, candidate, audit, execution, payout, token, wallet-credit,
      and compensation concepts; compatibility aliases remain only where legacy
      callers still need them.
- [ ] Move remaining reward statuses and event types into domain enums/newtypes.
      Keep database string conversion at the infra boundary.
- [x] Core rewards DTOs for policy, fraud block, history, audit, candidate
      lists/review, teacher decision, amount decision, and submission now live
      in `http/rewards/dto`.
- [x] Legacy `api/reward_policies` and `api/reward_fraud_blocks`
      compatibility wrappers have been deleted; callers import reward routes
      from `http/rewards`.
- [ ] Move remaining reward request/response structs out of service imports and
      into `http/rewards/dto`.
- [ ] Move candidate transition rules into pure domain functions:
      submit, teacher approve/reject, amount approve/reject, token confirmed,
      wallet credited, notified, reconciliation needed.
- [x] Reward command/read use cases now have module-local store ports and
      Postgres adapters for policy, fraud block, history, candidate audit,
      course/platform candidate lists, teacher/amount decisions, submission,
      compensation, payout planning, token confirmation, wallet credit,
      wallet-credit notification, and reconciliation.
- [ ] Define remaining context-owned repository ports before moving Diesel code.
      Reward and fraud dashboards belong to `application/reporting/*`; only
      reward command and reward read-model ports stay in
      `application/rewards`.
- [ ] Move remaining reward Diesel implementations behind
      `infra/postgres/rewards`.
- [x] Most reward permission checks now flow through
      `application/access_control`, including payout, platform review/policy,
      compensation, course/organization reward actions, fraud-block actions,
      and notification recipient groups.
- [ ] Move reward permission decisions through `application/access_control`
      instead of calling `user_permission_*_request` directly from reward use
      cases.
- [ ] Keep the old route paths stable while swapping internals.
- [ ] Add tests at three levels: pure domain transition tests, application
      use-case tests with fake ports, and API regression tests for existing
      routes.

## Reporting Context

- [x] Platform summary, fraud dashboard, reward dashboard, wallet
      reconciliation, platform CSV exports, organization summary, and
      organization reward dashboard now live in `application/reporting`,
      `infra/postgres/reporting`, and `http/reporting` with legacy report URLs
      preserved.
- [x] `http/reporting` owns the `/reports` Actix scope and exposes only a
      context-level route configurator to the rest of the app.

## Wallet Context

- [x] Wallet audit, read, link, token-tax, deposit-intent, retirement, and
      observed-deposit indexing now have Level 2 application/use-case,
      Postgres, and HTTP/worker ownership with legacy routes preserved.
- [x] Wallet audit reconciliation classification lives in `domain/wallet`, and
      wallet access checks flow through `application/access_control`.
- [x] `http/wallet` owns the `/wallets` Actix scope and exposes only a
      context-level route configurator to the rest of the app.

## Operations Context

- [x] `http/operations` owns health/readiness route composition; the legacy
      `api/health` compatibility wrapper has been deleted.

## Content Context

- [x] `http/content` exposes one context-level route configurator for chapter
      and content-item routes; the legacy `api/chapters` and `api/contents`
      wrappers have been deleted.

## Access Control Context

- [x] Wallet and reward authorization decisions now use
      `application/access_control` with typed permission vocabulary and
      Postgres permission adapters for the moved wallet/reward use cases.
- [x] `http/access_control` owns role route composition and exposes a
      context-level route configurator for the app.
- [x] `http/access_control` owns delegated-permission route composition; the
      legacy `api/delegated_permissions` module has been deleted.
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

## Identity Context

- [x] `http/identity` owns current-session and platform `/user` route
      composition; the legacy `api/users` module has been deleted.
- [x] `http/identity/authentication` owns `/auth`, JWKS, login, registration,
      email verification, password reset, and auth session helper routes; the
      legacy `api/authentication` module has been deleted.

## Learning Context

- [x] `http/learning` owns `/courses` route composition, including content,
      reward candidate, catalog, teaching dashboard, enrollment, progress, and
      assessment routes; the legacy `api/courses` module has been deleted.
- [x] `http/learning/course_routes` uses normal modules instead of legacy
      `include!`/`imports.rs` structure.
- [x] `/courses/{id}/organizations` now has application output/error/port
      contracts, a Postgres adapter/use case, HTTP DTO mapping, bootstrap
      wiring, and route/permission tests.
- [x] `DELETE /courses/{id}` now has application outcome/error/port contracts,
      a Postgres adapter/use case, bootstrap wiring, and route/application
      tests.
- [x] `GET /courses/{id}` now has application output/error/port contracts, a
      Postgres adapter/use case, HTTP DTO mapping, bootstrap wiring, and
      route/permission/application tests.
- [x] `GET /courses` now has application query/output/error/port contracts, a
      Postgres adapter/use case, HTTP DTO mapping, bootstrap wiring, and
      route/discovery/application tests.
- [x] `PUT /courses/{id}/lifecycle` now has application command/output/error
      and store-port contracts, a Postgres adapter/use case, HTTP request DTO
      mapping, bootstrap wiring, and application/integration/route tests.
- [x] `PUT /courses/{id}` now has application command/patch/output/error and
      store-port contracts, a Postgres adapter/use case, HTTP request DTO
      mapping, bootstrap wiring, and application/integration/route tests.
- [x] `POST /courses` now has application command/output/error and store-port
      contracts, a Postgres adapter/use case, HTTP request DTO mapping,
      bootstrap wiring, and application/integration/route tests.
- [x] `GET/POST /courses/{id}/progress` now has application command/output/error
      and store-port contracts, a Postgres adapter/use case, HTTP DTO mapping,
      bootstrap wiring, and application/integration/route tests.
- [x] `POST /courses/{id}/users/{user_id}/roles` now has application
      command/output/error and store-port contracts, a Postgres adapter/use
      case, HTTP DTO mapping, bootstrap wiring, and application/route tests.
- [x] Course enrollment request, decision/waitlist/approval, notification
      payload creation, and student enrollment removal now have application
      command/output/error and store-port contracts, Postgres adapters/use case,
      HTTP DTO mapping, bootstrap wiring, and application/integration/route
      tests; the old include-based enrollment service was deleted.
- [x] Assessment listing, attempt listing, and attempt submission routes now
      receive injected application use cases with Postgres adapters/use cases,
      bootstrap wiring, and route tests; the HTTP assessment handler no longer
      owns DB pool access or concrete Postgres store construction.
- [ ] Move remaining learning service/DB-heavy handlers into application use
      cases with Postgres adapters.

## Organizations Context

- [x] `http/organizations` owns `/organizations` route composition, including
      members, dashboard, course lists, reward submissions, and teacher
      application nomination/list routes; the legacy `api/organizations` module
      has been deleted.
- [x] `http/organizations` uses normal modules instead of legacy
      `include!`/`imports.rs` structure.
- [ ] Move organization CRUD/member/dashboard orchestration into application
      use cases with Postgres adapters.

## KYC Context

- [x] `http/kyc` owns KYC route composition; the legacy `api/kyc` module has
      been deleted.

## Teacher Applications Context

- [x] `http/teacher_applications` owns teacher-application route composition;
      the legacy `api/teacher_applications` include-based module has been
      deleted.

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
- [ ] API handlers mostly contain extraction, use-case call, and response
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

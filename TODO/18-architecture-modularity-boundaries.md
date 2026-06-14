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
| 101 | Moved `GET /courses/teaching` behind `application/learning/list_teacher_course_dashboard`, teacher dashboard application vocabulary, Postgres scope/permission/summary adapters, an HTTP-owned dashboard list response DTO, and bootstrap app-data wiring; the list route no longer opens the DB pool or calls `course_service::discover_teacher_course_dashboard`. |
| 102 | Moved `GET /courses/teaching/{id}` behind `application/learning/get_teacher_course_workspace`, Postgres teacher workspace adapters, shared learning content-processing display helpers, an HTTP-owned workspace response DTO, and bootstrap app-data wiring; the workspace route no longer opens the DB pool or calls `course_service::get_teacher_course_workspace`. |
| 103 | Moved `GET /courses/teaching/{id}/enrollments` behind `application/learning/get_teacher_course_enrollment_workspace`, shared teacher enrollment vocabulary, Postgres join-request/roster/reward-eligibility adapters, an HTTP-owned enrollment workspace response DTO, and bootstrap app-data wiring; the enrollments route no longer opens the DB pool or calls `course_service::get_teacher_course_enrollment_workspace`. |
| 104 | Moved `GET /courses/teaching/{id}/students` behind `application/learning/get_teacher_course_students`, Postgres roster/progress/reward-evidence adapters, an HTTP-owned students response DTO, and bootstrap app-data wiring; `http/learning/course_routes/teaching.rs` no longer imports `DbPool` or `course_service`. |
| 105 | Started the organizations application/infra context by moving `GET /organizations/{id}/courses` behind `application/organizations/list_organization_courses`, Postgres organization course adapters, HTTP-owned course-list DTOs, and bootstrap app-data wiring; the route no longer opens the DB pool or calls `course_service::discover_organization_courses`. |
| 106 | Moved `GET /organizations/{id}/members` behind `application/organizations/list_organization_members`, Postgres member/permission/delegation adapters, HTTP-owned member-list DTOs, and bootstrap app-data wiring; the route no longer opens the DB pool or calls `organization_service::list_organization_members`. |
| 107 | Moved `GET /organizations/{id}/members/{user_id}/audit` behind `application/organizations/list_organization_member_audit`, a Postgres member-audit adapter/use case, an HTTP-owned audit response DTO, and bootstrap app-data wiring; the route no longer opens the DB pool, imports Diesel/schema/model types, or relies on route middleware to run the audit query. |
| 108 | Moved `GET /organizations/{id}/dashboard` behind `application/organizations/get_organization_dashboard`, granular Postgres dashboard read adapters, HTTP-owned dashboard DTOs, and bootstrap app-data wiring; the route no longer opens the DB pool or calls `organization_service::get_organization_dashboard`, while dashboard gating, missing-permission sections, health, and alerts now live in application code. |
| 109 | Moved `DELETE /organizations/{id}/users/{user_id}` behind `application/organizations/remove_organization_member`, a Postgres member-removal adapter/use case, and bootstrap app-data wiring; the route no longer opens the DB pool, calls `organization_service::remove_organization_member`, or relies on route middleware for the manage-members permission check. |
| 110 | Moved `POST /organizations/{id}/members` behind `application/organizations/invite_organization_member`, a Postgres member-invite adapter/use case, and organization-context bootstrap wiring; the route no longer opens the DB pool, looks up `User` records, calls `organization_service::assign_role`, or relies on route middleware for the invite permission check. |
| 111 | Moved `POST /organizations/{id}/users/{user_id}/roles` behind `application/organizations/assign_organization_member_role`, a Postgres role-assignment adapter/use case, and organization-context bootstrap wiring; the route no longer opens the DB pool, calls `organization_service::assign_role`, or relies on route middleware for the assignment permission check. |
| 112 | Moved `GET /organizations/{id}/teacher-applications` behind `application/organizations/list_organization_teacher_applications`, granular Postgres teacher-application read adapters, HTTP-owned response DTOs, and organization-context bootstrap wiring; the route no longer opens the DB pool or calls `teacher_application_service::list_organization_applications`. |
| 113 | Moved organization CRUD routes behind `application/organizations/manage_organizations`, a Postgres management adapter/use case, HTTP-owned organization DTOs, and organization-context bootstrap wiring; `http/organizations/handlers.rs` no longer opens the DB pool, imports Diesel/model types, or calls `organization_service`. |
| 114 | Moved KYC status/submission/review/audit flows behind `domain/kyc`, granular `application/kyc` use cases, a Postgres KYC adapter/use case, HTTP-owned DTOs, and bootstrap wiring; the include-based `services/kyc_service` was deleted and `http/kyc` no longer opens DB pools or calls services. |
| 115 | Moved delegated-permission grant/list/revoke behind `domain/access_control/delegation`, `application/access_control/manage_delegated_permissions`, a Postgres delegated-permission adapter/use case, HTTP-owned DTOs, and access-control bootstrap wiring; the include-based `services/delegated_permission_service` was deleted and `http/access_control/delegated_permissions.rs` no longer opens DB pools or calls services. |
| 116 | Moved `GET /teacher-applications/me` behind `application/teacher_applications/get_my_application`, a Postgres teacher-application self adapter/use case, HTTP-owned self snapshot DTOs, and teacher-application bootstrap wiring; the route no longer opens the DB pool, returns Diesel records, or calls `teacher_application_service::get_my_application`. |
| 117 | Moved `GET /teacher-applications/{id}/audit` behind `application/teacher_applications/list_application_audit`, a Postgres audit adapter/use case, shared teacher-application audit output, HTTP-owned audit DTO mapping, and teacher-application bootstrap wiring; the route no longer opens the DB pool, returns Diesel audit records, or calls `teacher_application_service::list_audit_events`. |
| 118 | Moved `GET /teacher-applications` behind `domain/teacher_applications` status normalization, `application/teacher_applications/list_applications`, a Postgres list adapter/use case, HTTP-owned list query/response DTOs, and teacher-application bootstrap wiring; the route no longer opens the DB pool, returns Diesel application records, or calls `teacher_application_service::list_applications`. |
| 119 | Moved `GET /teacher-applications/review` behind `application/teacher_applications/list_platform_review`, a granular Postgres review store/use case/context mapper, HTTP-owned platform-review query/response DTO modules, and teacher-application bootstrap wiring; the route no longer opens the DB pool, returns service response structs, or calls `teacher_application_service::list_platform_applications`. |
| 120 | Moved `POST /teacher-applications` behind `domain/teacher_applications` scope validation, `application/teacher_applications/submit_application`, a Postgres submit adapter/use case, HTTP-owned submit request mapping, and teacher-application bootstrap wiring; the route no longer opens the DB pool for creation or calls `teacher_application_service::submit_application`. |
| 121 | Moved `PUT /teacher-applications/{id}/decision` behind `domain/teacher_applications` decision-status normalization, `application/teacher_applications/decide_application`, a Postgres decision adapter/use case with role-assignment helpers, HTTP-owned decision request mapping, and teacher-application bootstrap wiring; the route no longer opens the DB pool or calls `teacher_application_service::decide_application`. |
| 122 | Moved `POST /organizations/{id}/teacher-applications` behind `application/teacher_applications/nominate_application`, a Postgres nomination adapter/use case, HTTP-owned nomination request mapping, and teacher-application bootstrap wiring; the route no longer opens the DB pool or calls `teacher_application_service::nominate_application`, and the include-based `teacher_application_service` module was deleted. |
| 123 | Moved teacher-application notification fan-out behind `application/teacher_applications/notify_application_event`, a Postgres recipient lookup and notification sender adapter/use case, optional HTTP app-data wiring, and teacher-application bootstrap construction; `http/teacher_applications` no longer imports `DbPool`, repositories, or `NotificationsState` for notification recipient lookup. |
| 124 | Moved `GET /user` behind an injected `application/identity/list_users::UserListUseCase`, a Postgres user-list use-case wrapper, `bootstrap/identity_wiring`, and a DB-free `http/identity/user_list` handler; central app-data registration now delegates identity and teacher-application use-case bundles to context wiring helpers. |
| 125 | Moved `GET /user/{id}` behind `application/identity/get_user_profile` command/service/authorization behavior, a Postgres profile-read use-case wrapper, `UserProfileAccessStore` permission port, identity app-data wiring, and a DB-free HTTP profile handler; `http/identity/user_handlers.rs` no longer imports `DbPool`, repositories, infra, or Diesel. |
| 126 | Moved `POST /user/{id}/role` behind `application/identity/assign_platform_role`, a Postgres hierarchy-aware role-assignment store/use-case, best-effort platform role notification delivery inside infra, identity app-data wiring, and a DB-free HTTP assignment handler; `http/identity/platform_role_assignment.rs` no longer imports `DbPool`, repositories, Diesel, or `NotificationsState`. |
| 127 | Moved `POST /auth/login` behind `application/identity/login`, Postgres credential lookup, bcrypt password verification, JWT token issuance adapters, identity app-data wiring, and a DB-free HTTP login handler; login keeps existing response semantics for invalid credentials, unverified email, connection failure, and JWT creation failure. |
| 128 | Moved `GET /auth/verify-email` behind `application/identity/verify_email`, a Postgres email-verification token adapter/use case, identity app-data wiring, and a DB-free `http/identity/authentication/verify_email.rs` route; resend-verification remains isolated in the legacy email-verification HTTP file for a later slice. |
| 129 | Moved `POST /auth/resend-verification` behind `application/identity/resend_verification`, Postgres account lookup/token-rotation adapters, token generation and mock verification-email adapters, identity app-data wiring, and a DB-free HTTP resend handler; privacy-preserving response semantics remain unchanged. |
| 130 | Moved `POST /auth/register` behind `application/identity/register`, a reusable application password policy, Postgres registration transaction adapter/use case, bcrypt hashing/token generation/mock verification-email adapters, identity app-data wiring, and a DB-free HTTP registration handler with unchanged response semantics. |
| 131 | Moved `POST /auth/forgot-password` behind `application/identity/request_password_reset`, Postgres account lookup/reset-token adapters, token generation and mock password-reset email adapters, identity app-data wiring, and a DB-free `http/identity/authentication/forgot_password.rs` route; private known/unknown-account response semantics remain unchanged. |
| 132 | Moved `POST /auth/reset-password` behind `application/identity/reset_password`, a Postgres reset-token consumption/password-update transaction adapter/use case, bcrypt reset-password hashing adapter, identity app-data wiring, and a DB-free HTTP completion handler; missing/invalid/expired token and successful password-update responses remain unchanged. |
| 133 | Moved auth email normalization and privacy log hashing from `http/identity/authentication/support.rs` into `application/identity/email`, deleted the HTTP support module, and updated migrated auth routes to import identity email vocabulary without reaching into `utils::email`. |
| 134 | Moved identity mock email URL building, rendering, and printing from `utils::email` into `infra/email/identity`, wired registration/resend-verification/password-reset request delivery adapters to the email infra module, and updated the `mock_email` helper binary; `utils::email` now contains only token generation/hash helpers. |
| 135 | Moved identity token generation and hashing from `utils::email` into `infra/tokens/identity`, updated identity Postgres adapters and auth-flow token seeding to use the token infra module, removed `utils::email` from the utility module tree, and deleted the old utility file. |
| 136 | Moved email-verification token creation and verification behavior from `models::email_verification_token` into `infra/postgres/identity/email_verification_tokens`, updated registration/resend/verify adapters and auth-flow token seeding to call the Postgres identity token adapter, and left the model as a Diesel record/insert shape only. |
| 137 | Moved password-reset token creation and consumption behavior from `models::password_reset_token` into `infra/postgres/identity/password_reset_tokens`, updated request/reset adapters and auth-flow token seeding to call the Postgres identity token adapter, and left the model as a Diesel record/insert shape only. |
| 138 | Moved migrated identity account lookup and password-auth reads from `models::user` Active Record methods into `infra/postgres/identity/accounts`, and updated login, resend-verification, and password-reset request stores to call the context-owned Postgres account adapter. |
| 139 | Moved registration account creation, default student role lookup/assignment, and password-auth insertion from `models::*` Active Record methods into `infra/postgres/identity/accounts`; the registration store now delegates account setup to the identity Postgres account adapter while keeping verification-token creation in the same transaction. |
| 140 | Moved the migrated user-profile access check off `repositories::platform_repository::user_permission_platform_request` and into `infra/postgres/identity/platform_permissions`, preserving direct platform-role permission checks, active platform delegation checks, and delegation logging. |
| 141 | Moved hierarchy-aware platform role assignment from `repositories::platform_repository::assign_role_to_user_with_hierarchy` into `infra/postgres/identity/platform_role_assignments`; the migrated platform-role assignment store now calls identity-owned Diesel queries for assigner hierarchy, target hierarchy, target role lookup, and assignment insertion. |
| 142 | Moved current-session delegated organization/course label lookups from `repositories::session_repository` into `infra/postgres/identity/current_session_delegations`, and expanded current-session API coverage to prove organization and course delegated permissions keep their labels in both scoped and top-level session output. |
| 143 | Moved current-session active delegated-permission loading from `repositories::delegated_permission_repository` into `infra/postgres/identity/current_session_delegations`; `current_session_store` now asks identity infra for active delegations and no longer builds a legacy `DelegatedPermissionFilter`. |
| 144 | Moved current-session user, platform role/permission, organization scope, and course scope reads from `repositories::session_repository` into granular identity-owned Postgres query modules; the migrated current-session store no longer imports legacy repositories. |
| 145 | Moved KYC review platform permission checks from `repositories::platform_repository::user_permission_platform_request` into `infra/postgres/kyc/kyc_permissions`, preserving direct platform-role checks, active platform delegation checks, and delegation logging. |
| 146 | Moved migrated access-control reward, wallet, and delegated-permission grant checks from legacy `user_permission_*_request` repository helpers into `infra/postgres/access_control` permission-check/delegation helpers, preserving direct role checks, active scoped delegation checks, and delegated-permission logging. |
| 147 | Moved organization course-list, member-list, dashboard, teacher-application tracking, member-audit, invite, removal, and role-assignment permission gates off legacy `user_permission_*_request` repository helpers and into an organization-owned Postgres permission adapter that asks the reusable access-control Postgres helper. |
| 148 | Moved teacher-application list, platform-review, submission, decision, audit, and nomination permission gates off legacy platform/organization repository permission helpers and into `infra/postgres/teacher_applications/teacher_application_permissions`, backed by the reusable access-control Postgres permission adapter. |
| 149 | Moved hierarchy-aware organization role assignment from `repositories::organization_repository::assign_role_to_user_in_organization` into `infra/postgres/organizations/organization_role_assignments`; invite and explicit role-assignment stores now share organization-owned hierarchy, role lookup, and assignment queries. |
| 150 | Moved wallet-facing persistent key/value state reads and writes from `repositories::persistent_state_repository` and `models::PersistentState` into `infra/postgres/operations/persistent_state`; wallet deposit-intent, retirement, and token-tax stores now use the operations-owned adapter directly. |
| 151 | Moved teacher-application persistence and reviewer-recipient queries from `repositories::teacher_application_repository` into `infra/postgres/teacher_applications` records/recipients modules; submit, nomination, decision, and notification stores no longer import legacy repositories. |
| 152 | Moved reward payout planning off legacy reward-candidate and persistent-state repositories; `reward_payout_plan_store` now uses `infra/postgres/rewards/reward_candidate_records` and `infra/postgres/operations/persistent_state` for candidate loading and presigner detection. |
| 153 | Moved reward candidate creation/idempotency lookup, teacher decision updates, amount decision updates, audit-event insertion, and execution-job enqueueing off legacy repositories into rewards-owned Postgres record helpers; candidate submission, teacher decision, amount decision, and reconciliation audit stores now use context-owned persistence helpers. |
| 154 | Moved platform reward candidate listing/counting off the legacy reward-candidate repository; `platform_reward_candidate_store` now uses rewards-owned candidate filtering and list/count helpers. |
| 155 | Moved reward policy create/list/version/deactivation persistence off the legacy reward-policy repository; `reward_policy_store` and mappers now use rewards-owned policy record and activation helpers. |
| 156 | Moved reward fraud-block create/find/list/revoke persistence off the legacy fraud-block repository; `reward_fraud_block_store` and mappers now use rewards-owned fraud-block record helpers, with production list coverage in the integration test. |
| 157 | Moved reward compensation idempotency lookup, candidate lookup, and compensation-record creation off legacy repositories; `reward_compensation_transaction` now uses rewards-owned candidate and compensation record helpers. |
| 158 | Moved token-confirmation candidate lookup/status update, payout-record lookup/create, and audit-event insert calls off legacy repositories; `reward_token_confirmation_transaction` now uses rewards-owned candidate, payout, and audit helpers. |
| 159 | Moved core wallet-credit candidate lookup/status update, wallet-credit record lookup/create, and audit-event insert calls off legacy repositories; `reward_wallet_credit_transaction` now uses rewards-owned candidate, wallet-credit record, and audit helpers. |
| 160 | Moved wallet-credit notification candidate lookup/status update, wallet-credit record lookup/notification marking, and audit-event insert calls off legacy repositories; `reward_wallet_credit_notification_transaction` now uses rewards-owned candidate, wallet-credit record, and audit helpers. |
| 161 | Moved reconciliation candidate refreshes, payout-record lookup, and wallet-credit record lookup off legacy repositories; `src/infra/postgres/rewards` no longer imports `crate::repositories`. |
| 162 | Moved migrated learning and organization reward queue/status read models off reward-candidate model status aliases; teacher dashboard, teacher-student reward progress, and organization course metric queries now import reward status vocabulary from `domain/rewards/candidate/status`. |
| 163 | Moved production service entrypoint imports for reward candidate event/source/status vocabulary off reward-candidate model aliases; course reads, candidate submission/decision helpers, and execution test support now import reward vocabulary from `domain/rewards/candidate`. |
| 164 | Removed reward candidate event/source/status compatibility aliases from the Diesel model; tests and fixtures now import reward candidate vocabulary directly from `domain/rewards/candidate`, leaving `models::reward_candidate` as record/insert structs only. |
| 165 | Removed reward audit-event compatibility aliases from the Diesel model; legacy reward service/test fixtures now import audit event vocabulary from `domain/rewards/audit`, leaving `models::reward_audit_event` as record/insert structs only. |
| 166 | Moved course lifecycle vocabulary into `domain/learning/course/status`; migrated lifecycle/progress use cases, Postgres read adapters, legacy service hubs, and fixtures now import course statuses from domain, leaving `models::course` as record/change-set structs only. |
| 167 | Moved course enrollment join-request vocabulary into `domain/learning/enrollment/status`; migrated enrollment/progress use cases, Postgres read/write adapters, legacy service hubs, and fixtures now import join statuses from domain, leaving `models::course_join_request` as record/insert structs only. |
| 168 | Removed teacher-application status/scope vocabulary from the Diesel model; organization/dashboard/reporting adapters, legacy service hubs, and fixtures now import lifecycle/scope vocabulary from `domain/teacher_applications`, leaving `models::teacher_application` as persistence record/insert structs only. |
| 169 | Removed KYC status/audit-event vocabulary and unused Active Record query helpers from the Diesel models; KYC Postgres adapters and tests now import submission statuses from `domain/kyc/submission` and audit events from `domain/kyc/audit`, leaving KYC models as persistence shapes only. |
| 170 | Removed delegated-permission scope compatibility aliases from the Diesel model; course-service legacy helpers, access-control/learning/organization/reward Postgres adapters, and fixtures now import delegation scopes from `domain/access_control/delegation`, leaving `models::delegated_permission` as persistence shapes only. |
| 171 | Removed the wallet deposit pending-status compatibility alias from the Diesel model; legacy wallet deposit-intent creation and the migrated Postgres wallet deposit-intent adapter now import status vocabulary from `domain/wallet/deposit`, leaving `models::wallet_token_deposit_intent` as persistence shapes only. |
| 172 | Moved the notification inbox list limit out of the Diesel model and into the notification inbox application boundary; migrated inbox stores, legacy notification state, and tests now pass the caller-owned limit into the model helper, leaving `src/models` with no public constants. |
| 173 | Moved notification create/list/mark-read/delete Diesel operations out of `models::notification` and into `infra/postgres/notifications/notification_records`; migrated inbox stores and legacy notification utilities now use the notification-owned Postgres record helper, leaving `models::notification` as persistence shapes only. |
| 174 | Moved persistent-state get/set calls off the Diesel model; the legacy persistent-state repository now delegates to operations-owned Postgres helpers, leaving `models::persistent_state` as a persistence shape only. |
| 175 | Moved DB version-control get/update queries off the Diesel model and into `infra/postgres/operations/db_version_control`; startup DB setup and the regression test now use operations-owned Postgres records, leaving `models::db_version_control` as a persistence shape only. |
| 176 | Moved platform, organization, and course role-hierarchy reads off the Diesel models and into `infra/postgres/access_control/hierarchy_records`; legacy repository bridges and hierarchy tests now use access-control Postgres records, leaving the hierarchy models as persistence shapes only. |
| 177 | Moved platform role-permission assignment off the Diesel model and into `infra/postgres/access_control/permission_assignment_records`; the legacy platform-permission repository now delegates to access-control Postgres records, leaving `models::role_permission_platform` as a persistence shape only. |
| 178 | Moved platform user-role assignment and platform role-permission checks off the Diesel model and into `infra/postgres/access_control/platform_role_records`; legacy platform repositories, teacher-application role assignment, and fixtures now use access-control Postgres records, leaving `models::user_role_platform` as a persistence shape only. |
| 179 | Moved password authentication row creation off the Diesel model and into `infra/postgres/identity/authentication_records`; legacy user creation now delegates to identity Postgres records, leaving `models::authentication` as a persistence shape only. |
| 180 | Moved organization user-role assignment and organization permission checks off the Diesel model and into `infra/postgres/access_control/organization_role_records`; legacy organization repositories, teacher-application role assignment, and fixtures now use access-control Postgres records, leaving `models::user_role_organization` as a persistence shape only. |
| 181 | Moved course user-role assignment and course permission checks off the Diesel model and into `infra/postgres/access_control/course_role_records`; legacy course repositories, teacher-application role assignment, and fixtures now use access-control Postgres records, leaving `models::user_role_course` as a persistence shape only. |
| 182 | Moved platform, organization, and course role-name lookups off `models::role` and into `infra/postgres/access_control/role_catalog_store`; repositories, teacher-application assignment, and fixtures now resolve role IDs through access-control infra, leaving `models::role` as Diesel row shapes only. |
| 183 | Moved teacher-application approved-bundle exists-before-assign guards into `infra/postgres/access_control/*_role_records`; teacher-application role decisions now only resolve the teacher role and delegate idempotent assignment to access-control record adapters. |
| 184 | Moved teacher-application reviewer-recipient permission queries into `infra/postgres/access_control/permission_recipient_records`; teacher-application notification storage and the legacy teacher-application repository export now share the same access-control read adapter, removing duplicate recipient SQL. |
| 185 | Removed the thin `infra/postgres/teacher_applications/teacher_application_permissions` shim; teacher-application stores now call `infra/postgres/access_control/permission_checks` directly for platform and organization authorization. |
| 186 | Removed duplicate KYC platform permission/delegation SQL; `infra/postgres/kyc/kyc_store` now calls `infra/postgres/access_control/permission_checks` directly for KYC review authorization. |
| 187 | Removed duplicate identity user-profile platform permission/delegation SQL; `infra/postgres/identity/user_profile_store` now calls `infra/postgres/access_control/permission_checks` directly for `VIEW_USER` authorization. |
| 188 | Removed duplicate learning course/platform/organization permission-delegation SQL; learning Postgres stores and query helpers now call `infra/postgres/access_control/permission_checks` directly for course-context authorization. |
| 189 | Turned legacy platform/course/organization repository permission request functions into thin compatibility bridges over `infra/postgres/access_control/permission_checks`, removing their local role-plus-delegation composition. |
| 190 | Turned legacy delegated-permission active platform/organization/course checks into thin compatibility bridges over `infra/postgres/access_control/permission_delegations`, removing hard-coded scoped delegation SQL from the repository active-check helpers. |
| 191 | Replaced the include-based legacy delegated-permission repository shell with normal `records` and `revocation` child modules plus explicit public re-exports; the repository no longer has an `imports.rs` file. |
| 192 | Moved delegated-permission create/find/list/revoke SQL into `infra/postgres/access_control/delegated_permissions/records`; the Level 2 Postgres adapter and legacy repository now share the same access-control record adapter. |

## Recent Slice Evidence

Slice 192: move delegated-permission record SQL into access-control.

- [x] Add `infra/postgres/access_control/delegated_permissions/records.rs` as
      the shared Postgres record adapter for delegated-permission create, find,
      find-active, list, and revoke operations.
- [x] Retarget the Level 2 delegated-permission `read_queries` and
      `write_queries` modules to call the shared record adapter while keeping
      application output/error mapping at the application adapter boundary.
- [x] Retarget the legacy `repositories::delegated_permission_repository`
      `records` and `revocation` modules to call the same access-control record
      adapter while preserving their public `QueryResult<DelegatedPermission>`
      API for tests and unmigrated callers.
- [x] Keep active platform/organization/course delegation checks delegated to
      `infra/postgres/access_control/permission_delegations`; this slice moves
      record CRUD/query SQL, not authorization decision composition.
- [x] Self-critique: the legacy repository still exists as a compatibility
      wrapper because tests and fixtures import it directly. It is now thin
      enough to delete once those callers move to access-control test helpers
      or application use cases.
- [x] Prove behavior with `cargo fmt --all --check`,
      `./scripts/run-host-tests.sh cargo test --test repository_delegation_tests`,
      `./scripts/run-host-tests.sh cargo test --test delegated_permissions`,
      `./scripts/run-host-tests.sh cargo test --test current_session_api`,
      `./scripts/run-host-tests.sh cargo test --test organization_members`,
      `./scripts/run-host-tests.sh cargo test --test kyc_review`,
      `./scripts/run-host-tests.sh bash -lc 'cargo test --tests --no-run'`,
      `git diff --check`, line-count checks, and boundary scans proving no
      delegated-permission record insert/update/list/find SQL remains under
      `repositories/delegated_permission_repository`.

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
- [x] Delegated-permission grant/list/revoke now have domain validation rules,
      application command/query/output/error/store contracts, a Postgres
      adapter/use case, HTTP DTO mapping, bootstrap wiring, and
      delegated-permission/API tests; the legacy include-based delegated
      permission service has been deleted.
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
- [x] `GET /user` now uses an injected identity user-list application use case,
      a Postgres use-case wrapper, and a DB-free HTTP handler.
- [x] `GET /user/{id}` now uses an injected identity profile-read application
      use case, a Postgres use-case wrapper, and a DB-free HTTP handler.
- [x] `POST /user/{id}/role` now uses an injected identity platform
      role-assignment use case, a Postgres hierarchy-aware adapter, and
      DB-free HTTP error mapping.
- [x] `POST /auth/login` now uses an injected identity login use case with
      Postgres credential lookup, bcrypt verification, and JWT issuance behind
      infra adapters.
- [x] `GET /auth/verify-email` now uses an injected identity verify-email use
      case, a Postgres token adapter, and a DB-free HTTP route file.
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
- [x] `GET /courses/teaching` now has application query/output/error and
      store-port contracts, Postgres scope/permission/summary adapters, HTTP DTO
      mapping, bootstrap wiring, and teacher dashboard/API route tests.
- [x] `GET /courses/teaching/{id}` now has application query/output/error and
      store-port contracts, Postgres workspace adapters, HTTP DTO mapping,
      bootstrap wiring, and teacher dashboard/API route tests.
- [x] `GET /courses/teaching/{id}/enrollments` now has application
      query/output/error and store-port contracts, Postgres
      join-request/roster/reward-eligibility adapters, HTTP DTO mapping,
      bootstrap wiring, and teacher dashboard/API route tests.
- [x] `GET /courses/teaching/{id}/students` now has application query/output
      and store-port contracts, Postgres progress/reward-evidence adapters,
      HTTP DTO mapping, bootstrap wiring, and teacher dashboard/API route
      tests.
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
- [x] `GET /organizations/{id}/courses` now has application query/output/error
      and store-port contracts, Postgres adapters, HTTP DTO mapping, bootstrap
      wiring, and course discovery/API route tests.
- [x] `GET /organizations/{id}/members` now has application query/output/error
      and store-port contracts, Postgres member/permission/delegation adapters,
      HTTP DTO mapping, bootstrap wiring, and organization member/API route
      tests.
- [x] `GET /organizations/{id}/members/{user_id}/audit` now has application
      query/output/error and store-port contracts, a Postgres member-audit
      adapter/use case, HTTP DTO mapping, bootstrap wiring, and organization
      member/API route tests.
- [x] `GET /organizations/{id}/dashboard` now has application query/output/error
      and store-port contracts, Postgres summary/permission adapters, HTTP DTO
      mapping, bootstrap wiring, and organization dashboard/API route tests.
- [x] `DELETE /organizations/{id}/users/{user_id}` now has application
      command/error and store-port contracts, a Postgres member-removal adapter
      use case, bootstrap wiring, and organization member/API route tests.
- [x] `POST /organizations/{id}/members` now has application command/output/error
      and store-port contracts, a Postgres member-invite adapter use case,
      organization-context bootstrap wiring, and organization member/API route
      tests.
- [x] `POST /organizations/{id}/users/{user_id}/roles` now has application
      command/output/error and store-port contracts, a Postgres member-role
      assignment adapter use case, organization-context bootstrap wiring, and
      organization member/API route tests.
- [x] `GET /organizations/{id}/teacher-applications` now has application
      query/output/error and store-port contracts, Postgres teacher-application
      read adapters, HTTP DTO mapping, bootstrap wiring, and organization
      teacher-application/API route tests.
- [x] Organization CRUD routes now have application command/output/error and
      store-port contracts, a Postgres management adapter/use case, HTTP DTO
      mapping, bootstrap wiring, and organization management/API route tests.

## KYC Context

- [x] `http/kyc` owns KYC route composition; the legacy `api/kyc` module has
      been deleted.
- [x] KYC status, submission, review queue/decision, and audit reads now have
      domain validation/transition rules, granular application use-case
      contracts, a Postgres adapter/use case, HTTP DTO mapping, bootstrap
      wiring, and KYC/API route tests; the legacy include-based KYC service has
      been deleted.

## Teacher Applications Context

- [x] `http/teacher_applications` owns teacher-application route composition;
      the legacy `api/teacher_applications` include-based module has been
      deleted.
- [x] `GET /teacher-applications/me` now has application output/error/store
      contracts, a Postgres adapter/use case, HTTP DTO mapping, bootstrap
      wiring, and application/integration/API route tests.
- [x] `GET /teacher-applications/{id}/audit` now has application
      query/output/error/store contracts, a Postgres adapter/use case, HTTP DTO
      mapping, bootstrap wiring, and application/integration/API route tests.
- [x] `GET /teacher-applications` now has domain status normalization,
      application query/output/error/store contracts, a Postgres adapter/use
      case, HTTP query/response DTO mapping, bootstrap wiring, and
      application/integration/API route tests.
- [x] `GET /teacher-applications/review` now has application
      query/output/error/store contracts, a Postgres review adapter/use case,
      HTTP query/response DTO mapping, bootstrap wiring, and
      application/integration/API route tests.
- [x] `POST /teacher-applications` now has domain scope validation,
      application command/output/error/store contracts, a Postgres submit
      adapter/use case, HTTP request/response DTO mapping, bootstrap wiring,
      and application/integration/API route tests.
- [x] `PUT /teacher-applications/{id}/decision` now has domain decision-status
      normalization, application command/output/error/store contracts, a
      Postgres decision adapter/use case, HTTP request/response DTO mapping,
      bootstrap wiring, and application/integration/API route tests.
- [x] `POST /organizations/{id}/teacher-applications` now has application
      command/output/error/store contracts, a Postgres nomination adapter/use
      case, HTTP request/response DTO mapping, bootstrap wiring, and
      application/integration/API route tests.
- [x] The include-based `services/teacher_application_service` module has been
      deleted after all teacher-application routes moved behind Level 2
      application/Postgres/HTTP ownership.
- [x] Teacher-application notification fan-out now has application
      command/outcome/error/store contracts, a Postgres recipient/sender
      adapter use case, HTTP best-effort app-data wiring, and unit/integration
      coverage; `http/teacher_applications` no longer owns DB-backed recipient
      lookup.

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

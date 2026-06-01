# RustLearn Vision

## Mission

RustLearn exists to make learning feel immediately valuable. The platform combines structured online education with transparent achievement tracking and token-based rewards so students, teachers, organizations, and platform operators can collaborate around measurable progress.

## Product vision

Build a learning platform where:

- Students can discover courses, consume content, complete assessments, and receive notifications about progress.
- Teachers and organization staff can create courses, publish content, manage enrollments, and moderate discussions.
- Organizations can operate learning programs with their own memberships, roles, reporting, and course relationships.
- Platform administrators can manage global roles, permissions, wallets, billing-adjacent flows, exports, integrations, and compliance operations.
- Learning achievements can trigger blockchain-backed rewards through LearnToken and centralized wallet workflows.
- Video and other rich content uploads are processed reliably in the background and stored in S3-compatible object storage.

## Architecture direction

RustLearn should remain a modular service-oriented monolith until product boundaries are proven. The current shape is intentional:

- Actix Web provides the API surface and middleware stack.
- Diesel and Diesel Async provide typed PostgreSQL persistence.
- Repository and service modules isolate business logic from HTTP handlers.
- Middleware enforces JWT authentication plus platform, organization, and course-level permission checks.
- A dedicated worker binary handles long-running upload/video processing outside request paths.
- Ethereum utilities and Solidity contracts support LearnToken deployment, minting, presigned operations, and future reward events.
- Docker Compose and Kubernetes manifests keep local and deployment environments aligned.
- The Next.js app in `web/` can evolve as the primary learner/admin experience while the Rust API remains the source of truth.

## Strategic pillars

### 1. Trustworthy learning progress

Course content, chapters, assessments, enrollments, and notifications should produce an auditable learning record. Permission checks must protect every mutation and every sensitive read.

### 2. Reward mechanics that are safe by default

Token rewards should be transparent, testable, and abuse-resistant. Wallet linkage, minting, transfers, and presigned operations need strong audit trails and clear admin controls before being exposed broadly.

### 3. Organization-first administration

Organizations should be able to manage their members, teachers, courses, and reporting without platform operators handling routine tasks. Platform-level super admins should remain a safety net, not a bottleneck.

### 4. Reliable media operations

Uploads and video processing should be resilient: queued jobs, retries, health checks, structured errors, and clear operational runbooks. Learners should never wait on CPU-heavy media processing in a web request.

### 5. Developer confidence

The project should grow through migrations, tests, typed models, documented environment variables, and clear permission matrices. New contributors should be able to run the stack locally and understand where changes belong.

## Near-term outcomes

- Harden authentication and registration with password policy, email confirmation, and external public-key/JWKS support.
- Complete endpoint-level permission audits, especially read endpoints that expose user, organization, course, or financial data.
- Add API integration coverage for authentication and high-risk authorization flows.
- Define the token reward lifecycle from learning event to wallet credit, including event listeners and reconciliation.
- Improve setup automation for RSA keys, local secrets, database readiness, and first-run administrator creation.
- Keep README, TODO, PERMISSIONS, and environment examples synchronized with implementation.

## Non-goals for now

- Splitting into microservices before operational boundaries are clear.
- Mainnet token operations without security review, test coverage, and operational controls.
- Storing secrets or private keys in the repository.
- Treating generated artifacts or dependency directories as hand-authored source.

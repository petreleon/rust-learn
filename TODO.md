# Project Tasks

## 1. Project Setup & DevOps
- [x] **Environment Configuration**
  - [x] Verify `.env` matches `.env.example` keys
  - [x] Document specific values needed for `PRIVATE_KEY` and `PUBLIC_KEY` (in README.md)
- [ ] **Automated Setup**
  - [ ] Create a shell script to generate RSA keys automatically (replacing manual openssl steps)
  - [x] Add check for PostgreSQL availability in startup scripts (`scripts/app-entrypoint.sh` retries diesel migration up to 30 times)
- [x] **Docker & Deployment**
  - [ ] Verify `docker-compose` builds on target platforms (Linux/ARM64)
  - [x] Optimize Worker service build time: binary prebuilt into Docker image (`docker/worker.Dockerfile`)
  - [x] **Migrate S3 Storage:** Replace `minio` image with `rustfs` in `docker-compose.yml` to avoid AGPLv3 licensing issues.
  - [x] Replace `minio` Rust crate with `aws-sdk-s3`. Rename `MinioState` → `S3State` and `MANAGE_MINIO_OBJECTS` → `MANAGE_S3_OBJECTS`.

## 2. Authentication & User Management
- [ ] **Registration Flow**
  - [x] Add initial role assignment functionality `src/api/authentication.rs`
  - [ ] Implement email confirmation trigger upon registration
  - [ ] Validate password strength constraints
- [ ] **Security**
  - [ ] Review JWT token expiration settings (currently `1 day`, may be too short for long sessions)
  - [ ] Expose public keys for external verification (JWKS endpoint or `.well-known/jwks.json`)

## 3. Blockchain & Tokenomics
- [x] **Smart Contracts**
  - [ ] Review Solidity contracts in `ethereum/contracts` for security/audit
  - [x] `ethers-solc` compiles latest contract versions successfully (verified via `blockchain_integration_tests.rs`)
- [ ] **Integration**
  - [ ] Implement event listeners for token rewards (mint events → user wallet credit)
  - [ ] Test wallet linkage for users end-to-end

## 4. Video Worker Service
- [x] **Processing**
  - [x] Handle `ffmpeg` failures gracefully (retry with exponential backoff, mark failed after max attempts)
  - [x] Add retries for failed video uploads (configurable via `WORKER_MAX_ATTEMPTS` and `WORKER_BASE_BACKOFF_SECONDS`)
- [x] **Monitoring**
  - [x] Ensure `/tmp/worker_alive` is updated reliably (written every loop iteration; healthcheck in compose)
  - [x] Add logging for specific encoding errors (errors saved to DB; logged via `eprintln!`)

## 5. Testing & Quality Assurance
- [x] **Integration Tests**
  - [x] Expand `tests/blockchain_integration_tests.rs` (deploy+mint+presigner+EIP-2612 permit, 243 lines)
  - [ ] Add API integration tests for Login/Register (`POST /api/auth/login`, `POST /api/auth/register`)
  - [x] Add middleware access control tests (247 lines in `tests/middleware_access_control.rs`)
- [ ] **Unit Tests**
  - [ ] Add unit tests for `api/authentication` logic
  - [ ] Add unit tests for worker job definitions

## 6. Permissions & Role-Based Access Control (RBAC)
- [x] **Core Logic Implementation**
  - [x] Verify `RolePlatformHierarchy` assignment logic
  - [x] Verify `RoleOrganizationHierarchy` assignment logic
  - [x] Verify `RoleCourseHierarchy` assignment logic
- [x] **Authentication Integration**
  - [x] Implement default role assignment on user registration (`src/api/authentication.rs`)
  - [x] Create API endpoint to list available roles (`src/api/roles.rs` — platform, org, course)
  - [x] Create API endpoint to assign roles to users (admin only, with hierarchy checks)
- [ ] **Permission Checks**
  - [ ] Audit all API endpoints for missing permission checks (many GET endpoints are unprotected)
  - [x] Implement middleware for permission-based route protection (infrastructure exists; applied on PUT/DELETE courses, PUT/DELETE orgs, assign-role routes)
- [x] **Testing**
  - [x] Add unit tests for `UserRolePlatform::has_permission` (`tests/platform_permissions.rs`, `tests/platform_permissions_unit.rs`)
  - [x] Add unit tests for `UserRoleOrganization::has_permission` (`tests/organization_permissions.rs`)
  - [x] Add unit tests for `UserRoleCourse::has_permission` (`tests/course_permissions.rs`)

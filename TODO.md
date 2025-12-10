# Project Tasks

## 1. Project Setup & DevOps
- [ ] **Environment Configuration**
  - [ ] Verify `.env` matches `.env.example` keys
  - [ ] Document specific values needed for `PRIVATE_KEY` and `PUBLIC_KEY`
- [ ] **Automated Setup**
  - [ ] Create a shell script to generate RSA keys automatically (replacing manual openssl steps)
  - [ ] Add check for PostgreSQL availability in startup scripts
- [ ] **Docker & Deployment**
  - [ ] Verify `docker-compose` builds on target platforms (Linux/ARM64)
  - [ ] Optimize Worker service build time (cache dependencies)

## 2. Authentication & User Management
- [ ] **Registration Flow**
  - [ ] Add initial role assignment functionality `src/api/authentication.rs`
  - [ ] Implement email confirmation trigger upon registration `src/api/authentication.rs`
  - [ ] Validate password strength constraints
- [ ] **Security**
  - [ ] Review JWT token expiration settings
  - [ ] Ensure public keys are correctly exposed for verification

## 3. Blockchain & Tokenomics
- [ ] **Smart Contracts**
  - [ ] Review Solidity contracts in `ethereum/contracts`
  - [ ] Ensure `ethers-solc` is correctly compiling latest contract versions
- [ ] **Integration**
  - [ ] Verify event listeners for token rewards
  - [ ] Test wallet linkage for users

## 4. Video Worker Service
- [ ] **Processing**
  - [ ] Handle `ffmpeg` failures gracefully
  - [ ] Add retries for failed video uploads
- [ ] **Monitoring**
  - [ ] Ensure `/tmp/worker_alive` is updated reliably
  - [ ] Add logging for specific encoding errors

## 5. Testing & Quality Assurance
- [ ] **Integration Tests**
  - [ ] Expand `tests/blockchain_integration_tests.rs`
  - [ ] Add API integration tests for Login/Register
- [ ] **Unit Tests**
  - [ ] Add unit tests for `api/authentication` logic
  - [ ] Add unit tests for worker job definitions

## 6. Permissions & Role-Based Access Control (RBAC)
- [ ] **Core Logic Implementation**
  - [ ] Verify `RolePlatformHierarchy` assignment logic
  - [ ] Verify `RoleOrganizationHierarchy` assignment logic
  - [ ] Verify `RoleCourseHierarchy` assignment logic
- [ ] **Authentication Integration**
  - [x] Implement default role assignment on user registration (`src/api/authentication.rs`)
  - [ ] Create API endpoint to list available roles
  - [ ] Create API endpoint to assign roles to users (admin only)
- [ ] **Permission Checks**
  - [ ] Audit all API endpoints for missing permission checks
  - [ ] Implement middleware for role-based route protection
- [ ] **Testing**
  - [ ] Add unit tests for `UserRolePlatform::has_permission`
  - [ ] Add unit tests for `UserRoleOrganization::has_permission`
  - [ ] Add unit tests for `UserRoleCourse::has_permission`

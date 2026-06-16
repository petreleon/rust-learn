# TODO 21: Allowance-Based Token Burns, Fees, and Burner Leaderboards

Created: 2026-06-16.
Status: in progress.

Implementation status checked before first push:

- [x] Backend migration adds burn requests, burn fee records, and immutable
  leaderboard events.
- [x] Diesel schema is regenerated through `make migrate-redo && make schema`.
- [x] Backend domain/application/http/infra slice supports user and
  organization burn requests, fee-path validation, burn attribution,
  idempotency keys, and rolling leaderboard queries.
- [x] Organization/platform burn permissions are seeded for the initial roles.
- [x] Focused host and Docker Compose burn-token tests pass.
- [ ] Smart contract burn semantics still need to move from owner-arbitrary
  burn to `burn` / `burnFrom` allowance semantics.
- [ ] Web surfaces from the Web Boundary Plan are still pending.
- [ ] Reconciliation/admin failure handling still needs the next backend slice.

Objective: replace owner-arbitrary token burning with allowance-based burns and
add rolling leaderboards for users and organizations that burn LearnToken.

This rule must work for both platform-held wallets and decentralized wallets
such as MetaMask.

## Product Rule

Token burning must be user/organization authorized:

- The platform owner must not be able to burn arbitrary user balances.
- A user can burn from their own decentralized wallet.
- A user can authorize the platform to burn from an allowance.
- An organization owner/operator can burn tokens for an organization only when
  they have organization burn authority.
- Every burn is recorded with actor, beneficiary identity, wallet source,
  fee path, amount, transaction evidence, and leaderboard visibility.

Burners can be:

- users
- organizations

Organization burns must attribute:

- organization id
- actor user id
- actor role or permission evidence
- wallet source
- burn amount

## Wallet Modes

Centralized platform wallet:

- Tokens are already represented inside RustLearn's internal wallet ledger.
- Burn request debits the internal wallet only after a valid burn execution plan
  exists.
- The platform can execute the on-chain burn only from tokens it controls or
  from an explicit allowance.
- Internal ledger, burn record, external transaction, and leaderboard aggregate
  must reconcile by idempotency key.

Decentralized wallet, direct MetaMask payment:

- User connects MetaMask.
- User approves or directly executes the burn from their wallet.
- User pays network gas directly.
- No platform deposit fee is charged because tokens do not move into platform
  custody first.
- Backend records the confirmed transaction and indexes it into burn history.

Decentralized wallet, platform-mediated payment:

- User chooses to use the platform-mediated path.
- User transfers/deposits tokens into the platform-controlled flow first.
- User pays the deposit/transfer fee because tokens move to the platform before
  the platform executes or settles the burn.
- Backend links deposit intent, platform burn execution, fee record, and final
  burn record.

## Smart Contract Direction

Replace owner-arbitrary burn:

```solidity
function burn(address from, uint256 amount) public onlyOwner
```

with safer patterns:

```solidity
function burn(uint256 amount) public
function burnFrom(address from, uint256 amount) public
```

Rules:

- `burn(amount)` burns only `msg.sender` balance.
- `burnFrom(from, amount)` spends allowance first, then burns.
- The platform can burn user tokens only when the user/organization has granted
  allowance.
- Owner burn should be limited to treasury/platform-owned addresses or removed.

## Backend Boundary Plan

Domain:

- Add `domain/rewards/burn` or `domain/wallet/burn`.
- Model burn source:
  `centralized_wallet`, `decentralized_direct`, `decentralized_platform_mediated`.
- Model burner type:
  `user`, `organization`.
- Model fee path:
  `network_fee_paid_by_user`, `platform_deposit_fee`, `platform_subsidized`,
  `none`.
- Model burn status:
  `requested`, `approval_pending`, `deposit_pending`, `burn_pending`,
  `confirmed`, `ledger_recorded`, `leaderboard_indexed`, `failed`,
  `needs_reconciliation`.

Application:

- Add burn use cases:
  `RequestTokenBurn`, `ConfirmTokenBurn`, `ListBurnHistory`,
  `LoadBurnLeaderboard`, `ReconcileTokenBurn`.
- Validate:
  positive amount, supported wallet mode, actor authority, organization
  permission, idempotency key uniqueness, and no double indexing.
- Organization burns require explicit permission such as
  `BURN_ORGANIZATION_TOKENS` or inclusion in an existing owner-level wallet
  administration permission.

Infra/Postgres:

- Add tables:
  `token_burn_requests`
  `token_burn_fee_records`
  `token_burn_leaderboard_events`
- Link to:
  users, organizations, wallets, external transactions, internal transactions,
  deposit intents, and actor audit events.
- Store rolling leaderboard events as immutable facts; derive windows by
  timestamp rather than mutating totals.

HTTP:

- User routes:
  `POST /api/wallet/burns`
  `GET /api/wallet/burns`
  `GET /api/wallet/burns/leaderboard?window=7d|30d|365d`
- Organization routes:
  `POST /api/organizations/{id}/wallet/burns`
  `GET /api/organizations/{id}/wallet/burns`
  `GET /api/organizations/{id}/wallet/burns/permissions`
- Platform/admin routes:
  reconciliation and failed burn inspection only.

## Web Boundary Plan

User wallet:

- Show burn action from centralized balance.
- Show MetaMask direct burn option.
- Show platform-mediated option with deposit fee warning.
- Explain fee path before submission.
- Show pending, confirmed, failed, and reconciliation states.

Organization wallet:

- Show burn controls only for owners/operators with burn permission.
- Attribute the burn to the organization while preserving actor identity.
- Require explicit confirmation for organization burns.

Leaderboards:

- Weekly: last 7 days.
- Monthly: last 30 days.
- Annual: last 365 days.
- Separate filters for:
  users, organizations, all burners.
- Ranking metric:
  total burned LearnToken in the selected rolling window.
- Tie breaker:
  earliest latest burn timestamp, then stable burner id.

## Permissions

Add or map permissions:

- `BURN_OWN_TOKENS`
- `BURN_ORGANIZATION_TOKENS`
- `VIEW_BURN_LEADERBOARD`
- `RECONCILE_TOKEN_BURNS`

Organization owners should receive `BURN_ORGANIZATION_TOKENS` by default only
if the organization wallet is intended to be owner-managed. Otherwise, make it
an assignable organization permission.

## Verification Checklist

- Contract tests prove owner cannot burn arbitrary user balances.
- Contract tests prove `burn` and `burnFrom` work with allowance semantics.
- Application tests for centralized, MetaMask direct, and platform-mediated
  burn requests.
- Permission tests for organization owners/operators/non-members.
- Reconciliation tests prevent duplicate burn records and duplicate leaderboard
  events.
- Leaderboard tests for rolling 7/30/365 day windows.
- HTTP tests for fee path validation and permission mapping.
- Web tests for fee explanation, MetaMask direct path, platform-mediated path,
  and organization burn controls.

## Self Critique

Bad shape:

- Keep owner burn and simply promise not to abuse it. This fails trust.

Better shape:

- Allow user self-burn, but ignore platform-held wallets. This fails the current
  product because RustLearn also has centralized wallet ledger flows.

Better shape:

- Add MetaMask burn only. This excludes organizations and misses permissioned
  organization wallet operations.

Final shape:

- Use allowance-based burn semantics for decentralized wallets.
- Treat platform-held wallet burns as ledger plus execution workflows, not magic
  owner power.
- Make fee path explicit: direct MetaMask pays gas directly; platform-mediated
  burns include the deposit/transfer fee because tokens enter platform custody.
- Index immutable burn facts into rolling weekly, monthly, and annual
  leaderboards for both users and organizations.

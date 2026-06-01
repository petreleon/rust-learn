# Learning Reward Lifecycle

This document defines the target lifecycle for LearnToken rewards. It is the
contract future reward services, blockchain listeners, wallet flows,
notifications, and reporting should follow.

## Goals

- Reward only durable learning achievements, not raw page views or transient UI
  events.
- Keep reward decisions auditable, idempotent, and reversible through explicit
  correction records.
- Separate eligibility and calculation from token execution so failed blockchain
  operations can be retried safely.
- Keep the centralized wallet ledger, blockchain transaction record,
  notification, and audit trail in sync.

## Lifecycle

1. Course event

   A course-domain action creates a reward candidate event. Valid first events
   are assessment completion, course completion, manually approved completion,
   and administrative reward adjustment. Each event must have a stable
   idempotency key such as `course_completion:{course_id}:{user_id}:{attempt_id}`.

2. Eligibility check

   The reward service verifies that the user is enrolled, email-verified, not
   blocked by organization or course policy, and has not already received the
   same reward. Course-specific checks should validate passing score,
   completion percentage, assessment attempt status, and any cooldown or
   anti-abuse limits.

3. Reward calculation

   The service calculates the reward from a versioned policy. Inputs should
   include course id, organization id when present, user id, event type,
   assessment score or completion signal, policy version, and configured token
   amount or multiplier. The calculated amount is stored before token execution.

4. Audit record

   The system writes an immutable audit row for the decision with event payload,
   eligibility result, policy version, amount, actor, and idempotency key. A
   rejected candidate is still audited with a rejection reason.

5. Token mint or transfer

   If the reward is approved, the blockchain operation is enqueued as a durable
   reward execution job. Production policy should prefer transfer from a
   treasury/presigner balance when possible and mint only when platform policy
   explicitly allows supply expansion. The job records chain id, contract
   address, recipient wallet address, amount, nonce or idempotency key, and
   current status.

6. Wallet credit

   After the token transaction is confirmed, the centralized wallet ledger is
   credited using the same reward idempotency key. The wallet credit records a
   generic `reward_credit` transaction plus the internal wallet transaction. If
   wallet credit fails after blockchain confirmation, reconciliation must retry
   the wallet credit rather than sending another token transaction.

7. External transaction link

   The blockchain transaction hash, recipient address, contract address, amount,
   chain id, and confirmation metadata are stored as an external transaction and
   linked to the reward audit record and wallet transaction. This link is the
   source of truth for chain reconciliation.

8. Notification

   A successful reward creates a user notification that includes course context,
   reward amount, and wallet destination. Failed or delayed rewards create
   operator-visible events first; user-facing failure notifications should be
   sent only when the platform cannot complete the reward without user action.

9. Reconciliation

   A scheduled reconciliation process compares approved reward audit records,
   blockchain transaction status, wallet credits, and notifications. Missing or
   inconsistent steps are repaired from the last confirmed state. Reconciliation
   must never create a duplicate token transfer or duplicate wallet credit for
   the same idempotency key.

## States

Reward records should progress through these states:

- `candidate`: event captured, not yet checked.
- `rejected`: eligibility failed; no token or wallet mutation.
- `approved`: reward amount calculated and audit record written.
- `token_pending`: blockchain job is queued or in progress.
- `token_confirmed`: blockchain transaction is confirmed.
- `wallet_credited`: centralized wallet ledger is credited.
- `notified`: user notification was created.
- `completed`: all required side effects are present.
- `needs_reconciliation`: a recoverable mismatch or retryable failure occurred.
- `failed`: terminal failure requiring manual operator action.

## Required Data

Future implementation should add a reward table or equivalent event store with:

- reward id and idempotency key;
- user id, course id, optional organization id, optional assessment attempt id;
- event type and original event payload;
- eligibility result and rejection reason;
- policy version and calculated amount;
- status and retry metadata;
- blockchain chain id, contract address, transaction hash, and recipient address;
- centralized wallet id and transaction id;
- notification id;
- created, updated, approved, confirmed, credited, and completed timestamps.

## Operational Rules

- The idempotency key is unique and enforced at the database layer.
- Eligibility and calculation run in a database transaction.
- Token execution is asynchronous and retryable.
- Wallet credit happens only after token confirmation unless a deployment
  explicitly operates in off-chain-only mode.
- Every state transition emits structured logs with reward id, status, user id,
  course id, and idempotency key.
- Manual corrections create compensating records; existing audit records are not
  overwritten.

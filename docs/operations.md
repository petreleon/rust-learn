# Operations Runbook

This runbook defines the operational state that must survive restarts,
redeployments, and disaster recovery for RustLearn.

## State Inventory

PostgreSQL is the primary application database. It stores users, roles,
courses, content metadata, upload job state, notification events, token
reconciliation records, transactions, and `persistent_states`.

S3-compatible storage stores uploaded course materials and worker-generated
media outputs. The API currently creates upload jobs for the `course-materials`
bucket, and the worker writes processed video and extracted audio objects back
to the same bucket.

Ethereum state is split across the chain, application database, and secrets.
Public or managed chains are the source of truth for token balances and emitted
events. Local Anvil state is only durable if the Anvil state file and mnemonic
are preserved. PostgreSQL stores deployed contract addresses in
`persistent_states`.

The deployment signer secret, JWT keys, database credentials, and S3 credentials
must be backed up through the environment's secret manager. Do not put secrets
in repository backups.

## Backup Expectations

Back up PostgreSQL at least daily for development/staging and before every
release that changes migrations, permissions, token workflows, or contract
state. Production should use managed continuous backup or WAL archiving when
the acceptable recovery point objective is lower than the dump cadence.

Back up S3 objects with versioning or object replication where available. If
using RustFS locally, export each active bucket before destructive maintenance
and keep the exported object tree with the matching database dump.

Back up blockchain-related persistent state before deploying contracts or
restoring data:

- `persistent_states.learn_token_address`
- `persistent_states.learn_token_presigner_address`
- `persistent_states.platform_importer_address`
- Ethereum deployment signer secret and chain id
- Local Anvil `state.json` when using the bundled Anvil service
- Transaction hashes for deployment and token reconciliation events

## Local Backup Commands

Create a PostgreSQL dump from the Compose database:

```bash
mkdir -p backups
docker compose exec db sh -lc 'pg_dump -U "$POSTGRES_USER" -d "$POSTGRES_DB" --format=custom --file=/tmp/rust_learn.dump'
docker cp rust-learn-db-1:/tmp/rust_learn.dump backups/rust_learn-$(date +%Y%m%d%H%M%S).dump
```

Export RustFS/S3 objects with the AWS CLI from the host when RustFS is exposed
on `localhost:9000`:

```bash
mkdir -p backups/s3
AWS_ACCESS_KEY_ID="$S3_ACCESS_KEY" AWS_SECRET_ACCESS_KEY="$S3_SECRET_KEY" \
  aws --endpoint-url "http://localhost:9000" s3 sync s3://course-materials backups/s3/course-materials
```

For Kubernetes or managed S3, use the provider's native snapshot, replication,
or lifecycle tooling and record the bucket names included in the backup.

Back up local Anvil state when using the bundled Anvil service:

```bash
docker compose cp anvil:/anvil/state.json backups/anvil-state-$(date +%Y%m%d%H%M%S).json
```

## Restore Expectations

Restore in this order:

1. Restore secrets first so database, S3, JWT, and Ethereum clients can start.
2. Restore PostgreSQL and run pending migrations only after the dump is loaded.
3. Restore S3 objects for every bucket referenced by content metadata and
   upload jobs.
4. Restore local Anvil state, or configure the app to point at the original
   managed chain.
5. Verify `persistent_states` contract addresses match the selected chain.
6. Start the API and worker, then check `/ready` and worker logs.

Restore a Compose PostgreSQL dump into an empty or intentionally reset database:

```bash
docker cp backups/rust_learn.dump rust-learn-db-1:/tmp/rust_learn.dump
docker compose exec db sh -lc 'pg_restore -U "$POSTGRES_USER" -d "$POSTGRES_DB" --clean --if-exists /tmp/rust_learn.dump'
```

Restore S3 objects:

```bash
AWS_ACCESS_KEY_ID="$S3_ACCESS_KEY" AWS_SECRET_ACCESS_KEY="$S3_SECRET_KEY" \
  aws --endpoint-url "http://localhost:9000" s3 sync backups/s3/course-materials s3://course-materials
```

After restore, query the contract state persisted by the application:

```sql
select key, value
from persistent_states
where key in (
  'learn_token_address',
  'learn_token_presigner_address',
  'platform_importer_address'
);
```

If those addresses are missing, startup may deploy new contracts. If they point
to contracts from a different chain or discarded Anvil state, token operations
will not reconcile with restored data.

## Ethereum Artifact Release Process

Contract sources live in `ethereum/contracts/`. Runtime ABI/bin artifacts live
in `ethereum/artifacts/` and are used as a fallback by
`src/utils/eth/compiler.rs` when direct `ethers-solc` compilation fails.

When a Solidity contract changes:

1. Rebuild artifacts with the same Solidity version used by the app image.
2. Review the generated ABI/bin diff and commit only intentional artifact
   changes.
3. Run blockchain integration tests against Anvil.
4. Record the deployment signer, chain id, contract addresses, and transaction
   hashes for the release.
5. Back up PostgreSQL and chain state before starting a production deployment.

Generate ABI/bin artifacts through Compose:

```bash
docker compose run --rm --no-deps --entrypoint bash app -lc 'cd /usr/src/app; solc --abi --bin --overwrite -o ethereum/artifacts @openzeppelin=ethereum/contracts/lib/openzeppelin-contracts ethereum/contracts/LearnToken.sol ethereum/contracts/LearnTokenPresigner.sol ethereum/contracts/PlatformImporter.sol'
```

Run contract integration tests through Compose with Anvil available:

```bash
docker compose up -d anvil
docker compose --profile test run --rm --no-deps test-runner cargo test --test blockchain_integration_tests -- --ignored --nocapture
docker compose stop anvil
```

## Deployment Address Rules

The startup path is idempotent only when the correct address keys already exist
in `persistent_states`. Preserve these keys during database migrations and
restores. Do not clear them unless the release intentionally redeploys a
contract.

Current persisted keys:

- `learn_token_address`
- `learn_token_presigner_address`
- `platform_importer_address`

Current deployment environment inputs:

- `ETH_RPC_URL`, or `ETH_HOST` and `ETH_PORT`
- `ETH_CHAIN_ID`
- `ETH_MNEMONIC`
- `PLATFORM_TREASURY` for `PlatformImporter`, when used
- `OZ_PATH` when OpenZeppelin contracts are outside the default checkout path

The existing contracts are not upgradeable proxies. A breaking ABI or storage
change requires a new deployment address, application configuration review, and
a reconciliation plan for historical token events.

"use client";

import { ListChecks } from "lucide-react";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { EmptyState } from "@/features/admin/shared/route-kit/EmptyState";
import { formatDate } from "@/features/admin/shared/route-kit/formatDate";
import { PanelError } from "@/features/admin/shared/route-kit/PanelError";
import { PanelLoading } from "@/features/admin/shared/route-kit/PanelLoading";
import { StatusPill } from "@/features/admin/shared/route-kit/StatusPill";
import { type RewardPolicyItem } from "@/lib/admin/RewardPolicyItem";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { ADMIN_REWARD_POLICY_PAGE_SIZE, type RewardPolicyFilters } from "../model/RewardPolicyFilters";
import { humanizeKey, rewardPolicyContext } from "../model/rewardPolicyDisplay";

export function RewardPolicyListPanel({
  error,
  filters,
  hasNextPage,
  onPageOffset,
  onRefresh,
  onSelectPolicy,
  policies,
  selectedPolicyId,
  state,
}: {
  error: RouteError | null;
  filters: RewardPolicyFilters;
  hasNextPage: boolean;
  onPageOffset: (offset: number) => void;
  onRefresh: () => void;
  onSelectPolicy: (policyId: number) => void;
  policies: RewardPolicyItem[];
  selectedPolicyId: number | null;
  state: LoadState;
}) {
  if (state === "loading" || state === "idle") return <PanelLoading title="Loading reward policies" />;
  if (state === "error") return <PanelError error={error} onRetry={onRefresh} title="Reward policies failed" />;

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <ListChecks size={20} aria-hidden />
        <div>
          <h2>Policy versions</h2>
          <p>Newest matching policies from the current scope, event, and active-state filters.</p>
        </div>
        <StatusPill label={`${policies.length} loaded`} />
      </div>
      {policies.length ? (
        <div className={styles.rowList}>
          {policies.map((policy) => (
            <PolicyRow
              key={policy.id}
              onSelectPolicy={onSelectPolicy}
              policy={policy}
              selected={policy.id === selectedPolicyId}
            />
          ))}
        </div>
      ) : (
        <EmptyState text="No reward policies match these filters." />
      )}
      <Pagination filters={filters} hasNextPage={hasNextPage} loaded={policies.length} onPageOffset={onPageOffset} />
    </section>
  );
}

function PolicyRow({
  onSelectPolicy,
  policy,
  selected,
}: {
  onSelectPolicy: (policyId: number) => void;
  policy: RewardPolicyItem;
  selected: boolean;
}) {
  return (
    <article className={`${styles.compactRow} ${selected ? styles.selectedRow : ""}`}>
      <div>
        <strong>{rewardPolicyContext(policy)} policy #{policy.id}</strong>
        <span>
          {humanizeKey(policy.event_type)} · {policy.token_amount} tokens · v{policy.version}
        </span>
        <small>
          {humanizeKey(policy.payment_strategy)} · Multiplier {policy.multiplier} · Cooldown {policy.cooldown_seconds}s
        </small>
      </div>
      <div className={styles.rowMeta}>
        <StatusPill label={policy.active ? "Active" : "Inactive"} tone={policy.active ? "good" : "neutral"} />
        <span>{formatDate(policy.updated_at)}</span>
        <button
          aria-label={`Inspect policy #${policy.id}`}
          aria-pressed={selected}
          className={styles.secondaryButton}
          onClick={() => onSelectPolicy(policy.id)}
          type="button"
        >
          Inspect
        </button>
      </div>
    </article>
  );
}

function Pagination({
  filters,
  hasNextPage,
  loaded,
  onPageOffset,
}: {
  filters: RewardPolicyFilters;
  hasNextPage: boolean;
  loaded: number;
  onPageOffset: (offset: number) => void;
}) {
  const start = loaded ? filters.offset + 1 : 0;
  const end = filters.offset + loaded;

  return (
    <div className={styles.paginationRow}>
      <button className={styles.secondaryButton} disabled={filters.offset <= 0} onClick={() => onPageOffset(filters.offset - ADMIN_REWARD_POLICY_PAGE_SIZE)} type="button">
        Previous
      </button>
      <span>
        Showing {start}-{end}
      </span>
      <button className={styles.secondaryButton} disabled={!hasNextPage} onClick={() => onPageOffset(filters.offset + ADMIN_REWARD_POLICY_PAGE_SIZE)} type="button">
        Next
      </button>
    </div>
  );
}

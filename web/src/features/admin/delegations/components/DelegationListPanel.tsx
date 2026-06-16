"use client";

import { Loader2, RefreshCw, ShieldCheck } from "lucide-react";
import { type DelegationItem } from "@/lib/admin/DelegationItem";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { EmptyState } from "@/components/admin-routes/EmptyState";
import { formatDate } from "@/components/admin-routes/formatDate";
import { formatUnderscoreLabel } from "@/components/admin-routes/formatUnderscoreLabel";
import { PanelError } from "@/components/admin-routes/PanelError";
import { PanelLoading } from "@/components/admin-routes/PanelLoading";
import { StatusPill } from "@/components/admin-routes/StatusPill";
import { delegationScopeLabel, delegationStatus } from "../model/delegationDisplay";

export function DelegationListPanel({
  delegations,
  error,
  onRefresh,
  onSelect,
  selectedDelegation,
  state,
}: {
  delegations: DelegationItem[];
  error: RouteError | null;
  onRefresh: () => void;
  onSelect: (delegation: DelegationItem) => void;
  selectedDelegation: DelegationItem | null;
  state: LoadState;
}) {
  return (
    <div>
      <div className={styles.sectionHeader}>
        <ShieldCheck size={20} aria-hidden />
        <h2>Delegated permissions</h2>
        {state === "loading" ? <Loader2 className={styles.spin} size={18} aria-hidden /> : null}
        <button className={styles.secondaryButton} disabled={state === "loading"} onClick={onRefresh} type="button">
          <RefreshCw size={16} aria-hidden />
          Refresh
        </button>
      </div>
      {state === "loading" || state === "idle" ? <PanelLoading title="Loading delegations" /> : null}
      {state === "error" ? <PanelError error={error} onRetry={onRefresh} title="Delegation list failed" /> : null}
      {state === "success" && !delegations.length ? <EmptyState text="No delegated permissions found." /> : null}
      {state === "success" && delegations.length ? (
        <ul className={styles.queueList} role="list">
          {delegations.map((delegation) => (
            <DelegationListItem
              delegation={delegation}
              isSelected={selectedDelegation?.id === delegation.id}
              key={delegation.id}
              onSelect={onSelect}
            />
          ))}
        </ul>
      ) : null}
    </div>
  );
}

function DelegationListItem({
  delegation,
  isSelected,
  onSelect,
}: {
  delegation: DelegationItem;
  isSelected: boolean;
  onSelect: (delegation: DelegationItem) => void;
}) {
  const status = delegationStatus(delegation);

  return (
    <li
      className={isSelected ? styles.queueItemSelected : styles.queueItem}
      onClick={() => onSelect(delegation)}
      role="button"
      tabIndex={0}
    >
      <div className={styles.queueRow}>
        <strong>{formatUnderscoreLabel(delegation.permission)}</strong>
        <StatusPill label={status} tone={status === "active" ? "good" : "neutral"} />
      </div>
      <div className={styles.queueRow}>
        <small>
          Grantee {delegation.grantee_user_id} · {delegationScopeLabel(delegation)}
        </small>
        <small>{formatDate(delegation.created_at)}</small>
      </div>
    </li>
  );
}

"use client";

import { AlertTriangle, FileSearch, Loader2, Power, PowerOff, RefreshCw } from "lucide-react";
import { type RewardPolicyAuditEvent } from "@/lib/admin/RewardPolicyAuditEvent";
import { type RewardPolicyItem } from "@/lib/admin/RewardPolicyItem";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { EmptyState } from "@/features/admin/shared/route-kit/EmptyState";
import { formatDate } from "@/features/admin/shared/route-kit/formatDate";
import { formatUnderscoreLabel } from "@/features/admin/shared/route-kit/formatUnderscoreLabel";
import { StatusPill } from "@/features/admin/shared/route-kit/StatusPill";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { rewardPolicyInspectionRows } from "../model/rewardPolicyInspection";

export function RewardPolicyInspectionPanel({
  auditError,
  auditEvents,
  auditState,
  onRefreshAudit,
  onSetActive,
  policy,
  policyActionState,
}: {
  auditError: RouteError | null;
  auditEvents: RewardPolicyAuditEvent[];
  auditState: LoadState;
  onRefreshAudit: () => void;
  onSetActive: (active: boolean) => void;
  policy: RewardPolicyItem | null;
  policyActionState: LoadState;
}) {
  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <FileSearch size={20} aria-hidden />
        <div>
          <h2>Policy inspection</h2>
          <p>Selected policy version, payout rules, scope identifiers, and ownership timestamps.</p>
        </div>
        {policy ? (
          <StatusPill label={policy.active ? "Active" : "Inactive"} tone={policy.active ? "good" : "neutral"} />
        ) : null}
      </div>
      {policy ? (
        <>
          <PolicyInspectionDetails policy={policy} />
          <PolicyActivationActions
            onSetActive={onSetActive}
            policy={policy}
            state={policyActionState}
          />
          <PolicyAuditHistory
            error={auditError}
            events={auditEvents}
            onRefresh={onRefreshAudit}
            state={auditState}
          />
        </>
      ) : (
        <EmptyState text="Select a policy version to inspect." />
      )}
    </section>
  );
}

function PolicyInspectionDetails({ policy }: { policy: RewardPolicyItem }) {
  return (
    <div className={styles.detailGrid}>
      <ContextRow label="Policy ID" value={`#${policy.id}`} />
      {rewardPolicyInspectionRows(policy).map((row) => (
        <ContextRow key={row.label} label={row.label} value={row.value} />
      ))}
      <ContextRow label="Created" value={formatDate(policy.created_at)} />
      <ContextRow label="Updated" value={formatDate(policy.updated_at)} />
    </div>
  );
}

function PolicyActivationActions({
  onSetActive,
  policy,
  state,
}: {
  onSetActive: (active: boolean) => void;
  policy: RewardPolicyItem;
  state: LoadState;
}) {
  const loading = state === "loading";

  return (
    <div className={styles.filterActions}>
      <button
        className={styles.secondaryButton}
        disabled={loading || policy.active}
        onClick={() => onSetActive(true)}
        type="button"
      >
        {loading ? <Loader2 className={styles.spin} size={15} aria-hidden /> : <Power size={15} aria-hidden />}
        Activate
      </button>
      <button
        className={styles.secondaryButton}
        disabled={loading || !policy.active}
        onClick={() => onSetActive(false)}
        type="button"
      >
        {loading ? <Loader2 className={styles.spin} size={15} aria-hidden /> : <PowerOff size={15} aria-hidden />}
        Deactivate
      </button>
    </div>
  );
}

function PolicyAuditHistory({
  error,
  events,
  onRefresh,
  state,
}: {
  error: RouteError | null;
  events: RewardPolicyAuditEvent[];
  onRefresh: () => void;
  state: LoadState;
}) {
  return (
    <div className={styles.textBlock}>
      <h3>Policy audit</h3>
      <button className={styles.secondaryButton} disabled={state === "loading"} onClick={onRefresh} type="button">
        {state === "loading" ? <Loader2 className={styles.spin} size={15} aria-hidden /> : <RefreshCw size={15} aria-hidden />}
        Refresh audit
      </button>
      {state === "loading" || state === "idle" ? <p className={styles.muted}>Loading policy audit events.</p> : null}
      {state === "error" ? (
        <div className={styles.inlineError} role="alert">
          <AlertTriangle size={16} aria-hidden />
          <span>{error?.message || "Policy audit could not be loaded."}</span>
        </div>
      ) : null}
      {state === "success" && events.length ? <PolicyAuditList events={events} /> : null}
      {state === "success" && !events.length ? (
        <EmptyState text="No audit events were returned for this policy." />
      ) : null}
    </div>
  );
}

function PolicyAuditList({ events }: { events: RewardPolicyAuditEvent[] }) {
  return (
    <ol className={styles.auditList}>
      {events.map((event) => (
        <li key={event.id}>
          <strong>{formatUnderscoreLabel(event.event_type)}</strong>
          <span>
            {activeLabel(event.previous_active)}
            {" -> "}
            {activeLabel(event.new_active)} · {formatDate(event.created_at)}
          </span>
          <small>Actor {event.actor_user_id ?? "System"}</small>
        </li>
      ))}
    </ol>
  );
}

function activeLabel(active: boolean | null) {
  if (active === null) return "New policy";
  return active ? "Active" : "Inactive";
}

function ContextRow({ label, value }: { label: string; value: string }) {
  return (
    <div className={styles.contextRow}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

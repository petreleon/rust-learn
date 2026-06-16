"use client";

import { AlertTriangle, ClipboardList, Loader2, RefreshCw } from "lucide-react";
import { type AdminUserProfile, type AdminUserRoleAssignmentAuditEvent } from "@/lib/admin";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { EmptyState } from "@/features/admin/shared/route-kit/EmptyState";
import { formatDate } from "@/features/admin/shared/route-kit/formatDate";
import { formatUnderscoreLabel } from "@/features/admin/shared/route-kit/formatUnderscoreLabel";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";

export function RoleAssignmentAuditPanel({
  canViewAudit,
  error,
  events,
  onRefresh,
  state,
  user,
}: {
  canViewAudit: boolean;
  error: RouteError | null;
  events: AdminUserRoleAssignmentAuditEvent[];
  onRefresh: () => void;
  state: LoadState;
  user: AdminUserProfile | null;
}) {
  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <ClipboardList size={20} aria-hidden />
        <div>
          <h2>Role assignment history</h2>
          <p>Persisted platform role changes for the selected user.</p>
        </div>
        <button className={styles.secondaryButton} disabled={!user || state === "loading"} onClick={onRefresh} type="button">
          {state === "loading" ? <Loader2 className={styles.spin} size={15} aria-hidden /> : <RefreshCw size={15} aria-hidden />}
          Refresh
        </button>
      </div>
      {!user ? <EmptyState text="Select a user before reviewing role assignment history." /> : null}
      {user && !canViewAudit ? <EmptyState text="This session is missing VIEW_ROLE_ASSIGNMENTS." /> : null}
      {user && canViewAudit && (state === "loading" || state === "idle") ? (
        <p className={styles.muted}>Loading role assignment history.</p>
      ) : null}
      {user && canViewAudit && state === "error" ? (
        <div className={styles.inlineError} role="alert">
          <AlertTriangle size={16} aria-hidden />
          <span>{error?.message || "Role assignment history could not be loaded."}</span>
        </div>
      ) : null}
      {user && canViewAudit && state === "success" && events.length ? (
        <ol className={styles.auditList}>
          {events.map((event) => (
            <li key={event.id}>
              <strong>{formatUnderscoreLabel(event.event_type)}</strong>
              <span>{event.role_name} · {formatDate(event.created_at)}</span>
              <small>{event.actor_user_id ? `Actor user ${event.actor_user_id}` : "Actor unavailable"}</small>
            </li>
          ))}
        </ol>
      ) : null}
      {user && canViewAudit && state === "success" && !events.length ? (
        <EmptyState text="No role assignment history was returned for this user." />
      ) : null}
    </section>
  );
}

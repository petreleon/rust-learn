"use client";

import { AlertTriangle, Loader2, RefreshCw } from "lucide-react";
import { type TeacherApplicationAuditEvent } from "@/lib/admin/TeacherApplicationAuditEvent";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { EmptyState } from "@/components/admin-routes/EmptyState";
import { formatDate } from "@/components/admin-routes/formatDate";
import { formatUnderscoreLabel } from "@/components/admin-routes/formatUnderscoreLabel";

export function TeacherApplicationAuditPanel({
  error,
  events,
  onRefresh,
  state,
}: {
  error: RouteError | null;
  events: TeacherApplicationAuditEvent[];
  onRefresh: () => void;
  state: LoadState;
}) {
  return (
    <div className={styles.textBlock}>
      <div className={styles.subsectionHeader}>
        <h3>Audit history</h3>
        <button className={styles.secondaryButton} disabled={state === "loading"} onClick={onRefresh} type="button">
          {state === "loading" ? <Loader2 className={styles.spin} size={15} aria-hidden /> : <RefreshCw size={15} aria-hidden />}
          Refresh
        </button>
      </div>
      {state === "loading" || state === "idle" ? <p className={styles.muted}>Loading audit events.</p> : null}
      {state === "error" ? (
        <div className={styles.inlineError} role="alert">
          <AlertTriangle size={16} aria-hidden />
          <span>{error?.message || "Audit history could not be loaded."}</span>
        </div>
      ) : null}
      {state === "success" && events.length ? (
        <ol className={styles.auditList}>
          {events.map((event) => (
            <li key={event.id}>
              <strong>{formatUnderscoreLabel(event.event_type)}</strong>
              <span>
                {event.from_status ? `${formatUnderscoreLabel(event.from_status)} -> ` : ""}
                {formatUnderscoreLabel(event.to_status)} · {formatDate(event.created_at)}
              </span>
              {event.reason ? <small>{event.reason}</small> : null}
            </li>
          ))}
        </ol>
      ) : null}
      {state === "success" && !events.length ? (
        <EmptyState text="No audit events were returned for this application." />
      ) : null}
    </div>
  );
}

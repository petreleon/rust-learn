"use client";

import { AlertTriangle, Loader2, RefreshCw } from "lucide-react";
import { type KycAuditEvent } from "@/lib/admin";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { EmptyState } from "./EmptyState";
import { formatDate } from "./formatDate";
import { formatUnderscoreLabel } from "./formatUnderscoreLabel";
import { type RouteError } from "./RouteError";
import { type SectionState } from "./SectionState";

export function KycAuditTimeline({
  auditError,
  auditEvents,
  auditState,
  onRefreshAudit,
}: {
  auditError: RouteError | null;
  auditEvents: KycAuditEvent[];
  auditState: SectionState;
  onRefreshAudit: () => void;
}) {
  return (
    <div className={styles.textBlock}>
      <div className={styles.subsectionHeader}>
        <h3>Audit history</h3>
        <button className={styles.secondaryButton} disabled={auditState === "loading"} onClick={onRefreshAudit} type="button">
          {auditState === "loading" ? <Loader2 className={styles.spin} size={15} aria-hidden /> : <RefreshCw size={15} aria-hidden />}
          Refresh
        </button>
      </div>
      {auditState === "loading" || auditState === "idle" ? <p className={styles.muted}>Loading audit events.</p> : null}
      {auditState === "error" ? (
        <div className={styles.inlineError} role="alert">
          <AlertTriangle size={16} aria-hidden />
          <span>{auditError?.message || "Audit history could not be loaded."}</span>
        </div>
      ) : null}
      {auditState === "success" && auditEvents.length ? (
        <ol className={styles.auditList}>
          {auditEvents.map((event) => (
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
      {auditState === "success" && !auditEvents.length ? <EmptyState text="No audit events were returned for this KYC submission." /> : null}
    </div>
  );
}

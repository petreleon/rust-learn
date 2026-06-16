"use client";

import { Clock3 } from "lucide-react";
import { type TeacherApplicationAuditEvent } from "@/lib/teacher";
import styles from "@/features/teacher/application/page.module.css";
import { formatDate } from "../model/formatDate";
import { statusLabel } from "../model/statusLabel";

export function AuditTimeline({
  auditEvents,
  decisionReason,
}: {
  auditEvents: TeacherApplicationAuditEvent[];
  decisionReason: string | null;
}) {
  return (
    <section className={styles.timelinePanel}>
      <div className={styles.panelHeader}>
        <Clock3 size={20} aria-hidden />
        <h2>Review history</h2>
      </div>
      {decisionReason ? <p className={styles.reviewReason}>{decisionReason}</p> : null}
      {auditEvents.length ? (
        <ol className={styles.timelineList}>
          {auditEvents.map((event) => (
            <li key={event.id}>
              <span className={styles.timelineDot} aria-hidden />
              <div>
                <strong>{statusLabel(event.to_status)}</strong>
                <small>{formatDate(event.created_at)}</small>
                {event.reason ? <p>{event.reason}</p> : null}
              </div>
            </li>
          ))}
        </ol>
      ) : (
        <p className={styles.muted}>Review events will appear after the application is submitted.</p>
      )}
    </section>
  );
}

"use client";

import { UserCheck } from "lucide-react";
import { type PlatformTeacherApplicationsResponse } from "@/lib/admin";
import styles from "../admin-routes.module.css";
import { EmptyState } from "./EmptyState";
import { StatusPill } from "./StatusPill";
import { formatDate } from "./formatDate";
import { formatUnderscoreLabel } from "./formatUnderscoreLabel";
import { scopeTargetLabel } from "./scopeTargetLabel";
import { statusTone } from "./statusTone";

export function TeacherApplicationQueue({
  applications,
  offset,
  onPage,
  onSelect,
  selectedApplicationId,
}: {
  applications: PlatformTeacherApplicationsResponse;
  offset: number;
  onPage: (offset: number) => void;
  onSelect: (applicationId: number) => void;
  selectedApplicationId: number | null;
}) {
  const hasPrevious = offset > 0;
  const hasNext = offset + applications.limit < applications.total;

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <UserCheck size={20} aria-hidden />
        <div>
          <h2>Review queue</h2>
          <p>
            {applications.total} application{applications.total === 1 ? "" : "s"}{" "}
            {applications.total === 1 ? "matches" : "match"} the current filters.
          </p>
        </div>
      </div>

      {applications.applications.length ? (
        <div className={styles.rowList}>
          {applications.applications.map((application) => (
            <article
              className={`${styles.compactRow} ${selectedApplicationId === application.id ? styles.selectedRow : ""}`}
              key={application.id}
            >
              <div>
                <strong>{application.applicant.name}</strong>
                <span>{application.applicant.email}</span>
                <small>
                  {formatUnderscoreLabel(application.requested_scope)} · {scopeTargetLabel(application)}
                </small>
                <small>{application.experience_summary}</small>
              </div>
              <div className={styles.rowMeta}>
                <StatusPill label={formatUnderscoreLabel(application.status)} tone={statusTone(application.status)} />
                <span>{formatDate(application.updated_at)}</span>
                <button
                  aria-label={`Review ${application.applicant.name}`}
                  className={styles.secondaryButton}
                  onClick={() => onSelect(application.id)}
                  type="button"
                >
                  Review
                </button>
              </div>
            </article>
          ))}
        </div>
      ) : (
        <EmptyState text="No teacher applications match these filters." />
      )}

      <div className={styles.paginationRow}>
        <button className={styles.secondaryButton} disabled={!hasPrevious} onClick={() => onPage(Math.max(0, offset - applications.limit))} type="button">
          Previous
        </button>
        <span>
          Showing {applications.applications.length ? offset + 1 : 0}-
          {Math.min(offset + applications.applications.length, applications.total)} of {applications.total}
        </span>
        <button className={styles.secondaryButton} disabled={!hasNext} onClick={() => onPage(offset + applications.limit)} type="button">
          Next
        </button>
      </div>
    </section>
  );
}

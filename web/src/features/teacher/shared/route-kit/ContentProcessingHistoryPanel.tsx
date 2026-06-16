"use client";

import { type ContentProcessingHistoryState } from "./ContentProcessingHistoryState";
import { formatDateTime } from "./formatDateTime";
import { statusLabel } from "./statusLabel";
import styles from "@/features/teacher/shared/teacher-routes.module.css";

export function ContentProcessingHistoryPanel({
  state,
}: {
  state: ContentProcessingHistoryState;
}) {
  if (state.status === "loading") {
    return <p className={styles.muted}>Loading processing history.</p>;
  }
  if (state.status === "error") {
    return <p className={styles.reviewNote}>{state.message}</p>;
  }
  if (!state.history) return null;

  return (
    <div className={styles.contentHistory}>
      <strong>Processing history</strong>
      <p className={styles.muted}>{state.history.object_key || "No upload object key is stored for this item."}</p>
      {state.history.jobs.length ? (
        <ul>
          {state.history.jobs.map((job) => (
            <li key={job.id}>
              <span>
                {statusLabel(job.status)} - {job.attempts} attempt{job.attempts === 1 ? "" : "s"} -{" "}
                {formatDateTime(job.updated_at || job.created_at)}
              </span>
              {job.last_error ? <small>{job.last_error}</small> : null}
            </li>
          ))}
        </ul>
      ) : (
        <p className={styles.muted}>No processing jobs have been queued for this content.</p>
      )}
    </div>
  );
}

"use client";

import { IdCard } from "lucide-react";
import { type KycSubmission } from "@/lib/admin";
import styles from "../admin-routes.module.css";
import { EmptyState } from "./EmptyState";
import { StatusPill } from "./StatusPill";
import { formatDate } from "./formatDate";
import { formatUnderscoreLabel } from "./formatUnderscoreLabel";
import { kycStatusTone } from "./kycStatusTone";

export function KycReviewQueue({
  onSelect,
  selectedSubmissionId,
  submissions,
}: {
  onSelect: (submissionId: number) => void;
  selectedSubmissionId: number | null;
  submissions: KycSubmission[];
}) {
  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <IdCard size={20} aria-hidden />
        <div>
          <h2>Review queue</h2>
          <p>{submissions.length} KYC submission{submissions.length === 1 ? "" : "s"} need a platform decision.</p>
        </div>
      </div>
      {submissions.length ? (
        <div className={styles.rowList}>
          {submissions.map((submission) => (
            <article
              className={`${styles.compactRow} ${selectedSubmissionId === submission.id ? styles.selectedRow : ""}`}
              key={submission.id}
            >
              <div>
                <strong>{submission.legal_name}</strong>
                <span>User #{submission.user_id}</span>
                <small>
                  {submission.country_code} · {formatUnderscoreLabel(submission.document_type)}
                  {submission.document_last4 ? ` ending ${submission.document_last4}` : ""}
                </small>
                <small>Submitted {formatDate(submission.submitted_at)}</small>
              </div>
              <div className={styles.rowMeta}>
                <StatusPill label={formatUnderscoreLabel(submission.status)} tone={kycStatusTone(submission.status)} />
                <button
                  aria-label={`Review KYC submission ${submission.id}`}
                  className={styles.secondaryButton}
                  onClick={() => onSelect(submission.id)}
                  type="button"
                >
                  Review
                </button>
              </div>
            </article>
          ))}
        </div>
      ) : (
        <EmptyState text="No KYC submissions are waiting for review." />
      )}
    </section>
  );
}

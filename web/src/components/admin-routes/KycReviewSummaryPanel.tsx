"use client";

import { ShieldCheck } from "lucide-react";
import { type KycReviewQueueResponse } from "@/lib/admin";
import styles from "../admin-routes.module.css";
import { MetricCard } from "./MetricCard";
import { type SectionState } from "./SectionState";

export function KycReviewSummaryPanel({
  onRefresh,
  queue,
  state,
}: {
  onRefresh: () => void;
  queue: KycReviewQueueResponse | null;
  state: SectionState;
}) {
  const submitted = queue?.submissions.filter((submission) => submission.status === "submitted").length ?? 0;
  const underReview = queue?.submissions.filter((submission) => submission.status === "under_review").length ?? 0;

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <ShieldCheck size={20} aria-hidden />
        <div>
          <h2>KYC queue health</h2>
          <p>Reviewable submissions are limited to submitted and under-review records from the API queue.</p>
        </div>
        <button className={styles.secondaryButton} disabled={state === "loading"} onClick={onRefresh} type="button">
          Refresh queue
        </button>
      </div>
      <div className={styles.metricGrid}>
        <MetricCard label="Reviewable submissions" tone={queue?.submissions.length ? "warn" : "neutral"} value={queue?.submissions.length ?? (state === "loading" ? "Loading" : 0)} />
        <MetricCard label="Submitted" tone={submitted ? "warn" : "neutral"} value={submitted} />
        <MetricCard label="Under review" tone={underReview ? "warn" : "neutral"} value={underReview} />
      </div>
    </section>
  );
}

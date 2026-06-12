"use client";

import { AlertTriangle, FileText, Loader2, Send, ShieldCheck } from "lucide-react";
import { type FormEvent } from "react";
import { type KycAuditEvent, type KycDecisionStatus, type KycSubmission } from "@/lib/admin";
import styles from "../admin-routes.module.css";
import { ContextRow } from "./ContextRow";
import { KycAuditTimeline } from "./KycAuditTimeline";
import { StatusPill } from "./StatusPill";
import { formatDate } from "./formatDate";
import { formatUnderscoreLabel } from "./formatUnderscoreLabel";
import { kycStatusTone } from "./kycStatusTone";
import { type RouteError } from "./RouteError";
import { type SectionState } from "./SectionState";

export function KycReviewDetail({
  auditError,
  auditEvents,
  auditState,
  decisionError,
  decisionState,
  decisionStatus,
  onDecisionStatusChange,
  onRefreshAudit,
  onRejectionReasonChange,
  onSubmitDecision,
  rejectionReason,
  submission,
}: {
  auditError: RouteError | null;
  auditEvents: KycAuditEvent[];
  auditState: SectionState;
  decisionError: RouteError | null;
  decisionState: "idle" | "submitting" | "success" | "error";
  decisionStatus: KycDecisionStatus;
  onDecisionStatusChange: (status: KycDecisionStatus) => void;
  onRefreshAudit: () => void;
  onRejectionReasonChange: (value: string) => void;
  onSubmitDecision: (event: FormEvent<HTMLFormElement>) => void;
  rejectionReason: string;
  submission: KycSubmission | null;
}) {
  if (!submission) {
    return (
      <section className={styles.panel}>
        <div className={styles.panelHeader}>
          <ShieldCheck size={20} aria-hidden />
          <div>
            <h2>KYC detail</h2>
            <p>Select a submission from the queue to inspect evidence and decide verification.</p>
          </div>
        </div>
      </section>
    );
  }

  const canSubmitDecision =
    decisionState !== "submitting" && (decisionStatus === "verified" || rejectionReason.trim().length > 0);

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <FileText size={20} aria-hidden />
        <div>
          <h2>{submission.legal_name}</h2>
          <p>User #{submission.user_id}</p>
        </div>
        <StatusPill label={formatUnderscoreLabel(submission.status)} tone={kycStatusTone(submission.status)} />
      </div>

      <div className={styles.detailGrid}>
        <ContextRow label="Country" value={submission.country_code} />
        <ContextRow label="Document type" value={formatUnderscoreLabel(submission.document_type)} />
        <ContextRow label="Document last 4" value={submission.document_last4 || "Not provided"} />
        <ContextRow label="Submitted" value={formatDate(submission.submitted_at)} />
        <ContextRow label="Latest update" value={formatDate(submission.updated_at)} />
        <ContextRow label="Reviewer" value={submission.reviewer_user_id ? `User #${submission.reviewer_user_id}` : "No reviewer yet"} />
      </div>

      <div className={styles.textBlock}>
        <h3>Evidence</h3>
        <p>{submission.evidence_reference || "No evidence reference was submitted."}</p>
        {submission.provider_reference ? <p className={styles.muted}>Provider reference: {submission.provider_reference}</p> : null}
      </div>

      {submission.rejection_reason ? (
        <div className={styles.inlineNotice}>
          <strong>Latest rejection reason</strong>
          <span>{submission.rejection_reason}</span>
        </div>
      ) : null}

      <KycAuditTimeline auditError={auditError} auditEvents={auditEvents} auditState={auditState} onRefreshAudit={onRefreshAudit} />

      <form className={styles.decisionForm} onSubmit={onSubmitDecision}>
        <div className={styles.subsectionHeader}>
          <h3>Decision</h3>
          <StatusPill label={decisionStatus === "verified" ? "Verify account" : "Reject submission"} tone={decisionStatus === "verified" ? "good" : "warn"} />
        </div>
        <label>
          <span>Decision status</span>
          <select onChange={(event) => onDecisionStatusChange(event.target.value as KycDecisionStatus)} value={decisionStatus}>
            <option value="verified">Verify</option>
            <option value="rejected">Reject</option>
          </select>
        </label>
        <label>
          <span>Rejection reason</span>
          <textarea
            onChange={(event) => onRejectionReasonChange(event.target.value)}
            placeholder="Required only when rejecting KYC."
            rows={4}
            value={rejectionReason}
          />
        </label>
        {decisionStatus === "rejected" && !rejectionReason.trim() ? (
          <p className={styles.muted}>A rejection reason is required before rejecting KYC.</p>
        ) : null}
        {decisionError ? (
          <div className={styles.inlineError} role="alert">
            <AlertTriangle size={16} aria-hidden />
            <span>{decisionError.message}</span>
          </div>
        ) : null}
        <button className={styles.primaryButton} disabled={!canSubmitDecision} type="submit">
          {decisionState === "submitting" ? <Loader2 className={styles.spin} size={16} aria-hidden /> : <Send size={16} aria-hidden />}
          Save KYC decision
        </button>
      </form>
    </section>
  );
}

"use client";

import { AlertTriangle, FileText, Loader2, Send, UserCheck } from "lucide-react";
import { type FormEvent } from "react";
import { type PlatformTeacherApplicationItem, type TeacherApplicationAuditEvent, type TeacherApplicationStatus } from "@/lib/admin";
import styles from "../admin-routes.module.css";
import { AuditTimeline } from "./AuditTimeline";
import { ContextRow } from "./ContextRow";
import { StatusPill } from "./StatusPill";
import { formatDate } from "./formatDate";
import { formatUnderscoreLabel } from "./formatUnderscoreLabel";
import { isFinalApplicationStatus } from "./isFinalApplicationStatus";
import { scopeTargetLabel } from "./scopeTargetLabel";
import { statusTone } from "./statusTone";
import { type RouteError } from "./RouteError";
import { type SectionState } from "./SectionState";

export function TeacherApplicationDetail({
  application,
  auditError,
  auditEvents,
  auditState,
  canApprove,
  canReject,
  decisionError,
  decisionReason,
  decisionState,
  decisionStatus,
  onDecisionReasonChange,
  onDecisionStatusChange,
  onRefreshAudit,
  onSubmitDecision,
}: {
  application: PlatformTeacherApplicationItem | null;
  auditError: RouteError | null;
  auditEvents: TeacherApplicationAuditEvent[];
  auditState: SectionState;
  canApprove: boolean;
  canReject: boolean;
  decisionError: RouteError | null;
  decisionReason: string;
  decisionState: "idle" | "submitting" | "success" | "error";
  decisionStatus: Extract<TeacherApplicationStatus, "approved" | "needs_changes" | "rejected">;
  onDecisionReasonChange: (value: string) => void;
  onDecisionStatusChange: (value: Extract<TeacherApplicationStatus, "approved" | "needs_changes" | "rejected">) => void;
  onRefreshAudit: () => void;
  onSubmitDecision: (event: FormEvent<HTMLFormElement>) => void;
}) {
  if (!application) {
    return (
      <section className={styles.panel}>
        <div className={styles.panelHeader}>
          <UserCheck size={20} aria-hidden />
          <div>
            <h2>Application detail</h2>
            <p>Select an application from the queue to inspect context and audit history.</p>
          </div>
        </div>
      </section>
    );
  }

  const isFinal = isFinalApplicationStatus(application.status);
  const canSubmitDecision =
    !isFinal &&
    decisionReason.trim().length > 0 &&
    (decisionStatus === "needs_changes" || (decisionStatus === "approved" && canApprove) || (decisionStatus === "rejected" && canReject)) &&
    decisionState !== "submitting";

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <FileText size={20} aria-hidden />
        <div>
          <h2>{application.applicant.name}</h2>
          <p>{application.applicant.email}</p>
        </div>
        <StatusPill label={formatUnderscoreLabel(application.status)} tone={statusTone(application.status)} />
      </div>

      <div className={styles.detailGrid}>
        <ContextRow label="Requested scope" value={formatUnderscoreLabel(application.requested_scope)} />
        <ContextRow label="Target" value={scopeTargetLabel(application)} />
        <ContextRow label="Sponsor" value={application.sponsor_organization?.name || "No sponsor recorded"} />
        <ContextRow label="Submitted" value={formatDate(application.created_at)} />
        <ContextRow label="Latest update" value={formatDate(application.updated_at)} />
        <ContextRow label="Reviewer" value={application.reviewer?.name || "No reviewer yet"} />
      </div>

      <div className={styles.textBlock}>
        <h3>Experience summary</h3>
        <p>{application.experience_summary}</p>
      </div>

      <div className={styles.textBlock}>
        <h3>Portfolio</h3>
        {application.portfolio_links.length ? (
          <div className={styles.linkList}>
            {application.portfolio_links.map((link) => (
              <a href={link} key={link} rel="noreferrer" target="_blank">
                {link}
              </a>
            ))}
          </div>
        ) : (
          <p className={styles.muted}>No portfolio links were submitted.</p>
        )}
      </div>

      {application.decision_reason ? (
        <div className={styles.inlineNotice}>
          <strong>Latest decision note</strong>
          <span>{application.decision_reason}</span>
        </div>
      ) : null}

      <AuditTimeline
        auditError={auditError}
        auditEvents={auditEvents}
        auditState={auditState}
        onRefreshAudit={onRefreshAudit}
      />

      <form className={styles.decisionForm} onSubmit={onSubmitDecision}>
        <div className={styles.subsectionHeader}>
          <h3>Decision</h3>
          {isFinal ? <StatusPill label="Final state" tone="neutral" /> : null}
        </div>
        <label>
          <span>Decision status</span>
          <select
            disabled={isFinal}
            onChange={(event) =>
              onDecisionStatusChange(event.target.value as Extract<TeacherApplicationStatus, "approved" | "needs_changes" | "rejected">)
            }
            value={decisionStatus}
          >
            <option value="needs_changes">Needs changes</option>
            <option disabled={!canApprove} value="approved">
              Approve
            </option>
            <option disabled={!canReject} value="rejected">
              Reject
            </option>
          </select>
        </label>
        {!canApprove || !canReject ? (
          <p className={styles.muted}>
            Approve and reject require the matching platform permissions; requesting changes remains available to reviewers.
          </p>
        ) : null}
        <label>
          <span>Decision reason</span>
          <textarea
            disabled={isFinal}
            onChange={(event) => onDecisionReasonChange(event.target.value)}
            placeholder="Explain what changed, what is missing, or why the application is approved."
            rows={4}
            value={decisionReason}
          />
        </label>
        {!isFinal && !decisionReason.trim() ? (
          <p className={styles.muted}>A decision reason is required before saving.</p>
        ) : null}
        {decisionError ? (
          <div className={styles.inlineError} role="alert">
            <AlertTriangle size={16} aria-hidden />
            <span>{decisionError.message}</span>
          </div>
        ) : null}
        <button className={styles.primaryButton} disabled={!canSubmitDecision} type="submit">
          {decisionState === "submitting" ? <Loader2 className={styles.spin} size={16} aria-hidden /> : <Send size={16} aria-hidden />}
          Save decision
        </button>
        {isFinal ? <p className={styles.muted}>Approved and rejected applications are final in the current backend contract.</p> : null}
      </form>
    </section>
  );
}

"use client";

import { Send } from "lucide-react";
import { type FormEvent } from "react";
import { type TeacherCourseJoinRequestItem } from "@/lib/teacher";
import styles from "@/features/teacher/shared/teacher-routes.module.css";
import { DetailLine } from "./DetailLine";
import { formatDateTime } from "./formatDateTime";
import { joinRequestTone } from "./joinRequestTone";
import { statusLabel } from "./statusLabel";
import { type ActionState } from "./ActionState";
import { type DecisionDraft } from "./DecisionDraft";
import { type DecisionStatus } from "./DecisionStatus";

export function EnrollmentRequestCard({
  actionState,
  draft,
  onDraftChange,
  onSubmit,
  request,
}: {
  actionState: ActionState;
  draft: DecisionDraft;
  onDraftChange: (draft: DecisionDraft) => void;
  onSubmit: (event: FormEvent<HTMLFormElement>) => void;
  request: TeacherCourseJoinRequestItem;
}) {
  return (
    <article className={styles.enrollmentCard}>
      <div className={styles.courseTop}>
        <div>
          <p className={styles.eyebrow}>{request.requester.email}</p>
          <h3>{request.requester.name}</h3>
        </div>
        <span className={`${styles.statusPill} ${styles[joinRequestTone(request.status)]}`}>
          {statusLabel(request.status)}
        </span>
      </div>

      <div className={styles.detailList}>
        <DetailLine label="Requested" value={formatDateTime(request.created_at)} />
        <DetailLine label="Updated" value={formatDateTime(request.updated_at)} />
        <DetailLine label="Email" value={request.requester.email_verified ? "Verified" : "Needs verification"} />
        <DetailLine label="KYC" value={request.requester.kyc_verified ? "Verified" : "Not verified"} />
        {request.decision_reason ? <DetailLine label="Reason" value={request.decision_reason} /> : null}
      </div>

      {request.can_decide ? (
        <form className={styles.decisionForm} onSubmit={onSubmit}>
          <label>
            <span>Decision</span>
            <select
              disabled={actionState === "saving"}
              onChange={(event) => onDraftChange({ ...draft, status: event.target.value as DecisionStatus })}
              value={draft.status}
            >
              <option value="approved">Approve</option>
              <option value="waitlisted">Waitlist</option>
              <option value="rejected">Reject</option>
            </select>
          </label>
          <label>
            <span>Reason</span>
            <textarea
              disabled={actionState === "saving"}
              onChange={(event) => onDraftChange({ ...draft, reason: event.target.value })}
              placeholder="Decision reason"
              rows={3}
              value={draft.reason}
            />
          </label>
          <button className={styles.primaryButton} disabled={actionState === "saving"} type="submit">
            <Send size={17} aria-hidden />
            Apply decision
          </button>
        </form>
      ) : (
        <p className={styles.muted}>This request has already been decided.</p>
      )}
    </article>
  );
}

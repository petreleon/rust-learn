"use client";

import { Send } from "lucide-react";
import { type FormEvent } from "react";
import { type TeacherEnrollmentUserSummary, type TeacherRewardCandidate, type TeacherRewardCandidateDecisionStatus } from "@/lib/teacher";
import styles from "../teacher-routes.module.css";
import { DetailLine } from "./DetailLine";
import { formatDateTime } from "./formatDateTime";
import { rewardCandidateTone } from "./rewardCandidateTone";
import { statusLabel } from "./statusLabel";
import { summarizeEvidence } from "./summarizeEvidence";
import { type ActionState } from "./ActionState";
import { type RewardDecisionDraft } from "./RewardDecisionDraft";

export function RewardCandidateCard({
  actionState,
  canApprove,
  candidate,
  draft,
  learner,
  onDraftChange,
  onSubmit,
}: {
  actionState: ActionState;
  canApprove: boolean;
  candidate: TeacherRewardCandidate;
  draft: RewardDecisionDraft;
  learner: TeacherEnrollmentUserSummary | null;
  onDraftChange: (draft: RewardDecisionDraft) => void;
  onSubmit: (event: FormEvent<HTMLFormElement>) => void;
}) {
  const canDecide = canApprove && candidate.status === "pending_teacher_approval";
  return (
    <article className={styles.enrollmentCard}>
      <div className={styles.courseTop}>
        <div>
          <p className={styles.eyebrow}>{learner?.email || "Learner details unavailable"}</p>
          <h3>{learner?.name || "Learner unavailable"}</h3>
        </div>
        <span className={`${styles.statusPill} ${styles[rewardCandidateTone(candidate.status)]}`}>
          {statusLabel(candidate.status)}
        </span>
      </div>

      <div className={styles.detailList}>
        <DetailLine label="Event" value={statusLabel(candidate.event_type)} />
        <DetailLine label="Evidence" value={summarizeEvidence(candidate.evidence)} />
        <DetailLine label="Source" value={statusLabel(candidate.source_scope)} />
        <DetailLine label="Created" value={formatDateTime(candidate.created_at)} />
        <DetailLine label="Updated" value={formatDateTime(candidate.updated_at)} />
        {candidate.teacher_decided_at ? <DetailLine label="Teacher decided" value={formatDateTime(candidate.teacher_decided_at)} /> : null}
        {candidate.teacher_decision_reason ? <DetailLine label="Teacher reason" value={candidate.teacher_decision_reason} /> : null}
      </div>

      {canDecide ? (
        <form className={styles.decisionForm} onSubmit={onSubmit}>
          <label>
            <span>Teacher decision</span>
            <select
              disabled={actionState === "saving"}
              onChange={(event) =>
                onDraftChange({
                  ...draft,
                  status: event.target.value as TeacherRewardCandidateDecisionStatus,
                })
              }
              value={draft.status}
            >
              <option value="teacher_approved">Approve evidence</option>
              <option value="teacher_rejected">Reject evidence</option>
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
            Apply teacher decision
          </button>
        </form>
      ) : (
        <p className={styles.muted}>
          {candidate.status === "pending_teacher_approval"
            ? "This session can view the candidate but cannot apply the teacher decision."
            : "Teacher review is already recorded or this candidate has moved to later processing."}
        </p>
      )}
    </article>
  );
}

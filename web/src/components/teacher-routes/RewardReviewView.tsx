"use client";

import { AlertCircle, ArrowLeft, CheckCircle2, Clock3, RefreshCw, ShieldCheck, Trophy } from "lucide-react";
import Link from "next/link";
import { type FormEvent, useMemo } from "react";
import { type TeacherCourseStudentsResponse, type TeacherRewardCandidate, type TeacherRewardCandidateStatusFilter } from "@/lib/teacher";
import styles from "../teacher-routes.module.css";
import { PermissionChip } from "./PermissionChip";
import { RewardCandidateCard } from "./RewardCandidateCard";
import { StatePanel } from "./StatePanel";
import { SummaryCard } from "./SummaryCard";
import { defaultRewardDecisionDraft } from "./defaultRewardDecisionDraft";
import { rewardStatusOptions } from "./rewardStatusOptions";
import { statusLabel } from "./statusLabel";
import { type ActionState } from "./ActionState";
import { type RewardDecisionDraft } from "./RewardDecisionDraft";

export function RewardReviewView({
  actionMessage,
  actionState,
  candidates,
  decisionDrafts,
  onDecisionDraftChange,
  onRefresh,
  onStatusFilterChange,
  onSubmitDecision,
  statusFilter,
  students,
}: {
  actionMessage: string | null;
  actionState: ActionState;
  candidates: TeacherRewardCandidate[];
  decisionDrafts: Record<number, RewardDecisionDraft>;
  onDecisionDraftChange: (candidateId: number, draft: RewardDecisionDraft) => void;
  onRefresh: () => void;
  onStatusFilterChange: (status: TeacherRewardCandidateStatusFilter) => void;
  onSubmitDecision: (candidate: TeacherRewardCandidate, event: FormEvent<HTMLFormElement>) => void;
  statusFilter: TeacherRewardCandidateStatusFilter;
  students: TeacherCourseStudentsResponse;
}) {
  const learnersById = useMemo(
    () => new Map(students.students.map((student) => [student.user.id, student.user])),
    [students.students],
  );
  const canApprove = students.course.permissions.can_approve_reward_candidates;
  const canView = canApprove || students.course.permissions.can_view_reward_candidates;
  const pendingShown = candidates.filter((candidate) => candidate.status === "pending_teacher_approval").length;
  const decidedShown = candidates.filter(
    (candidate) => candidate.status === "teacher_approved" || candidate.status === "teacher_rejected",
  ).length;

  return (
    <>
      <section className={styles.workspaceHero}>
        <Link className={styles.secondaryLink} href={`/teach/courses/${students.course.id}`}>
          <ArrowLeft size={17} aria-hidden />
          Course workspace
        </Link>
        <div className={styles.workspaceTitleBlock}>
          <p className={styles.eyebrow}>{statusLabel(students.course.lifecycle_status)}</p>
          <h2>Reward review</h2>
          <p className={styles.muted}>
            Review learner evidence for this course and apply the teacher decision. Platform payout controls stay in admin workflows.
          </p>
        </div>
        <div className={styles.permissionRow} aria-label="Reward permissions">
          <PermissionChip enabled={canView} label="View rewards" />
          <PermissionChip enabled={canApprove} label="Teacher decision" />
        </div>
      </section>

      <section className={styles.summaryGrid}>
        <SummaryCard icon={<Clock3 size={20} aria-hidden />} label="Pending queue" value={students.course.reward_queue.pending_teacher_count} tone={students.course.reward_queue.pending_teacher_count ? "warn" : "neutral"} />
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Shown now" value={candidates.length} tone={candidates.length ? "good" : "neutral"} />
        <SummaryCard icon={<CheckCircle2 size={20} aria-hidden />} label="Decided shown" value={decidedShown} tone={decidedShown ? "good" : "neutral"} />
        <SummaryCard icon={<AlertCircle size={20} aria-hidden />} label="Failed queue" value={students.course.reward_queue.failed_count} tone={students.course.reward_queue.failed_count ? "warn" : "neutral"} />
      </section>

      <section className={`${styles.warningPanel} ${styles.singlePanel}`}>
        <div className={styles.panelHeader}>
          <ShieldCheck size={18} aria-hidden />
          <h2>Review boundary</h2>
        </div>
        <p>
          This page records teacher approval or rejection only. Financial review remains separated from the course workspace.
        </p>
      </section>

      {actionMessage ? (
        <section className={`${styles.warningPanel} ${styles.singlePanel}`} role="status">
          <div className={styles.panelHeader}>
            <AlertCircle size={18} aria-hidden />
            <h2>Reward update</h2>
          </div>
          <p>{actionMessage}</p>
        </section>
      ) : null}

      <section className={styles.filterPanel}>
        <label>
          <span>Status</span>
          <select
            value={statusFilter}
            onChange={(event) => onStatusFilterChange(event.target.value as TeacherRewardCandidateStatusFilter)}
          >
            {rewardStatusOptions.map((status) => (
              <option key={status} value={status}>
                {status === "all" ? "All statuses" : statusLabel(status)}
              </option>
            ))}
          </select>
        </label>
        <button className={styles.secondaryButton} type="button" onClick={onRefresh}>
          <RefreshCw size={17} aria-hidden />
          Refresh
        </button>
      </section>

      <section className={styles.courseSection}>
        <div className={styles.sectionHeader}>
          <div>
            <h2>Reward candidates</h2>
            <p className={styles.muted}>
              {statusFilter === "all" ? "Showing all visible candidates." : `Showing ${statusLabel(statusFilter)} candidates.`}
            </p>
          </div>
          <span className={`${styles.statusPill} ${pendingShown ? styles.warn : styles.neutral}`}>
            {pendingShown} pending shown
          </span>
        </div>
        {candidates.length ? (
          <div className={styles.enrollmentList}>
            {candidates.map((candidate) => (
              <RewardCandidateCard
                actionState={actionState}
                canApprove={canApprove}
                candidate={candidate}
                draft={decisionDrafts[candidate.id] || defaultRewardDecisionDraft}
                key={candidate.id}
                learner={learnersById.get(candidate.student_user_id) || null}
                onDraftChange={(draft) => onDecisionDraftChange(candidate.id, draft)}
                onSubmit={(event) => onSubmitDecision(candidate, event)}
              />
            ))}
          </div>
        ) : (
          <StatePanel
            detail="No reward candidates match this course filter."
            icon={<CheckCircle2 size={22} aria-hidden />}
            title="No matching candidates"
          />
        )}
      </section>
    </>
  );
}

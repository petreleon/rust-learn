"use client";

import { AlertCircle, ArrowLeft, CheckCircle2, RefreshCw, Users } from "lucide-react";
import Link from "next/link";
import { type FormEvent } from "react";
import { type TeacherCourseEnrollmentWorkspaceResponse, type TeacherCourseJoinRequestItem, type TeacherCourseRosterLearner } from "@/lib/teacher";
import styles from "../teacher-routes.module.css";
import { EnrollmentRequestCard } from "./EnrollmentRequestCard";
import { EnrollmentWorkspaceSummaryGrid } from "./EnrollmentWorkspaceSummaryGrid";
import { RosterLearnerCard } from "./RosterLearnerCard";
import { StatePanel } from "./StatePanel";
import { defaultDecisionDraft } from "./defaultDecisionDraft";
import { enrollmentStatusOptions } from "./enrollmentStatusOptions";
import { statusLabel } from "./statusLabel";
import { type ActionState } from "./ActionState";
import { type DecisionDraft } from "./DecisionDraft";
import { type EnrollmentStatusFilter } from "./EnrollmentStatusFilter";

export function EnrollmentWorkspaceView({
  actionMessage,
  actionState,
  confirmRemovalUserId,
  decisionDrafts,
  onDecisionDraftChange,
  onRefresh,
  onRemoveLearner,
  onStatusFilterChange,
  onSubmitDecision,
  statusFilter,
  workspace,
}: {
  actionMessage: string | null;
  actionState: ActionState;
  confirmRemovalUserId: number | null;
  decisionDrafts: Record<number, DecisionDraft>;
  onDecisionDraftChange: (requestId: number, draft: DecisionDraft) => void;
  onRefresh: () => void;
  onRemoveLearner: (learner: TeacherCourseRosterLearner) => void;
  onStatusFilterChange: (status: EnrollmentStatusFilter) => void;
  onSubmitDecision: (request: TeacherCourseJoinRequestItem, event: FormEvent<HTMLFormElement>) => void;
  statusFilter: EnrollmentStatusFilter;
  workspace: TeacherCourseEnrollmentWorkspaceResponse;
}) {
  const openRequestCount =
    workspace.course.roster.pending_join_request_count + workspace.course.roster.waitlisted_join_request_count;
  return (
    <>
      <section className={styles.workspaceHero}>
        <Link className={styles.secondaryLink} href={`/teach/courses/${workspace.course.id}`}>
          <ArrowLeft size={17} aria-hidden />
          Course workspace
        </Link>
        <div className={styles.workspaceTitleBlock}>
          <p className={styles.eyebrow}>{statusLabel(workspace.course.lifecycle_status)}</p>
          <h2>Enrollment queue</h2>
          <p className={styles.muted}>
            Review join requests, waitlist learners, approve access, and remove roster access from a course-scoped workspace.
          </p>
        </div>
      </section>

      <EnrollmentWorkspaceSummaryGrid openRequestCount={openRequestCount} workspace={workspace} />

      {actionMessage ? (
        <section className={`${styles.warningPanel} ${styles.singlePanel}`} role="status">
          <div className={styles.panelHeader}>
            <AlertCircle size={18} aria-hidden />
            <h2>Enrollment update</h2>
          </div>
          <p>{actionMessage}</p>
        </section>
      ) : null}

      <section className={styles.filterPanel} aria-label="Enrollment request filters">
        <label>
          <span>Request status</span>
          <select
            onChange={(event) => onStatusFilterChange(event.target.value as EnrollmentStatusFilter)}
            value={statusFilter}
          >
            {enrollmentStatusOptions.map((status) => (
              <option key={status} value={status}>
                {statusLabel(status)}
              </option>
            ))}
          </select>
        </label>
        <button className={styles.secondaryButton} type="button" onClick={onRefresh}>
          <RefreshCw size={17} aria-hidden />
          Refresh
        </button>
      </section>

      {!workspace.progress_supported || !workspace.reward_eligibility_supported ? (
        <section className={`${styles.warningPanel} ${styles.singlePanel}`}>
          <div className={styles.panelHeader}>
            <AlertCircle size={18} aria-hidden />
            <h2>Roster signals</h2>
          </div>
          <p>
            {workspace.progress_supported
              ? "Persisted progress is available from the student progress route; reward eligibility still needs its final route contract."
              : "Persisted progress and reward eligibility are not available in this enrollment route yet."}
          </p>
        </section>
      ) : null}

      <section className={styles.courseSection}>
        <div className={styles.sectionHeader}>
          <div>
            <h2>Join requests</h2>
            <p className={styles.muted}>Showing {statusLabel(workspace.join_requests.status || "all")} requests with learner context.</p>
          </div>
          <span className={`${styles.statusPill} ${workspace.join_requests.total ? styles.warn : styles.neutral}`}>
            {workspace.join_requests.total} total
          </span>
        </div>
        {workspace.join_requests.requests.length ? (
          <div className={styles.enrollmentList}>
            {workspace.join_requests.requests.map((request) => (
              <EnrollmentRequestCard
                actionState={actionState}
                draft={decisionDrafts[request.id] || defaultDecisionDraft}
                key={request.id}
                onDraftChange={(draft) => onDecisionDraftChange(request.id, draft)}
                onSubmit={(event) => onSubmitDecision(request, event)}
                request={request}
              />
            ))}
          </div>
        ) : (
          <StatePanel
            detail="No learner join requests match this filter."
            icon={<CheckCircle2 size={22} aria-hidden />}
            title="No matching requests"
          />
        )}
      </section>

      <section className={styles.courseSection}>
        <div className={styles.sectionHeader}>
          <div>
            <h2>Roster</h2>
            <p className={styles.muted}>Learners with course access. Removal uses a two-step confirmation.</p>
          </div>
          <span className={`${styles.statusPill} ${workspace.roster.total ? styles.good : styles.neutral}`}>
            {workspace.roster.total} enrolled
          </span>
        </div>
        {workspace.roster.learners.length ? (
          <div className={styles.enrollmentList}>
            {workspace.roster.learners.map((learner) => (
              <RosterLearnerCard
                actionState={actionState}
                confirmRemoval={confirmRemovalUserId === learner.user.id}
                key={learner.user.id}
                learner={learner}
                onRemove={() => onRemoveLearner(learner)}
              />
            ))}
          </div>
        ) : (
          <StatePanel
            detail="No enrolled learners are visible for this course."
            icon={<Users size={22} aria-hidden />}
            title="Roster is empty"
          />
        )}
      </section>
    </>
  );
}

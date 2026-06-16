"use client";

import { ClipboardCheck, RefreshCw } from "lucide-react";
import { type CourseAssessmentSnapshot } from "../api/courseAssessmentApi";
import { type AssessmentDraft } from "../model/assessmentState";
import { AssessmentCard } from "../components/AssessmentCard";
import { EmptyState } from "../../components/EmptyState";
import styles from "../../learner-workspace.module.css";

export function CourseAssessmentView({
  drafts,
  error,
  loadState,
  notice,
  onRetry,
  onSubmitAssessment,
  onUpdateAnswer,
  snapshot,
  submittingAssessmentId,
  submissionsEnabled,
}: {
  drafts: Record<number, AssessmentDraft>;
  error: string | null;
  loadState: "idle" | "loading" | "success" | "error";
  notice: string | null;
  snapshot: CourseAssessmentSnapshot | null;
  submittingAssessmentId: number | null;
  submissionsEnabled: boolean;
  onRetry: () => void;
  onSubmitAssessment: (assessmentId: number) => void;
  onUpdateAnswer: (assessmentId: number, questionId: number, value: string) => void;
}) {
  if (loadState === "idle") return null;

  return (
    <section className={`${styles.section} ${styles.assessmentSection}`}>
      <div className={styles.sectionHeader}>
        <h2>Assessments</h2>
        <ClipboardCheck size={20} aria-hidden />
      </div>
      {!submissionsEnabled ? (
        <p className={styles.assessmentNotice}>Preview mode shows assessments without allowing attempts.</p>
      ) : null}
      {notice ? <p className={styles.assessmentNotice}>{notice}</p> : null}
      {loadState === "loading" ? <p className={styles.statePanel}>Loading assessments...</p> : null}
      {loadState === "error" ? (
        <section className={styles.errorPanel}>
          <h3>Assessments unavailable</h3>
          <p>{error}</p>
          <button className={styles.secondaryButton} type="button" onClick={onRetry}>
            <RefreshCw size={18} aria-hidden />
            Retry
          </button>
        </section>
      ) : null}
      {loadState === "success" && snapshot && !snapshot.assessments.length ? (
        <EmptyState detail="This course has no published assessments yet." title="No assessments" />
      ) : null}
      {loadState === "success" && snapshot?.assessments.length ? (
        <div className={styles.assessmentGrid}>
          {snapshot.assessments.map((assessment) => (
            <AssessmentCard
              assessment={assessment}
              attempts={snapshot.attemptsByAssessmentId[assessment.id] || []}
              draft={drafts[assessment.id] || {}}
              isSubmitting={submittingAssessmentId === assessment.id}
              key={assessment.id}
              submissionsEnabled={submissionsEnabled}
              onSubmit={() => onSubmitAssessment(assessment.id)}
              onUpdateAnswer={(questionId, value) => onUpdateAnswer(assessment.id, questionId, value)}
            />
          ))}
        </div>
      ) : null}
    </section>
  );
}

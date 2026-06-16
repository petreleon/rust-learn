"use client";

import { Send } from "lucide-react";
import { type AssessmentAttempt, type AssessmentItem } from "@/lib/learner";
import {
  assessmentStatusLabel,
  assessmentStatusTone,
  canSubmitAssessment,
  remainingAttempts,
  type AssessmentDraft,
} from "../model/assessmentState";
import { AttemptHistory } from "./AttemptHistory";
import { QuestionField } from "./QuestionField";
import { StatusPill } from "../../components/StatusPill";
import styles from "../../learner-workspace.module.css";

export function AssessmentCard({
  assessment,
  attempts,
  draft,
  isSubmitting,
  onSubmit,
  onUpdateAnswer,
  submissionsEnabled,
}: {
  assessment: AssessmentItem;
  attempts: AssessmentAttempt[];
  draft: AssessmentDraft;
  isSubmitting: boolean;
  submissionsEnabled: boolean;
  onSubmit: () => void;
  onUpdateAnswer: (questionId: number, value: string) => void;
}) {
  const remaining = remainingAttempts(assessment, attempts);
  const canSubmit = canSubmitAssessment({ assessment, attempts, draft, submissionsEnabled });
  const answersDisabled = !submissionsEnabled || remaining === 0 || isSubmitting;

  return (
    <article className={styles.assessmentCard}>
      <div className={styles.itemHeader}>
        <div>
          <h3>{assessment.title}</h3>
          {assessment.description ? <p className={styles.muted}>{assessment.description}</p> : null}
        </div>
        <StatusPill label={assessmentStatusLabel(assessment, attempts)} tone={assessmentStatusTone(assessment, attempts)} />
      </div>
      <div className={styles.metaRow}>
        <span>{assessment.passing_score}% passing</span>
        <span>{remaining} attempts left</span>
        <span>{assessment.questions.length} questions</span>
      </div>
      <AttemptHistory attempts={attempts} />
      <form className={styles.questionList} onSubmit={(event) => {
        event.preventDefault();
        if (canSubmit) onSubmit();
      }}>
        {assessment.questions.map((question) => (
          <QuestionField
            disabled={answersDisabled}
            key={question.id}
            question={question}
            value={draft[question.id] || ""}
            onChange={(value) => onUpdateAnswer(question.id, value)}
          />
        ))}
        {!assessment.questions.length ? (
          <p className={styles.assessmentNotice}>Questions are not ready for this assessment.</p>
        ) : null}
        <button className={styles.primaryLink} disabled={!canSubmit || isSubmitting} type="submit">
          <Send size={18} aria-hidden />
          {isSubmitting ? "Submitting" : "Submit attempt"}
        </button>
      </form>
    </article>
  );
}

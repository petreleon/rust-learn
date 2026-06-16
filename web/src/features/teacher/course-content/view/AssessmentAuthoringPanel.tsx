"use client";

import { ClipboardCheck, Pencil, Plus, Send, X } from "lucide-react";
import { type TeacherAssessment } from "@/lib/teacher";
import styles from "@/features/teacher/shared/teacher-routes.module.css";
import { type ActionState } from "@/features/teacher/shared/route-kit/ActionState";
import { defaultAssessmentQuestionDraft } from "../model/AssessmentDraft";
import { type useAssessmentAuthoringActions } from "../route/useAssessmentAuthoringActions";
import { AssessmentQuestionEditor } from "./AssessmentQuestionEditor";

type AssessmentActions = ReturnType<typeof useAssessmentAuthoringActions>;

export function AssessmentAuthoringPanel({
  actionState,
  assessmentActions,
  canManageContent,
}: {
  actionState: ActionState;
  assessmentActions: AssessmentActions;
  canManageContent: boolean;
}) {
  const disabled = !canManageContent || actionState === "saving";
  const draft = assessmentActions.assessmentDraft;
  const isEditing = assessmentActions.editingAssessmentId !== null;
  return (
    <section className={styles.courseSection}>
      <div className={styles.sectionHeader}>
        <div>
          <h2>Assessment authoring</h2>
          <p className={styles.muted}>Draft, publish, edit, and preview course assessments with answer keys.</p>
        </div>
      </div>
      <section className={styles.twoColumn}>
        <form className={styles.authoringForm} onSubmit={assessmentActions.submitAssessment}>
          <div className={styles.panelHeader}>
            <ClipboardCheck size={20} aria-hidden />
            <h2>{isEditing ? "Edit assessment" : "Create assessment"}</h2>
          </div>
          <label>
            <span>Title</span>
            <input
              disabled={disabled}
              maxLength={255}
              onChange={(event) => assessmentActions.setAssessmentDraft({ ...draft, title: event.target.value })}
              value={draft.title}
            />
          </label>
          <label>
            <span>Description</span>
            <textarea
              disabled={disabled}
              onChange={(event) => assessmentActions.setAssessmentDraft({ ...draft, description: event.target.value })}
              rows={3}
              value={draft.description}
            />
          </label>
          <div className={styles.actionRow}>
            <label>
              <span>Passing %</span>
              <input
                disabled={disabled}
                max="100"
                min="0"
                onChange={(event) => assessmentActions.setAssessmentDraft({ ...draft, passingScore: event.target.value })}
                type="number"
                value={draft.passingScore}
              />
            </label>
            <label>
              <span>Attempts</span>
              <input
                disabled={disabled}
                min="1"
                onChange={(event) => assessmentActions.setAssessmentDraft({ ...draft, maxAttempts: event.target.value })}
                type="number"
                value={draft.maxAttempts}
              />
            </label>
          </div>
          {draft.questions.map((question, index) => (
            <AssessmentQuestionEditor
              canRemove={draft.questions.length > 1}
              disabled={disabled}
              index={index}
              key={index}
              onChange={(next) => assessmentActions.setAssessmentDraft({
                ...draft,
                questions: draft.questions.map((item, itemIndex) => (itemIndex === index ? next : item)),
              })}
              onRemove={() => assessmentActions.setAssessmentDraft({
                ...draft,
                questions: draft.questions.filter((_, itemIndex) => itemIndex !== index),
              })}
              question={question}
            />
          ))}
          <button
            className={styles.secondaryButton}
            disabled={disabled}
            onClick={() => assessmentActions.setAssessmentDraft({
              ...draft,
              questions: [...draft.questions, defaultAssessmentQuestionDraft()],
            })}
            type="button"
          >
            <Plus size={17} aria-hidden />
            Add question
          </button>
          <label className={styles.permissionRow}>
            <input
              checked={draft.published}
              disabled={disabled}
              onChange={(event) => assessmentActions.setAssessmentDraft({ ...draft, published: event.target.checked })}
              type="checkbox"
            />
            <span>Published for learners</span>
          </label>
          <div className={styles.actionRow}>
            <button className={styles.primaryButton} disabled={disabled} type="submit">
              <Send size={17} aria-hidden />
              {isEditing ? "Update assessment" : "Create assessment"}
            </button>
            {isEditing ? (
              <button className={styles.secondaryButton} disabled={actionState === "saving"} onClick={assessmentActions.cancelAssessmentEdit} type="button">
                <X size={17} aria-hidden />
                Cancel
              </button>
            ) : null}
          </div>
        </form>
        <AssessmentPreview assessments={assessmentActions.assessments} onEdit={assessmentActions.editAssessment} />
      </section>
    </section>
  );
}

function AssessmentPreview({
  assessments,
  onEdit,
}: {
  assessments: TeacherAssessment[];
  onEdit: (assessment: TeacherAssessment) => void;
}) {
  if (!assessments.length) {
    return <p className={styles.muted}>No assessments yet.</p>;
  }
  return (
    <div className={styles.contentList}>
      {assessments.map((assessment) => (
        <article className={styles.contentRow} key={assessment.id}>
          <div>
            <strong>{assessment.title}</strong>
            <p className={styles.muted}>
              {assessment.published ? "Published" : "Draft"} - {assessment.passing_score}% pass - {assessment.questions.length} question
              {assessment.questions.length === 1 ? "" : "s"}
            </p>
            {assessment.questions[0] ? <p className={styles.muted}>Answer key: {assessment.questions[0].correct_answer || "Not set"}</p> : null}
          </div>
          <button className={styles.secondaryButton} onClick={() => onEdit(assessment)} type="button">
            <Pencil size={16} aria-hidden />
            Edit
          </button>
        </article>
      ))}
    </div>
  );
}

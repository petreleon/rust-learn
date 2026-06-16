"use client";

import { X } from "lucide-react";
import styles from "@/features/teacher/shared/teacher-routes.module.css";
import { type AssessmentQuestionDraft } from "../model/AssessmentDraft";

export function AssessmentQuestionEditor({
  canRemove,
  disabled,
  index,
  onChange,
  onRemove,
  question,
}: {
  canRemove: boolean;
  disabled: boolean;
  index: number;
  onChange: (question: AssessmentQuestionDraft) => void;
  onRemove: () => void;
  question: AssessmentQuestionDraft;
}) {
  const labelIndex = index + 1;
  return (
    <fieldset className={styles.filterPanel}>
      <div className={styles.panelHeader}>
        <h3>Question {labelIndex}</h3>
        {canRemove ? (
          <button className={styles.secondaryButton} disabled={disabled} onClick={onRemove} type="button">
            <X size={16} aria-hidden />
            Remove
          </button>
        ) : null}
      </div>
      <label>
        <span>Question {labelIndex}</span>
        <textarea
          disabled={disabled}
          onChange={(event) => onChange({ ...question, text: event.target.value })}
          rows={3}
          value={question.text}
        />
      </label>
      <label>
        <span>Options {labelIndex}</span>
        <input
          disabled={disabled}
          onChange={(event) => onChange({ ...question, optionsText: event.target.value })}
          placeholder="Ownership, Borrowing, Lifetimes"
          value={question.optionsText}
        />
      </label>
      <label>
        <span>Correct answer {labelIndex}</span>
        <input
          disabled={disabled}
          onChange={(event) => onChange({ ...question, correctAnswer: event.target.value })}
          value={question.correctAnswer}
        />
      </label>
      <label>
        <span>Points {labelIndex}</span>
        <input
          disabled={disabled}
          min="1"
          onChange={(event) => onChange({ ...question, points: event.target.value })}
          type="number"
          value={question.points}
        />
      </label>
    </fieldset>
  );
}

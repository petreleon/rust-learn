"use client";

import { type AssessmentQuestionItem } from "@/lib/learner";
import { questionOptions } from "../model/assessmentState";
import styles from "../../learner-workspace.module.css";

export function QuestionField({
  disabled,
  onChange,
  question,
  value,
}: {
  disabled: boolean;
  question: AssessmentQuestionItem;
  value: string;
  onChange: (value: string) => void;
}) {
  const options = questionOptions(question);
  const fieldName = `assessment-question-${question.id}`;

  return (
    <fieldset className={styles.questionField}>
      <legend>
        {question.order + 1}. {question.text}
      </legend>
      {options.length ? (
        <div className={styles.radioStack}>
          {options.map((option) => (
            <label key={option}>
              <input
                checked={value === option}
                disabled={disabled}
                name={fieldName}
                type="radio"
                value={option}
                onChange={() => onChange(option)}
              />
              {option}
            </label>
          ))}
        </div>
      ) : (
        <input
          className={styles.assessmentInput}
          disabled={disabled}
          name={fieldName}
          type="text"
          value={value}
          onChange={(event) => onChange(event.target.value)}
        />
      )}
    </fieldset>
  );
}

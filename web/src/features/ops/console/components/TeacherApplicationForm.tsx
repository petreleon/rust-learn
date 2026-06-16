"use client";

import { Send } from "lucide-react";
import styles from "../ops-console.module.css";
import { PROTECTED_ACTIONS } from "../model/PROTECTED_ACTIONS";
import { optionLabel } from "../model/optionLabel";
import { positiveIntegerInputProps } from "../model/positiveIntegerInputProps";
import { type OpsConsoleController } from "../route/useOpsConsoleController";

export function TeacherApplicationForm({ controller }: { controller: OpsConsoleController }) {
  const { auth, teacher } = controller;

  return (
    <div className={styles.formGrid}>
      <label className={styles.fieldLabel}>
        Scope
        <select
          value={teacher.teacherForm.requested_scope}
          onChange={(event) => teacher.setTeacherForm((current) => ({ ...current, requested_scope: event.target.value }))}
        >
          <option value="platform">{optionLabel("platform")}</option>
          <option value="organization">{optionLabel("organization")}</option>
          <option value="course">{optionLabel("course")}</option>
        </select>
      </label>
      {teacher.teacherForm.requested_scope === "organization" && (
        <label className={styles.fieldLabel}>
          Organization id
          <input
            {...positiveIntegerInputProps}
            value={teacher.teacherForm.requested_organization_id}
            onChange={(event) => teacher.setTeacherForm((current) => ({ ...current, requested_organization_id: event.target.value }))}
          />
        </label>
      )}
      {teacher.teacherForm.requested_scope === "course" && (
        <label className={styles.fieldLabel}>
          Course id
          <input
            {...positiveIntegerInputProps}
            value={teacher.teacherForm.requested_course_id}
            onChange={(event) => teacher.setTeacherForm((current) => ({ ...current, requested_course_id: event.target.value }))}
          />
        </label>
      )}
      <label className={styles.fieldLabel}>
        Sponsor org id
        <input
          {...positiveIntegerInputProps}
          value={teacher.teacherForm.organization_sponsor_id}
          onChange={(event) => teacher.setTeacherForm((current) => ({ ...current, organization_sponsor_id: event.target.value }))}
        />
      </label>
      <label className={`${styles.fieldLabel} ${styles.fullWidth}`}>
        Experience summary
        <textarea
          rows={3}
          value={teacher.teacherForm.experience_summary}
          onChange={(event) => teacher.setTeacherForm((current) => ({ ...current, experience_summary: event.target.value }))}
        />
      </label>
      <label className={`${styles.fieldLabel} ${styles.fullWidth}`}>
        Portfolio links
        <textarea
          rows={2}
          value={teacher.teacherForm.portfolio_links}
          onChange={(event) => teacher.setTeacherForm((current) => ({ ...current, portfolio_links: event.target.value }))}
        />
      </label>
      <button
        type="button"
        className={styles.primaryButton}
        onClick={teacher.submitTeacherApplication}
        {...auth.actionState(
          teacher.canSubmitTeacherApplicationForm,
          true,
          "Required permission",
          PROTECTED_ACTIONS.submitTeacherApplication,
        )}
      >
        <Send size={17} aria-hidden />
        <span>Submit</span>
      </button>
    </div>
  );
}

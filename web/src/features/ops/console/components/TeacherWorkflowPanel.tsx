"use client";

import { CheckCircle2, ClipboardList, GraduationCap } from "lucide-react";
import styles from "../ops-console.module.css";
import { PROTECTED_ACTIONS } from "../model/PROTECTED_ACTIONS";
import { optionLabel } from "../model/optionLabel";
import { positiveIntegerInputProps } from "../model/positiveIntegerInputProps";
import { teacherApplicationStatuses } from "../model/teacherApplicationStatuses";
import { OpsServerDeniedNotice } from "./OpsServerDeniedNotice";
import { PermissionNotice } from "./PermissionNotice";
import { RequirementNotice } from "./RequirementNotice";
import { TeacherApplicationForm } from "./TeacherApplicationForm";
import { type OpsConsoleController } from "../route/useOpsConsoleController";

export function TeacherWorkflowPanel({ controller }: { controller: OpsConsoleController }) {
  const { auth, teacher } = controller;

  return (
    <section id="teacher-workflow" className={styles.panel} aria-labelledby="teacher-title">
      <div className={styles.panelHeader}>
        <div>
          <p className={styles.eyebrow}>Teacher applications</p>
          <h2 id="teacher-title">Application and review</h2>
        </div>
        <GraduationCap size={22} aria-hidden />
      </div>
      {!teacher.canUseTeacherWorkflow && (
        <PermissionNotice
          title="Teacher permissions disabled"
          detail="Enable application or review permissions to show teacher workflow actions."
        />
      )}
      {teacher.canTeacherApply && (
        <>
          {auth.hasSessionToken && teacher.hasTeacherApplicationDraft && (
            <RequirementNotice action="Submit application" fields={teacher.teacherApplicationMissingFields} />
          )}
          <OpsServerDeniedNotice
            auth={auth}
            actionKey={PROTECTED_ACTIONS.submitTeacherApplication}
            action="Submit application"
          />
          <TeacherApplicationForm controller={controller} />
        </>
      )}
      {teacher.canReviewTeachers && (
        <>
          <OpsServerDeniedNotice auth={auth} actionKey={PROTECTED_ACTIONS.teacherApplicationQueue} action="Load queue" />
          <div className={styles.actionStrip}>
            <select
              aria-label="Teacher application status filter"
              value={teacher.teacherStatus}
              onChange={(event) => teacher.setTeacherStatus(event.target.value)}
            >
              {teacherApplicationStatuses.map((status) => (
                <option key={status} value={status}>
                  {optionLabel(status)}
                </option>
              ))}
            </select>
            <button
              type="button"
              className={styles.secondaryButton}
              onClick={teacher.loadTeacherApplications}
              {...auth.actionState(true, true, "Required permission", PROTECTED_ACTIONS.teacherApplicationQueue)}
            >
              <ClipboardList size={17} aria-hidden />
              <span>Load queue</span>
            </button>
          </div>
        </>
      )}
      {teacher.canDecideTeachers && (
        <>
          {auth.hasSessionToken && teacher.hasTeacherDecisionDraft && (
            <RequirementNotice action="Decide" fields={teacher.teacherDecisionMissingFields} />
          )}
          <OpsServerDeniedNotice auth={auth} actionKey={PROTECTED_ACTIONS.teacherApplicationDecision} action="Decide" />
          <div className={styles.actionStrip}>
            <input
              aria-label="Teacher application id"
              {...positiveIntegerInputProps}
              placeholder="Application id"
              value={teacher.teacherDecision.application_id}
              onChange={(event) => teacher.setTeacherDecision((current) => ({ ...current, application_id: event.target.value }))}
            />
            <select
              aria-label="Teacher decision status"
              value={teacher.activeTeacherDecisionStatus}
              onChange={(event) => teacher.setTeacherDecision((current) => ({ ...current, status: event.target.value }))}
            >
              {teacher.teacherDecisionStatuses.map((status) => (
                <option key={status} value={status}>
                  {optionLabel(status)}
                </option>
              ))}
            </select>
            <input
              aria-label="Teacher decision reason"
              placeholder="Reason"
              value={teacher.teacherDecision.decision_reason}
              onChange={(event) => teacher.setTeacherDecision((current) => ({ ...current, decision_reason: event.target.value }))}
            />
            <button
              type="button"
              className={styles.secondaryButton}
              onClick={teacher.decideTeacherApplication}
              {...auth.actionState(
                teacher.canDecideTeacherApplicationForm,
                true,
                "Required permission",
                PROTECTED_ACTIONS.teacherApplicationDecision,
              )}
            >
              <CheckCircle2 size={17} aria-hidden />
              <span>Decide</span>
            </button>
          </div>
        </>
      )}
    </section>
  );
}

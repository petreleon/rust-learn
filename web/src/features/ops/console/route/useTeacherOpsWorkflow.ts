"use client";

import { useState } from "react";
import { DEFAULT_TEACHER_APPLICATION_SCOPE } from "../model/DEFAULT_TEACHER_APPLICATION_SCOPE";
import { DEFAULT_TEACHER_DECISION_STATUS } from "../model/DEFAULT_TEACHER_DECISION_STATUS";
import { PROTECTED_ACTIONS } from "../model/PROTECTED_ACTIONS";
import { buildQuery } from "../model/buildQuery";
import { hasAnyText } from "../model/hasAnyText";
import { hasPositiveInteger } from "../model/hasPositiveInteger";
import { hasText } from "../model/hasText";
import { missingFields } from "../model/missingFields";
import { optionalPositiveInteger } from "../model/optionalPositiveInteger";
import { splitLinks } from "../model/splitLinks";
import { type HasOpsPermission, type SendOpsApi } from "../model/opsConsoleTypes";

export function useTeacherOpsWorkflow({
  hasPermission,
  sendApi,
}: {
  hasPermission: HasOpsPermission;
  sendApi: SendOpsApi;
}) {
  const [teacherForm, setTeacherForm] = useState({
    experience_summary: "",
    organization_sponsor_id: "",
    portfolio_links: "",
    requested_course_id: "",
    requested_organization_id: "",
    requested_scope: DEFAULT_TEACHER_APPLICATION_SCOPE,
  });
  const [teacherStatus, setTeacherStatus] = useState("submitted");
  const [teacherDecision, setTeacherDecision] = useState({
    application_id: "",
    decision_reason: "",
    status: DEFAULT_TEACHER_DECISION_STATUS,
  });
  const canTeacherApply = hasPermission("SUBMIT_TEACHER_APPLICATION");
  const canReviewTeachers = hasPermission("REVIEW_TEACHER_APPLICATIONS");
  const canApproveTeachers = hasPermission("APPROVE_TEACHER_APPLICATION");
  const canRejectTeachers = hasPermission("REJECT_TEACHER_APPLICATION");
  const teacherDecisionStatuses = [
    ...(canApproveTeachers ? ["approved"] : []),
    ...(canReviewTeachers ? ["needs_changes"] : []),
    ...(canRejectTeachers ? ["rejected"] : []),
  ];
  const activeTeacherDecisionStatus = teacherDecisionStatuses.includes(teacherDecision.status)
    ? teacherDecision.status
    : teacherDecisionStatuses[0] || teacherDecision.status;
  const canDecideTeachers = teacherDecisionStatuses.length > 0;
  const canUseTeacherWorkflow = canTeacherApply || canReviewTeachers || canDecideTeachers;
  const hasTeacherApplicationDraft =
    teacherForm.requested_scope !== DEFAULT_TEACHER_APPLICATION_SCOPE ||
    hasAnyText([
      teacherForm.requested_organization_id,
      teacherForm.requested_course_id,
      teacherForm.experience_summary,
      teacherForm.organization_sponsor_id,
      teacherForm.portfolio_links,
    ]);
  const hasTeacherDecisionDraft =
    hasAnyText([teacherDecision.application_id, teacherDecision.decision_reason]) ||
    teacherDecision.status !== DEFAULT_TEACHER_DECISION_STATUS;
  const canSubmitTeacherApplicationForm =
    hasText(teacherForm.experience_summary) &&
    (teacherForm.requested_scope === "platform" ||
      (teacherForm.requested_scope === "organization" &&
        (hasPositiveInteger(teacherForm.requested_organization_id) ||
          hasPositiveInteger(teacherForm.organization_sponsor_id))) ||
      (teacherForm.requested_scope === "course" && hasPositiveInteger(teacherForm.requested_course_id)));
  const canDecideTeacherApplicationForm = canDecideTeachers && hasPositiveInteger(teacherDecision.application_id);
  const teacherApplicationMissingFields = missingFields([
    ["Experience summary", hasText(teacherForm.experience_summary)],
    [
      "Organization id or sponsor org id",
      teacherForm.requested_scope !== "organization" ||
        hasPositiveInteger(teacherForm.requested_organization_id) ||
        hasPositiveInteger(teacherForm.organization_sponsor_id),
    ],
    ["Course id", teacherForm.requested_scope !== "course" || hasPositiveInteger(teacherForm.requested_course_id)],
  ]);
  const teacherDecisionMissingFields = missingFields([
    ["Application id", hasPositiveInteger(teacherDecision.application_id)],
  ]);
  const visibleActions = [
    ...(canTeacherApply ? [PROTECTED_ACTIONS.submitTeacherApplication] : []),
    ...(canReviewTeachers ? [PROTECTED_ACTIONS.teacherApplicationQueue] : []),
    ...(canDecideTeachers ? [PROTECTED_ACTIONS.teacherApplicationDecision] : []),
  ];

  function submitTeacherApplication() {
    const requestedScope = teacherForm.requested_scope;
    void sendApi(PROTECTED_ACTIONS.submitTeacherApplication, "/teacher-applications", "POST", {
      experience_summary: teacherForm.experience_summary,
      organization_sponsor_id: optionalPositiveInteger(teacherForm.organization_sponsor_id),
      portfolio_links: splitLinks(teacherForm.portfolio_links),
      requested_course_id:
        requestedScope === "course" ? optionalPositiveInteger(teacherForm.requested_course_id) : undefined,
      requested_organization_id:
        requestedScope === "organization" ? optionalPositiveInteger(teacherForm.requested_organization_id) : undefined,
      requested_scope: requestedScope,
    });
  }

  function loadTeacherApplications() {
    void sendApi(PROTECTED_ACTIONS.teacherApplicationQueue, `/teacher-applications${buildQuery({ limit: 25, status: teacherStatus })}`);
  }

  function decideTeacherApplication() {
    void sendApi(
      PROTECTED_ACTIONS.teacherApplicationDecision,
      `/teacher-applications/${teacherDecision.application_id}/decision`,
      "PUT",
      { decision_reason: teacherDecision.decision_reason || undefined, status: activeTeacherDecisionStatus },
    );
  }

  return {
    activeTeacherDecisionStatus,
    canDecideTeacherApplicationForm,
    canDecideTeachers,
    canReviewTeachers,
    canSubmitTeacherApplicationForm,
    canTeacherApply,
    canUseTeacherWorkflow,
    decideTeacherApplication,
    hasTeacherApplicationDraft,
    hasTeacherDecisionDraft,
    loadTeacherApplications,
    setTeacherDecision,
    setTeacherForm,
    setTeacherStatus,
    submitTeacherApplication,
    teacherApplicationMissingFields,
    teacherDecision,
    teacherDecisionMissingFields,
    teacherDecisionStatuses,
    teacherForm,
    teacherStatus,
    visibleActions,
  };
}

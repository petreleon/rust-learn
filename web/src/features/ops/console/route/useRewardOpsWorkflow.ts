"use client";

import { useState } from "react";
import { DEFAULT_AMOUNT_DECISION_STATUS } from "../model/DEFAULT_AMOUNT_DECISION_STATUS";
import { DEFAULT_AMOUNT_VALUE } from "../model/DEFAULT_AMOUNT_VALUE";
import { DEFAULT_REWARD_STATUS } from "../model/DEFAULT_REWARD_STATUS";
import { DEFAULT_TEACHER_REWARD_DECISION_STATUS } from "../model/DEFAULT_TEACHER_REWARD_DECISION_STATUS";
import { PROTECTED_ACTIONS } from "../model/PROTECTED_ACTIONS";
import { buildQuery } from "../model/buildQuery";
import { hasAnyText } from "../model/hasAnyText";
import { hasNonNegativeNumber } from "../model/hasNonNegativeNumber";
import { hasPositiveInteger } from "../model/hasPositiveInteger";
import { hasText } from "../model/hasText";
import { missingFields } from "../model/missingFields";
import { optionalNumber } from "../model/optionalNumber";
import { optionalPositiveInteger } from "../model/optionalPositiveInteger";
import { type HasOpsPermission, type SendOpsApi } from "../model/opsConsoleTypes";

export function useRewardOpsWorkflow({
  hasPermission,
  sendApi,
}: {
  hasPermission: HasOpsPermission;
  sendApi: SendOpsApi;
}) {
  const [rewardCourseId, setRewardCourseId] = useState("");
  const [rewardCandidateId, setRewardCandidateId] = useState("");
  const [rewardStudentId, setRewardStudentId] = useState("");
  const [rewardStatus, setRewardStatus] = useState(DEFAULT_REWARD_STATUS);
  const [historyStatus, setHistoryStatus] = useState("");
  const [teacherRewardDecision, setTeacherRewardDecision] = useState({
    decision_reason: "",
    status: DEFAULT_TEACHER_REWARD_DECISION_STATUS,
  });
  const [amountDecision, setAmountDecision] = useState({
    approved_amount: DEFAULT_AMOUNT_VALUE,
    decision_reason: "",
    status: DEFAULT_AMOUNT_DECISION_STATUS,
  });
  const canSubmitReward = hasPermission("SUBMIT_COURSE_REWARD_EVENT");
  const canViewCourseRewards = hasPermission("VIEW_COURSE_REWARD_STATUS");
  const canTeacherApproveReward = hasPermission("APPROVE_STUDENT_REWARD_CANDIDATE");
  const canApproveAmount = hasPermission("APPROVE_REWARD_AMOUNT");
  const canUseRewardWorkflow = canSubmitReward || canViewCourseRewards || canTeacherApproveReward || canApproveAmount;
  const showRewardCourseId = canSubmitReward || canViewCourseRewards || canTeacherApproveReward;
  const showRewardCandidateId = canTeacherApproveReward || canApproveAmount;
  const showRewardStatusFilter = canViewCourseRewards;
  const showRewardSharedFields = showRewardCourseId || showRewardCandidateId || showRewardStatusFilter;
  const canSubmitRewardCandidateForm = hasPositiveInteger(rewardCourseId) && hasPositiveInteger(rewardStudentId);
  const canLoadRewardCandidatesForm = hasPositiveInteger(rewardCourseId);
  const canDecideStudentRewardForm = hasPositiveInteger(rewardCourseId) && hasPositiveInteger(rewardCandidateId);
  const canDecideRewardAmountForm =
    hasPositiveInteger(rewardCandidateId) &&
    (amountDecision.status !== "approved" || hasNonNegativeNumber(amountDecision.approved_amount));
  const hasSubmitRewardCandidateDraft = hasAnyText([rewardCourseId, rewardStudentId]);
  const hasLoadRewardCandidatesDraft = hasText(rewardCourseId) || rewardStatus !== DEFAULT_REWARD_STATUS;
  const hasTeacherRewardDecisionDraft =
    hasAnyText([rewardCourseId, rewardCandidateId, teacherRewardDecision.decision_reason]) ||
    teacherRewardDecision.status !== DEFAULT_TEACHER_REWARD_DECISION_STATUS;
  const hasAmountDecisionDraft =
    hasText(rewardCandidateId) ||
    amountDecision.approved_amount !== DEFAULT_AMOUNT_VALUE ||
    hasText(amountDecision.decision_reason) ||
    amountDecision.status !== DEFAULT_AMOUNT_DECISION_STATUS;
  const submitRewardCandidateMissingFields = missingFields([
    ["Course id", hasPositiveInteger(rewardCourseId)],
    ["Student user id", hasPositiveInteger(rewardStudentId)],
  ]);
  const loadRewardCandidatesMissingFields = missingFields([["Course id", hasPositiveInteger(rewardCourseId)]]);
  const teacherRewardDecisionMissingFields = missingFields([
    ["Course id", hasPositiveInteger(rewardCourseId)],
    ["Candidate id", hasPositiveInteger(rewardCandidateId)],
  ]);
  const amountDecisionMissingFields = missingFields([
    ["Candidate id", hasPositiveInteger(rewardCandidateId)],
    ["Approved amount", amountDecision.status !== "approved" || hasNonNegativeNumber(amountDecision.approved_amount)],
  ]);
  const visibleActions = [
    ...(canSubmitReward ? [PROTECTED_ACTIONS.submitRewardCandidate] : []),
    ...(canViewCourseRewards ? [PROTECTED_ACTIONS.courseRewardCandidates, PROTECTED_ACTIONS.studentRewardHistory] : []),
    ...(canTeacherApproveReward ? [PROTECTED_ACTIONS.courseRewardDecision] : []),
    ...(canApproveAmount ? [PROTECTED_ACTIONS.rewardAmountDecision] : []),
  ];

  function submitRewardCandidate() {
    void sendApi(PROTECTED_ACTIONS.submitRewardCandidate, `/courses/${rewardCourseId}/reward-candidates`, "POST", {
      event_type: "course_completion",
      evidence: { completion_percentage: 100 },
      student_user_id: optionalPositiveInteger(rewardStudentId),
    });
  }

  function loadRewardCandidates() {
    void sendApi(PROTECTED_ACTIONS.courseRewardCandidates, `/courses/${rewardCourseId}/reward-candidates${buildQuery({ limit: 25, status: rewardStatus })}`);
  }

  function decideStudentReward() {
    void sendApi(
      PROTECTED_ACTIONS.courseRewardDecision,
      `/courses/${rewardCourseId}/reward-candidates/${rewardCandidateId}/teacher-decision`,
      "PUT",
      { decision_reason: teacherRewardDecision.decision_reason || undefined, status: teacherRewardDecision.status },
    );
  }

  function decideRewardAmount() {
    void sendApi(PROTECTED_ACTIONS.rewardAmountDecision, `/reward-candidates/${rewardCandidateId}/amount-decision`, "PUT", {
      approved_amount: amountDecision.status === "approved" ? optionalNumber(amountDecision.approved_amount) : undefined,
      decision_reason: amountDecision.decision_reason || undefined,
      status: amountDecision.status,
    });
  }

  function loadStudentHistory() {
    void sendApi(PROTECTED_ACTIONS.studentRewardHistory, `/reward-candidates/me/history${buildQuery({ limit: 25, status: historyStatus })}`);
  }

  return {
    amountDecision,
    amountDecisionMissingFields,
    canApproveAmount,
    canDecideRewardAmountForm,
    canDecideStudentRewardForm,
    canLoadRewardCandidatesForm,
    canSubmitReward,
    canSubmitRewardCandidateForm,
    canTeacherApproveReward,
    canUseRewardWorkflow,
    canViewCourseRewards,
    decideRewardAmount,
    decideStudentReward,
    hasAmountDecisionDraft,
    hasLoadRewardCandidatesDraft,
    hasSubmitRewardCandidateDraft,
    hasTeacherRewardDecisionDraft,
    historyStatus,
    loadRewardCandidates,
    loadRewardCandidatesMissingFields,
    loadStudentHistory,
    rewardCandidateId,
    rewardCourseId,
    rewardStatus,
    rewardStudentId,
    setAmountDecision,
    setHistoryStatus,
    setRewardCandidateId,
    setRewardCourseId,
    setRewardStatus,
    setRewardStudentId,
    setTeacherRewardDecision,
    showRewardCandidateId,
    showRewardCourseId,
    showRewardSharedFields,
    showRewardStatusFilter,
    submitRewardCandidate,
    submitRewardCandidateMissingFields,
    teacherRewardDecision,
    teacherRewardDecisionMissingFields,
    visibleActions,
  };
}

"use client";

import { useState } from "react";
import { DEFAULT_FRAUD_BLOCK_SCOPE } from "../model/DEFAULT_FRAUD_BLOCK_SCOPE";
import { PROTECTED_ACTIONS } from "../model/PROTECTED_ACTIONS";
import { fraudBlockScopes } from "../model/fraudBlockScopes";
import { hasAnyText } from "../model/hasAnyText";
import { hasPositiveInteger } from "../model/hasPositiveInteger";
import { hasText } from "../model/hasText";
import { missingFields } from "../model/missingFields";
import { optionalPositiveInteger } from "../model/optionalPositiveInteger";
import {
  type HasOpsPermission,
  type ScopePermission,
  type SendOpsApi,
} from "../model/opsConsoleTypes";

export function useFraudOpsWorkflow({
  hasPermission,
  sendApi,
}: {
  hasPermission: HasOpsPermission;
  sendApi: SendOpsApi;
}) {
  const [fraudBlock, setFraudBlock] = useState({
    course_id: "",
    evidence_reference: "",
    organization_id: "",
    reason: "",
    reward_policy_id: "",
    scope_type: DEFAULT_FRAUD_BLOCK_SCOPE,
    teacher_user_id: "",
  });
  const [fraudBlockId, setFraudBlockId] = useState("");
  const canManageFraudBlocks = hasPermission("MANAGE_REWARD_FRAUD_BLOCKS");
  const canBlockTeacherRewards = hasPermission("BLOCK_REWARD_TEACHER") || canManageFraudBlocks;
  const canBlockOrganizationRewards = hasPermission("BLOCK_REWARD_ORGANIZATION") || canManageFraudBlocks;
  const canViewFraud = hasPermission("VIEW_REWARD_AUDIT") || canManageFraudBlocks;
  const canManageFraud = canManageFraudBlocks || canBlockTeacherRewards || canBlockOrganizationRewards;
  const canUseFraudWorkflow = canViewFraud || canManageFraud;
  const fraudBlockScopePermissions: Record<string, ScopePermission> = {
    course: {
      allowed: canManageFraudBlocks,
      detail: "Course blocks require manage fraud blocks permission.",
      permissionTitle: "Manage fraud blocks permission required",
    },
    organization: {
      allowed: canBlockOrganizationRewards,
      detail: "Organization blocks require block org rewards or manage fraud blocks permission.",
      permissionTitle: "Block org rewards or manage fraud blocks permission required",
    },
    reward_policy: {
      allowed: canManageFraudBlocks,
      detail: "Reward policy blocks require manage fraud blocks permission.",
      permissionTitle: "Manage fraud blocks permission required",
    },
    teacher: {
      allowed: canBlockTeacherRewards,
      detail: "Teacher blocks require block teacher rewards or manage fraud blocks permission.",
      permissionTitle: "Block teacher rewards or manage fraud blocks permission required",
    },
  };
  const selectedFraudBlockScopePermission = fraudBlockScopePermissions[fraudBlock.scope_type] ?? unsupportedScope;
  const firstAllowedFraudBlockScope = fraudBlockScopes.find((scope) => fraudBlockScopePermissions[scope]?.allowed);
  const activeFraudBlockScope =
    selectedFraudBlockScopePermission.allowed || !firstAllowedFraudBlockScope
      ? fraudBlock.scope_type
      : firstAllowedFraudBlockScope;
  const activeFraudBlockScopePermission = fraudBlockScopePermissions[activeFraudBlockScope] ?? unsupportedScope;
  const canCreateFraudBlockTarget =
    (activeFraudBlockScope === "teacher" && hasPositiveInteger(fraudBlock.teacher_user_id)) ||
    (activeFraudBlockScope === "organization" && hasPositiveInteger(fraudBlock.organization_id)) ||
    (activeFraudBlockScope === "course" && hasPositiveInteger(fraudBlock.course_id)) ||
    (activeFraudBlockScope === "reward_policy" && hasPositiveInteger(fraudBlock.reward_policy_id));
  const canCreateFraudBlockFields = canCreateFraudBlockTarget && hasText(fraudBlock.reason);
  const canCreateSelectedFraudScope = activeFraudBlockScopePermission.allowed;
  const canUseFraudBlockForm = hasPositiveInteger(fraudBlockId);
  const canUseFraudBlockActions = canViewFraud || canManageFraud;
  const fraudBlockActionLabel =
    canViewFraud && canManageFraud ? "Audit or revoke block" : canViewFraud ? "Audit block" : "Revoke block";
  const hasFraudBlockCreateDraft =
    fraudBlock.scope_type !== DEFAULT_FRAUD_BLOCK_SCOPE ||
    hasAnyText([
      fraudBlock.teacher_user_id,
      fraudBlock.organization_id,
      fraudBlock.course_id,
      fraudBlock.reward_policy_id,
      fraudBlock.reason,
      fraudBlock.evidence_reference,
    ]);
  const fraudBlockCreateMissingFields = missingFields([
    ["Teacher user id", activeFraudBlockScope !== "teacher" || hasPositiveInteger(fraudBlock.teacher_user_id)],
    ["Organization id", activeFraudBlockScope !== "organization" || hasPositiveInteger(fraudBlock.organization_id)],
    ["Course id", activeFraudBlockScope !== "course" || hasPositiveInteger(fraudBlock.course_id)],
    ["Policy id", activeFraudBlockScope !== "reward_policy" || hasPositiveInteger(fraudBlock.reward_policy_id)],
    ["Reason", hasText(fraudBlock.reason)],
  ]);
  const fraudBlockUseMissingFields = missingFields([["Block id", hasPositiveInteger(fraudBlockId)]]);
  const visibleActions = [
    ...(canManageFraud ? [PROTECTED_ACTIONS.createRewardFraudBlock, PROTECTED_ACTIONS.revokeRewardFraudBlock] : []),
    ...(canViewFraud ? [PROTECTED_ACTIONS.rewardFraudBlocks, PROTECTED_ACTIONS.rewardFraudAudit] : []),
  ];

  function createFraudBlock() {
    void sendApi(PROTECTED_ACTIONS.createRewardFraudBlock, "/reward-fraud-blocks", "POST", {
      course_id: activeFraudBlockScope === "course" ? optionalPositiveInteger(fraudBlock.course_id) : undefined,
      evidence_reference: fraudBlock.evidence_reference || undefined,
      organization_id:
        activeFraudBlockScope === "organization" ? optionalPositiveInteger(fraudBlock.organization_id) : undefined,
      reason: fraudBlock.reason,
      reward_policy_id:
        activeFraudBlockScope === "reward_policy" ? optionalPositiveInteger(fraudBlock.reward_policy_id) : undefined,
      scope_type: activeFraudBlockScope,
      teacher_user_id:
        activeFraudBlockScope === "teacher" ? optionalPositiveInteger(fraudBlock.teacher_user_id) : undefined,
    });
  }

  function listFraudBlocks() {
    void sendApi(PROTECTED_ACTIONS.rewardFraudBlocks, "/reward-fraud-blocks?active=true&limit=25");
  }

  function revokeFraudBlock() {
    void sendApi(PROTECTED_ACTIONS.revokeRewardFraudBlock, `/reward-fraud-blocks/${fraudBlockId}/revoke`, "PUT");
  }

  function loadFraudAudit() {
    void sendApi(PROTECTED_ACTIONS.rewardFraudAudit, `/reward-fraud-blocks/${fraudBlockId}/audit`);
  }

  return {
    activeFraudBlockScope,
    activeFraudBlockScopePermission,
    canCreateFraudBlockFields,
    canCreateSelectedFraudScope,
    canManageFraud,
    canUseFraudBlockActions,
    canUseFraudBlockForm,
    canUseFraudWorkflow,
    canViewFraud,
    createFraudBlock,
    fraudBlock,
    fraudBlockActionLabel,
    fraudBlockCreateMissingFields,
    fraudBlockId,
    fraudBlockScopePermissions,
    fraudBlockUseMissingFields,
    hasFraudBlockCreateDraft,
    hasFraudBlockUseDraft: hasText(fraudBlockId),
    listFraudBlocks,
    loadFraudAudit,
    revokeFraudBlock,
    selectedFraudBlockScopePermission,
    setFraudBlock,
    setFraudBlockId,
    visibleActions,
  };
}

const unsupportedScope: ScopePermission = {
  allowed: false,
  detail: "Selected fraud block scope is not supported.",
  permissionTitle: "Supported fraud block scope required",
};

"use client";

import { useState } from "react";
import { DEFAULT_DELEGATED_PERMISSION } from "../model/DEFAULT_DELEGATED_PERMISSION";
import { DEFAULT_DELEGATION_SCOPE } from "../model/DEFAULT_DELEGATION_SCOPE";
import { PROTECTED_ACTIONS } from "../model/PROTECTED_ACTIONS";
import { delegatedPermissionOptions } from "../model/delegatedPermissionOptions";
import { hasAnyText } from "../model/hasAnyText";
import { hasPositiveInteger } from "../model/hasPositiveInteger";
import { missingFields } from "../model/missingFields";
import { optionalPositiveInteger } from "../model/optionalPositiveInteger";
import { optionalUtcDateTime } from "../model/optionalUtcDateTime";
import { type HasOpsPermission, type SendOpsApi } from "../model/opsConsoleTypes";

export function useDelegationOpsWorkflow({
  hasPermission,
  sendApi,
}: {
  hasPermission: HasOpsPermission;
  sendApi: SendOpsApi;
}) {
  const [delegation, setDelegation] = useState({
    course_id: "",
    expires_at: "",
    grantee_user_id: "",
    organization_id: "",
    permission: DEFAULT_DELEGATED_PERMISSION,
    reason: "",
    scope_type: DEFAULT_DELEGATION_SCOPE,
  });
  const [delegationId, setDelegationId] = useState("");
  const [revokeReason, setRevokeReason] = useState("");
  const canDelegate = hasPermission("DELEGATE_REWARD_APPROVAL");
  const scopedDelegatedPermissions = delegatedPermissionOptions
    .filter((permission) => permission.scopes.includes(delegation.scope_type))
    .map((permission) => permission.key);
  const activeDelegatedPermission = scopedDelegatedPermissions.includes(delegation.permission)
    ? delegation.permission
    : scopedDelegatedPermissions[0] || delegation.permission;
  const canGrantDelegationForm =
    hasPositiveInteger(delegation.grantee_user_id) &&
    scopedDelegatedPermissions.length > 0 &&
    (delegation.scope_type === "platform" ||
      (delegation.scope_type === "organization" && hasPositiveInteger(delegation.organization_id)) ||
      (delegation.scope_type === "course" && hasPositiveInteger(delegation.course_id)));
  const canRevokeDelegationForm = hasPositiveInteger(delegationId);
  const hasDelegationGrantDraft =
    delegation.scope_type !== DEFAULT_DELEGATION_SCOPE ||
    activeDelegatedPermission !== DEFAULT_DELEGATED_PERMISSION ||
    hasAnyText([
      delegation.grantee_user_id,
      delegation.organization_id,
      delegation.course_id,
      delegation.reason,
      delegation.expires_at,
    ]);
  const hasDelegationRevokeDraft = hasAnyText([delegationId, revokeReason]);
  const delegationGrantMissingFields = missingFields([
    ["Grantee user id", hasPositiveInteger(delegation.grantee_user_id)],
    ["Organization id", delegation.scope_type !== "organization" || hasPositiveInteger(delegation.organization_id)],
    ["Course id", delegation.scope_type !== "course" || hasPositiveInteger(delegation.course_id)],
  ]);
  const delegationRevokeMissingFields = missingFields([["Delegation id", hasPositiveInteger(delegationId)]]);
  const visibleActions = canDelegate
    ? [
        PROTECTED_ACTIONS.grantDelegatedPermission,
        PROTECTED_ACTIONS.delegatedPermissions,
        PROTECTED_ACTIONS.revokeDelegatedPermission,
      ]
    : [];

  function grantDelegation() {
    void sendApi(PROTECTED_ACTIONS.grantDelegatedPermission, "/delegated-permissions", "POST", {
      course_id: delegation.scope_type === "course" ? optionalPositiveInteger(delegation.course_id) : undefined,
      expires_at: optionalUtcDateTime(delegation.expires_at),
      grantee_user_id: optionalPositiveInteger(delegation.grantee_user_id),
      organization_id:
        delegation.scope_type === "organization" ? optionalPositiveInteger(delegation.organization_id) : undefined,
      permission: activeDelegatedPermission,
      reason: delegation.reason || undefined,
      scope_type: delegation.scope_type,
    });
  }

  function listDelegations() {
    void sendApi(PROTECTED_ACTIONS.delegatedPermissions, "/delegated-permissions?active=true&limit=25");
  }

  function revokeDelegation() {
    void sendApi(PROTECTED_ACTIONS.revokeDelegatedPermission, `/delegated-permissions/${delegationId}/revoke`, "PUT", {
      revoke_reason: revokeReason || undefined,
    });
  }

  return {
    activeDelegatedPermission,
    canDelegate,
    canGrantDelegationForm,
    canRevokeDelegationForm,
    delegation,
    delegationGrantMissingFields,
    delegationId,
    delegationRevokeMissingFields,
    grantDelegation,
    hasDelegationGrantDraft,
    hasDelegationRevokeDraft,
    listDelegations,
    revokeDelegation,
    revokeReason,
    scopedDelegatedPermissions,
    setDelegation,
    setDelegationId,
    setRevokeReason,
    visibleActions,
  };
}

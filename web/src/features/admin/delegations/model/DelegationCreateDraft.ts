export type DelegationCreateDraft = {
  courseId: string;
  expiresAt: string;
  granteeUserId: string;
  organizationId: string;
  permission: string;
  reason: string;
  scopeType: string;
};

export const defaultDelegationCreateDraft: DelegationCreateDraft = {
  courseId: "",
  expiresAt: "",
  granteeUserId: "",
  organizationId: "",
  permission: "",
  reason: "",
  scopeType: "",
};

export function canSubmitDelegationCreate(draft: DelegationCreateDraft, state: string) {
  return (
    state !== "submitting" &&
    draft.scopeType.trim().length > 0 &&
    draft.permission.trim().length > 0 &&
    draft.granteeUserId.trim().length > 0
  );
}

export type FraudBlockCreateDraft = {
  evidence: string;
  reason: string;
  scopeType: string;
  targetId: string;
};

export const defaultFraudBlockCreateDraft: FraudBlockCreateDraft = {
  evidence: "",
  reason: "",
  scopeType: "",
  targetId: "",
};

export function canSubmitFraudBlockCreate(draft: FraudBlockCreateDraft, state: string) {
  return (
    state !== "submitting" &&
    draft.scopeType.trim().length > 0 &&
    draft.targetId.trim().length > 0 &&
    draft.reason.trim().length > 0
  );
}

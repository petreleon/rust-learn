export type TermsDraft = {
  completionRewardAmount: string;
  maxEnrolledStudents: string;
  note: string;
};

export const defaultTermsDraft: TermsDraft = {
  completionRewardAmount: "",
  maxEnrolledStudents: "",
  note: "",
};

export function termsPayload(draft: TermsDraft) {
  return {
    completion_reward_amount: draft.completionRewardAmount.trim(),
    max_enrolled_students: Number.parseInt(draft.maxEnrolledStudents.trim(), 10),
    note: optionalText(draft.note),
  };
}

export function termsDecisionPayload(note: string) {
  return { note: optionalText(note) };
}

export function validateTermsDraft(draft: TermsDraft): string | null {
  if (!draft.completionRewardAmount.trim()) return "Completion reward is required.";
  if (!/^\d+(\.\d+)?$/.test(draft.completionRewardAmount.trim())) {
    return "Completion reward must be a non-negative number.";
  }
  if (!/^[1-9]\d*$/.test(draft.maxEnrolledStudents.trim())) {
    return "Max students must be a positive whole number.";
  }
  return null;
}

function optionalText(value: string) {
  const trimmed = value.trim();
  return trimmed ? trimmed : null;
}

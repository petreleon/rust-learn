"use client";
export function isSafeEvidenceKey(key: string) {
  return !/(^id$|_id$|user_id|student|learner|candidate)/i.test(key);
}

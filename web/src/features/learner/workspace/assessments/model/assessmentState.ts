import { type AssessmentAttempt, type AssessmentItem, type AssessmentQuestionItem } from "@/lib/learner";

export type AssessmentDraft = Record<number, string>;

export function questionOptions(question: AssessmentQuestionItem): string[] {
  if (!Array.isArray(question.options)) return [];

  return question.options
    .filter((option) => ["boolean", "number", "string"].includes(typeof option))
    .map((option) => String(option))
    .map((option) => option.trim())
    .filter(Boolean);
}

export function completedAttempts(attempts: AssessmentAttempt[]): AssessmentAttempt[] {
  return attempts.filter((attempt) => attempt.completed_at);
}

export function latestAttempt(attempts: AssessmentAttempt[]): AssessmentAttempt | null {
  return completedAttempts(attempts)[0] || null;
}

export function remainingAttempts(assessment: AssessmentItem, attempts: AssessmentAttempt[]) {
  return Math.max(assessment.max_attempts - completedAttempts(attempts).length, 0);
}

export function assessmentStatusLabel(assessment: AssessmentItem, attempts: AssessmentAttempt[]) {
  const latest = latestAttempt(attempts);
  if (latest?.passed === true) return "Passed";
  if (remainingAttempts(assessment, attempts) === 0) return "Max attempts";
  if (latest?.passed === false) return "Retry available";
  return "Not started";
}

export function assessmentStatusTone(assessment: AssessmentItem, attempts: AssessmentAttempt[]) {
  const latest = latestAttempt(attempts);
  if (latest?.passed === true) return "good";
  if (remainingAttempts(assessment, attempts) === 0) return "bad";
  if (latest?.passed === false) return "warn";
  return "neutral";
}

export function emptyAssessmentDraft(assessment: AssessmentItem): AssessmentDraft {
  return Object.fromEntries(assessment.questions.map((question) => [question.id, ""]));
}

export function draftIsComplete(assessment: AssessmentItem, draft: AssessmentDraft) {
  return assessment.questions.length > 0 && assessment.questions.every((question) => draft[question.id]?.trim());
}

export function canSubmitAssessment({
  assessment,
  attempts,
  draft,
  submissionsEnabled,
}: {
  assessment: AssessmentItem;
  attempts: AssessmentAttempt[];
  draft: AssessmentDraft;
  submissionsEnabled: boolean;
}) {
  return submissionsEnabled && remainingAttempts(assessment, attempts) > 0 && draftIsComplete(assessment, draft);
}

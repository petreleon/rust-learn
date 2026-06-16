import {
  fetchAssessmentAttempts,
  fetchCourseAssessments,
  submitAssessmentAttempt,
  type AssessmentAttempt,
  type AssessmentAttemptResult,
  type AssessmentItem,
} from "@/lib/learner";

export type { AssessmentAttemptResult } from "@/lib/learner";

export type CourseAssessmentSnapshot = {
  assessments: AssessmentItem[];
  attemptsByAssessmentId: Record<number, AssessmentAttempt[]>;
};

export async function loadCourseAssessmentSnapshot({
  courseId,
  token,
}: {
  courseId: number;
  token: string;
}): Promise<CourseAssessmentSnapshot> {
  const assessments = await fetchCourseAssessments({ courseId, token });
  const attemptEntries = await Promise.all(
    assessments.map(async (assessment) => [
      assessment.id,
      await fetchAssessmentAttempts({ assessmentId: assessment.id, courseId, token }),
    ]),
  );

  return {
    assessments,
    attemptsByAssessmentId: Object.fromEntries(attemptEntries),
  };
}

export async function loadCourseAssessmentList({
  courseId,
  token,
}: {
  courseId: number;
  token: string;
}): Promise<AssessmentItem[]> {
  return fetchCourseAssessments({ courseId, token });
}

export async function submitCourseAssessmentAttempt({
  answers,
  assessmentId,
  courseId,
  token,
}: {
  answers: Record<number, string>;
  assessmentId: number;
  courseId: number;
  token: string;
}): Promise<AssessmentAttemptResult> {
  return submitAssessmentAttempt({ answers, assessmentId, courseId, token });
}

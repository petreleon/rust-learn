"use client";

import { useCourseAssessments } from "./useCourseAssessments";
import { CourseAssessmentView } from "../view/CourseAssessmentView";

export function CourseAssessmentPanel({
  courseId,
  submissionsEnabled,
}: {
  courseId: number;
  submissionsEnabled: boolean;
}) {
  const route = useCourseAssessments(courseId);

  return (
    <CourseAssessmentView
      drafts={route.drafts}
      error={route.error}
      loadState={route.loadState}
      notice={route.notice}
      snapshot={route.snapshot}
      submittingAssessmentId={route.submittingAssessmentId}
      submissionsEnabled={submissionsEnabled}
      onRetry={route.loadAssessments}
      onSubmitAssessment={route.submitAssessment}
      onUpdateAnswer={route.updateAnswer}
    />
  );
}

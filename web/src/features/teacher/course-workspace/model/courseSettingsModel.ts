import { type TeacherCourseDashboardItem } from "@/lib/teacher/TeacherCourseDashboardItem";
import { type UpdateTeacherCoursePayload } from "@/lib/teacher/UpdateTeacherCoursePayload";

export type CourseSettingsDraft = {
  description: string;
  prerequisites: string;
  title: string;
  topics: string;
};

export function courseSettingsDraft(course: TeacherCourseDashboardItem): CourseSettingsDraft {
  return {
    description: course.description || "",
    prerequisites: course.prerequisites || "",
    title: course.title,
    topics: course.topics || "",
  };
}

export function courseSettingsPayload(draft: CourseSettingsDraft): UpdateTeacherCoursePayload {
  return {
    description: nullableText(draft.description),
    prerequisites: nullableText(draft.prerequisites),
    title: draft.title.trim(),
    topics: nullableText(draft.topics),
  };
}

export function courseSettingsValidation(draft: CourseSettingsDraft): string | null {
  return draft.title.trim() ? null : "Course title is required.";
}

function nullableText(value: string): string | null {
  const trimmed = value.trim();
  return trimmed ? trimmed : null;
}

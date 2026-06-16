import { type TeacherCourseEnrollmentWorkspaceResponse } from "@/lib/teacher/TeacherCourseEnrollmentWorkspaceResponse";

export function courseEnrollmentTitle(workspace: TeacherCourseEnrollmentWorkspaceResponse | null) {
  return workspace?.course.title || "Course enrollments";
}

export function joinRequestCountLabel(workspace: TeacherCourseEnrollmentWorkspaceResponse) {
  const total = workspace.join_requests.total;
  return `${total} request${total === 1 ? "" : "s"}`;
}

export function rosterCountLabel(workspace: TeacherCourseEnrollmentWorkspaceResponse) {
  const total = workspace.roster.total;
  return `${total} learner${total === 1 ? "" : "s"}`;
}

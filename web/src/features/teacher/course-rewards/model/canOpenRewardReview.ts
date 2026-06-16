import { type TeacherCourseStudentsResponse } from "@/lib/teacher/TeacherCourseStudentsResponse";

export function canOpenRewardReview(students: TeacherCourseStudentsResponse) {
  return (
    students.course.permissions.can_view_reward_candidates ||
    students.course.permissions.can_approve_reward_candidates
  );
}

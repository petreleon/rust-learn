import { type CurrentSession } from "@/lib/session/CurrentSession";
import { fetchCurrentSession } from "@/lib/session/fetchCurrentSession";
import { decideTeacherRewardCandidate } from "@/lib/teacher/decideTeacherRewardCandidate";
import { fetchTeacherRewardCandidates } from "@/lib/teacher/fetchTeacherRewardCandidates";
import { fetchTeachingCourseStudents } from "@/lib/teacher/fetchTeachingCourseStudents";
import { type DecideTeacherRewardCandidatePayload } from "@/lib/teacher/DecideTeacherRewardCandidatePayload";
import { type TeacherCourseStudentsResponse } from "@/lib/teacher/TeacherCourseStudentsResponse";
import { type TeacherRewardCandidate } from "@/lib/teacher/TeacherRewardCandidate";
import { type TeacherRewardCandidateStatusFilter } from "@/lib/teacher/TeacherRewardCandidateStatusFilter";

export type CourseRewardContext = {
  session: CurrentSession;
  students: TeacherCourseStudentsResponse;
};

export async function loadCourseRewardContext({
  courseId,
  token,
}: {
  courseId: string;
  token: string;
}): Promise<CourseRewardContext> {
  const session = await fetchCurrentSession({ token });
  const students = await fetchTeachingCourseStudents({ courseId, token });

  return { session, students };
}

export function loadCourseRewardCandidates({
  courseId,
  status,
  token,
}: {
  courseId: string;
  status: TeacherRewardCandidateStatusFilter;
  token: string;
}): Promise<TeacherRewardCandidate[]> {
  return fetchTeacherRewardCandidates({ courseId, status, token });
}

export function decideCourseRewardCandidate({
  candidateId,
  courseId,
  payload,
  token,
}: {
  candidateId: number;
  courseId: string;
  payload: DecideTeacherRewardCandidatePayload;
  token: string;
}) {
  return decideTeacherRewardCandidate({ candidateId, courseId, payload, token });
}

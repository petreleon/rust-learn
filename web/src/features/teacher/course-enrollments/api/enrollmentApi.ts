import { fetchCurrentSession } from "@/lib/session/fetchCurrentSession";
import { decideTeacherJoinRequest } from "@/lib/teacher/decideTeacherJoinRequest";
import { fetchTeachingCourseEnrollments } from "@/lib/teacher/fetchTeachingCourseEnrollments";
import { removeTeacherEnrollment } from "@/lib/teacher/removeTeacherEnrollment";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type DecideTeacherJoinRequestPayload } from "@/lib/teacher/DecideTeacherJoinRequestPayload";
import { type TeacherCourseEnrollmentWorkspaceResponse } from "@/lib/teacher/TeacherCourseEnrollmentWorkspaceResponse";

export type EnrollmentRouteLoad = {
  session: CurrentSession;
  workspace: TeacherCourseEnrollmentWorkspaceResponse;
};

export async function loadTeacherCourseEnrollments({
  courseId,
  status,
  token,
}: {
  courseId: string;
  status: string;
  token: string;
}): Promise<EnrollmentRouteLoad> {
  const [session, workspace] = await Promise.all([
    fetchCurrentSession({ token }),
    fetchTeachingCourseEnrollments({ courseId, status, token }),
  ]);

  return { session, workspace };
}

export async function decideEnrollmentRequest({
  courseId,
  payload,
  requestId,
  token,
}: {
  courseId: string;
  payload: DecideTeacherJoinRequestPayload;
  requestId: number;
  token: string;
}) {
  return decideTeacherJoinRequest({ courseId, payload, requestId, token });
}

export async function removeEnrollmentLearner({
  courseId,
  token,
  userId,
}: {
  courseId: string;
  token: string;
  userId: number;
}) {
  return removeTeacherEnrollment({ courseId, token, userId });
}

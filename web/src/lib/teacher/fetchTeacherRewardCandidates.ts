import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "./teacherJsonRequest";
import { type TeacherRewardCandidate } from "./TeacherRewardCandidate";
import { type TeacherRewardCandidateListOptions } from "./TeacherRewardCandidateListOptions";

export async function fetchTeacherRewardCandidates({
  apiRoot = "/api",
  courseId,
  limit = 25,
  offset,
  status,
  studentUserId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: TeacherRewardCandidateListOptions): Promise<TeacherRewardCandidate[]> {
  const query = new URLSearchParams();
  query.set("limit", String(limit));
  if (typeof offset === "number") {
    query.set("offset", String(offset));
  }
  const trimmedStatus = status?.trim();
  if (trimmedStatus && trimmedStatus !== "all") {
    query.set("status", trimmedStatus);
  }
  if (typeof studentUserId === "number") {
    query.set("student_user_id", String(studentUserId));
  }

  const suffix = query.toString();
  return teacherJsonRequest<TeacherRewardCandidate[]>({
    method: "GET",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/reward-candidates${suffix ? `?${suffix}` : ""}`,
  });
}

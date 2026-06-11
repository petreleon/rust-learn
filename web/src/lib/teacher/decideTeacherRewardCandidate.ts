import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "./teacherJsonRequest";
import { type DecideTeacherRewardCandidateOptions } from "./DecideTeacherRewardCandidateOptions";
import { type TeacherRewardCandidate } from "./TeacherRewardCandidate";

export async function decideTeacherRewardCandidate({
  apiRoot = "/api",
  candidateId,
  courseId,
  payload,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: DecideTeacherRewardCandidateOptions): Promise<TeacherRewardCandidate> {
  return teacherJsonRequest<TeacherRewardCandidate>({
    body: JSON.stringify(payload),
    method: "PUT",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/reward-candidates/${candidateId}/teacher-decision`,
  });
}

import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "./teacherJsonRequest";
import { type SubmitTeacherApplicationOptions } from "./SubmitTeacherApplicationOptions";
import { type TeacherApplication } from "./TeacherApplication";

export async function submitTeacherApplication({
  apiRoot = "/api",
  payload,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: SubmitTeacherApplicationOptions): Promise<TeacherApplication> {
  return teacherJsonRequest<TeacherApplication>({
    body: JSON.stringify(payload),
    method: "POST",
    timeoutMs,
    token,
    url: `${apiRoot}/teacher-applications`,
  });
}

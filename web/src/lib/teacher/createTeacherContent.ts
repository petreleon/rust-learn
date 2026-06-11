import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "./teacherJsonRequest";
import { type CreateTeacherContentOptions } from "./CreateTeacherContentOptions";
import { type TeacherContent } from "./TeacherContent";

export async function createTeacherContent({
  apiRoot = "/api",
  chapterId,
  courseId,
  payload,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: CreateTeacherContentOptions): Promise<TeacherContent> {
  return teacherJsonRequest<TeacherContent>({
    body: JSON.stringify(payload),
    method: "POST",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/chapters/${chapterId}/contents`,
  });
}

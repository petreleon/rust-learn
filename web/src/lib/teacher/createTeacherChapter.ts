import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "./teacherJsonRequest";
import { type CreateTeacherChapterOptions } from "./CreateTeacherChapterOptions";
import { type TeacherChapter } from "./TeacherChapter";

export async function createTeacherChapter({
  apiRoot = "/api",
  courseId,
  payload,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: CreateTeacherChapterOptions): Promise<TeacherChapter> {
  return teacherJsonRequest<TeacherChapter>({
    body: JSON.stringify(payload),
    method: "POST",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/chapters`,
  });
}

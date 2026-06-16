import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "./teacherJsonRequest";
import { type TeacherContent } from "./TeacherContent";
import { type UpdateTeacherContentOptions } from "./UpdateTeacherContentOptions";

export async function updateTeacherContent({
  apiRoot = "/api",
  chapterId,
  contentId,
  courseId,
  payload,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: UpdateTeacherContentOptions): Promise<TeacherContent> {
  return teacherJsonRequest<TeacherContent>({
    body: JSON.stringify(payload),
    method: "PUT",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/chapters/${chapterId}/contents/${contentId}`,
  });
}

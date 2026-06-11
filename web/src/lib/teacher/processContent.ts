import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "./teacherJsonRequest";
import { type TeacherRequestOptions } from "./TeacherRequestOptions";

export async function processContent({
  apiRoot = "/api",
  chapterId,
  contentId,
  courseId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: TeacherRequestOptions & {
  chapterId: number;
  contentId: number;
  courseId: number;
}): Promise<{ message: string }> {
  return teacherJsonRequest<{ message: string }>({
    method: "POST",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/chapters/${chapterId}/contents/${contentId}/process`,
  });
}

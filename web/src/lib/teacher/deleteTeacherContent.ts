import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherErrorFromResponse } from "./teacherErrorFromResponse";
import { teacherRawRequest } from "./teacherRawRequest";
import { type TeacherRequestOptions } from "./TeacherRequestOptions";

export async function deleteTeacherContent({
  apiRoot = "/api",
  chapterId,
  contentId,
  courseId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: TeacherRequestOptions & {
  chapterId: number | string;
  contentId: number;
  courseId: number | string;
}): Promise<{ message: string }> {
  const response = await teacherRawRequest({
    method: "DELETE",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/chapters/${chapterId}/contents/${contentId}`,
  });

  if (!response.ok) {
    throw await teacherErrorFromResponse(response, "Failed to delete content.");
  }

  return { message: await response.text() };
}

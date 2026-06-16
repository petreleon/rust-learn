import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherErrorFromResponse } from "./teacherErrorFromResponse";
import { teacherRawRequest } from "./teacherRawRequest";
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
  const response = await teacherRawRequest({
    method: "POST",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/chapters/${chapterId}/contents/${contentId}/process`,
  });

  if (!response.ok) {
    throw await teacherErrorFromResponse(response, "Failed to process content.");
  }

  const body = await response.text();
  try {
    const parsed = JSON.parse(body) as { message?: unknown };
    if (typeof parsed.message === "string") {
      return { message: parsed.message };
    }
  } catch {
    return { message: body };
  }

  return { message: body };
}

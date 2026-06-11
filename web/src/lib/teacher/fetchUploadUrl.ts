import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "./teacherJsonRequest";
import { type FetchUploadUrlOptions } from "./FetchUploadUrlOptions";
import { type UploadUrlResponse } from "./UploadUrlResponse";

export async function fetchUploadUrl({
  apiRoot = "/api",
  chapterId,
  contentType,
  courseId,
  filename,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: FetchUploadUrlOptions): Promise<UploadUrlResponse> {
  return teacherJsonRequest<UploadUrlResponse>({
    body: JSON.stringify({ content_type: contentType, filename }),
    method: "POST",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/chapters/${chapterId}/contents/upload_url`,
  });
}

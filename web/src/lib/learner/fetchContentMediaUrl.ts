import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { learnerJsonRequest } from "./learnerJsonRequest";
import { type ContentMediaUrl } from "./ContentMediaUrl";
import { type CourseDetailOptions } from "./CourseDetailOptions";

export async function fetchContentMediaUrl({
  apiRoot = "/api",
  chapterId,
  contentId,
  courseId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: CourseDetailOptions & { chapterId: number; contentId: number }): Promise<ContentMediaUrl> {
  return learnerJsonRequest<ContentMediaUrl>({
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/chapters/${chapterId}/contents/${contentId}/media`,
  });
}

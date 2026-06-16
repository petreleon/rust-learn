import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "./teacherJsonRequest";
import { type FetchTeacherContentProcessingHistoryOptions } from "./FetchTeacherContentProcessingHistoryOptions";
import { type TeacherContentProcessingHistory } from "./TeacherContentProcessingHistory";

export function fetchTeacherContentProcessingHistory({
  apiRoot = "/api",
  chapterId,
  contentId,
  courseId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: FetchTeacherContentProcessingHistoryOptions): Promise<TeacherContentProcessingHistory> {
  return teacherJsonRequest<TeacherContentProcessingHistory>({
    method: "GET",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/${courseId}/chapters/${chapterId}/contents/${contentId}/processing-history`,
  });
}

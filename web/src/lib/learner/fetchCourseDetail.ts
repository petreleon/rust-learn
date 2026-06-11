import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { learnerJsonRequest } from "./learnerJsonRequest";
import { type CourseCatalogDetail } from "./CourseCatalogDetail";
import { type CourseDetailOptions } from "./CourseDetailOptions";

export async function fetchCourseDetail({
  apiRoot = "/api",
  courseId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: CourseDetailOptions): Promise<CourseCatalogDetail> {
  return learnerJsonRequest<CourseCatalogDetail>({
    timeoutMs,
    token,
    url: `${apiRoot}/courses/catalog/${courseId}`,
  });
}

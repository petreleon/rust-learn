import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { teacherJsonRequest } from "./teacherJsonRequest";
import { type TeacherCoursesOptions } from "./TeacherCoursesOptions";
import { type TeacherCoursesResponse } from "./TeacherCoursesResponse";

export async function fetchTeachingCourses({
  apiRoot = "/api",
  lifecycleStatus,
  limit = 25,
  offset,
  search,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: TeacherCoursesOptions): Promise<TeacherCoursesResponse> {
  const query = new URLSearchParams();
  query.set("limit", String(limit));
  if (typeof offset === "number") {
    query.set("offset", String(offset));
  }
  if (lifecycleStatus && lifecycleStatus !== "all") {
    query.set("lifecycle_status", lifecycleStatus);
  }
  const trimmedSearch = search?.trim();
  if (trimmedSearch) {
    query.set("search", trimmedSearch);
  }

  const suffix = query.toString();
  return teacherJsonRequest<TeacherCoursesResponse>({
    method: "GET",
    timeoutMs,
    token,
    url: `${apiRoot}/courses/teaching${suffix ? `?${suffix}` : ""}`,
  });
}

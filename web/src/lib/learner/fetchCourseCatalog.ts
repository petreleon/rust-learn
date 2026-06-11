import { DEFAULT_COURSE_CATALOG_LIMIT } from "./DEFAULT_COURSE_CATALOG_LIMIT";
import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { learnerJsonRequest } from "./learnerJsonRequest";
import { type CourseCatalogOptions } from "./CourseCatalogOptions";
import { type CourseCatalogResponse } from "./CourseCatalogResponse";

export async function fetchCourseCatalog({
  apiRoot = "/api",
  enrollmentStatus,
  limit = DEFAULT_COURSE_CATALOG_LIMIT,
  offset,
  organizationId,
  rewardAvailable,
  search,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: CourseCatalogOptions): Promise<CourseCatalogResponse> {
  const query = new URLSearchParams();
  query.set("limit", String(limit));
  if (typeof offset === "number") {
    query.set("offset", String(offset));
  }
  if (typeof organizationId === "number") {
    query.set("organization_id", String(organizationId));
  }
  if (typeof rewardAvailable === "boolean") {
    query.set("reward_available", String(rewardAvailable));
  }
  const trimmedSearch = search?.trim();
  if (trimmedSearch) {
    query.set("search", trimmedSearch);
  }
  if (enrollmentStatus && enrollmentStatus !== "all") {
    query.set("enrollment_status", enrollmentStatus);
  }

  const suffix = query.toString();
  return learnerJsonRequest<CourseCatalogResponse>({
    timeoutMs,
    token,
    url: `${apiRoot}/courses/catalog${suffix ? `?${suffix}` : ""}`,
  });
}

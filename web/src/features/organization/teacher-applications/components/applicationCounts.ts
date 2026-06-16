import { type OrganizationTeacherApplicationList } from "@/lib/organization/OrganizationTeacherApplicationList";

export function teacherApplicationCounts(applications: OrganizationTeacherApplicationList, page: number) {
  return {
    canGoBack: applications.offset > 0,
    canGoForward: applications.offset + applications.limit < applications.total,
    pageLabel: `Page ${page + 1} of ${Math.max(1, Math.ceil(applications.total / applications.limit))}`,
    rangeLabel:
      applications.total === 0
        ? "0 applications"
        : `${applications.offset + 1}-${Math.min(applications.offset + applications.limit, applications.total)} of ${applications.total}`,
  };
}

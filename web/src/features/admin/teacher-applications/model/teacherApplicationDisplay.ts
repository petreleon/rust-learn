import { type PlatformTeacherApplicationItem } from "@/lib/admin/PlatformTeacherApplicationItem";
import { type TeacherApplicationStatus } from "@/lib/admin/TeacherApplicationStatus";

export const teacherApplicationStatusOptions: Array<{ label: string; value: TeacherApplicationStatus | "" }> = [
  { label: "All statuses", value: "" },
  { label: "Submitted", value: "submitted" },
  { label: "Needs changes", value: "needs_changes" },
  { label: "Approved", value: "approved" },
  { label: "Rejected", value: "rejected" },
];

export function teacherApplicationStatusTone(status: TeacherApplicationStatus): "good" | "neutral" | "warn" {
  if (status === "approved") return "good";
  if (status === "submitted" || status === "needs_changes") return "warn";
  return "neutral";
}

export function isFinalTeacherApplicationStatus(status: TeacherApplicationStatus) {
  return status === "approved" || status === "rejected";
}

export function teacherApplicationScopeTargetLabel(application: PlatformTeacherApplicationItem) {
  if (application.requested_course) return application.requested_course.title;
  if (application.requested_organization) return application.requested_organization.name;
  if (application.sponsor_organization) return application.sponsor_organization.name;
  return "Platform scope";
}

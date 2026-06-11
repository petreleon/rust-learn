"use client";

import { type PlatformTeacherApplicationItem } from "@/lib/admin";

export function scopeTargetLabel(application: PlatformTeacherApplicationItem) {
  if (application.requested_course) {
    return application.requested_course.title;
  }
  if (application.requested_organization) {
    return application.requested_organization.name;
  }
  if (application.sponsor_organization) {
    return application.sponsor_organization.name;
  }
  return "Platform scope";
}

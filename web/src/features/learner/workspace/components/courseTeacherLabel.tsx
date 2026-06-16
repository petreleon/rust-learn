"use client";

import { type CourseCatalogItem } from "@/lib/learner";

export function courseTeacherLabel(course: CourseCatalogItem) {
  return course.teachers.map((teacher) => teacher.name).join(", ") || "Teacher pending";
}

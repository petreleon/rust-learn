"use client";

import { type CourseCatalogItem } from "@/lib/learner";

export function courseContentLabel(course: CourseCatalogItem) {
  return course.content.has_content
    ? `${course.content.chapter_count} chapters, ${course.content.content_count} items`
    : "Content pending";
}

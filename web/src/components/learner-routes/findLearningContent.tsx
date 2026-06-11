"use client";

import { type CourseLearningResponse } from "@/lib/learner";

export function findLearningContent(learning: CourseLearningResponse | null, contentId: number | null) {
  if (!learning || contentId === null) {
    return null;
  }

  return learning.chapters
    .flatMap((chapter) => chapter.contents)
    .find((content) => content.id === contentId) || null;
}

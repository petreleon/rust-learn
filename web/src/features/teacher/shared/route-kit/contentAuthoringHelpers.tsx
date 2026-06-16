"use client";

import {
  type TeacherCourseWorkspaceContent,
  type TeacherCourseWorkspaceResponse,
} from "@/lib/teacher";
import { type ContentDraft } from "./ContentDraft";

export function findContentChapter(
  workspace: TeacherCourseWorkspaceResponse | null,
  content: TeacherCourseWorkspaceContent,
) {
  return workspace?.chapters.find((chapter) => chapter.contents.some((item) => item.id === content.id)) ?? null;
}

export function contentDraftOrder(draft: ContentDraft) {
  const orderText = draft.order.trim();
  return { order: Number(orderText), orderText };
}

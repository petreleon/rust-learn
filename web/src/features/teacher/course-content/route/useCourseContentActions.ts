"use client";

import { type FormEvent, useState } from "react";
import { type TeacherCourseWorkspaceContent } from "@/lib/teacher";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { contentDraftOrder, findContentChapter } from "@/features/teacher/shared/route-kit/contentAuthoringHelpers";
import { defaultContentDraft } from "@/features/teacher/shared/route-kit/defaultContentDraft";
import { isEditableTextContent } from "@/features/teacher/shared/route-kit/isEditableTextContent";
import {
  deleteTeacherCourseContentItem,
  processTeacherCourseContentItem,
} from "../api/courseContentApi";
import {
  isContentDraftDirty,
  type CourseContentActionArgs,
  runContentAction,
  submitTextContent,
  submitUploadContent,
} from "./courseContentActionHelpers";

export function useCourseContentActions(args: CourseContentActionArgs) {
  const [deleteConfirmContentId, setDeleteConfirmContentId] = useState<number | null>(null);
  const [editingContentId, setEditingContentId] = useState<number | null>(null);
  const [uploadProgress, setUploadProgress] = useState<number | null>(null);

  function cancelContentEdit() {
    setEditingContentId(null);
    args.setContentDraft(defaultContentDraft);
    args.setActionMessage(null);
  }

  function editContent(content: TeacherCourseWorkspaceContent) {
    const chapter = findContentChapter(args.workspace, content);
    if (!chapter || !isEditableTextContent(content.content_type)) {
      args.setActionMessage(chapter ? "Only text and article content can be edited here." : "Chapter not found for this content item.");
      return;
    }
    args.setContentDraft({
      chapterId: String(chapter.id),
      contentType: content.content_type,
      data: content.data ?? "",
      file: null,
      filename: "",
      order: String(content.order),
      uploadKind: "text",
    });
    setEditingContentId(content.id);
    setDeleteConfirmContentId(null);
    setUploadProgress(null);
    args.setActionMessage("Editing selected content item.");
  }

  async function deleteContent(content: TeacherCourseWorkspaceContent) {
    const chapter = findContentChapter(args.workspace, content);
    const token = readBrowserSessionToken();
    if (!token || !chapter) {
      args.setActionMessage("Chapter not found for this content item.");
      return;
    }
    if (deleteConfirmContentId !== content.id) {
      setDeleteConfirmContentId(content.id);
      args.setActionMessage("Select delete again to remove this content item.");
      return;
    }
    await runContentAction(args, "Content item deleted.", async () => {
      await deleteTeacherCourseContentItem({ chapterId: chapter.id, contentId: content.id, courseId: args.courseId, token });
      if (editingContentId === content.id) {
        setEditingContentId(null);
        args.setContentDraft(defaultContentDraft);
      }
      setDeleteConfirmContentId(null);
    });
  }

  async function triggerProcessing(content: TeacherCourseWorkspaceContent) {
    const chapter = findContentChapter(args.workspace, content);
    const token = readBrowserSessionToken();
    if (!token || !chapter) {
      args.setActionMessage("Chapter not found for this content item.");
      return;
    }
    await runContentAction(args, "Processing queued. Refresh to check status.", async () => {
      const response = await processTeacherCourseContentItem({ chapterId: chapter.id, contentId: content.id, courseId: args.courseId, token });
      return response.message;
    });
  }

  async function submitContent(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const token = readBrowserSessionToken();
    const chapterId = args.contentDraft.chapterId;
    const { order, orderText } = contentDraftOrder(args.contentDraft);
    if (!token || !chapterId || !orderText || !Number.isFinite(order) || order < 0) {
      args.setActionMessage("Chapter and a non-negative order are required.");
      return;
    }
    if (args.contentDraft.uploadKind === "text") {
      await submitTextContent(args, token, chapterId, order, editingContentId, setEditingContentId);
      return;
    }
    await submitUploadContent(args, token, chapterId, order, setUploadProgress);
  }

  return {
    cancelContentEdit,
    deleteConfirmContentId,
    deleteContent,
    editContent,
    editingContentId,
    isContentDraftDirty: isContentDraftDirty(args.contentDraft, editingContentId),
    submitContent,
    triggerProcessing,
    uploadProgress,
  };
}

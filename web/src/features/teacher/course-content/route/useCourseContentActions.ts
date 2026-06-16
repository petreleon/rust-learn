"use client";

import { type FormEvent, useState } from "react";
import { type TeacherCourseWorkspaceContent } from "@/lib/teacher";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { contentDraftOrder, findContentChapter } from "@/features/teacher/shared/route-kit/contentAuthoringHelpers";
import {
  idleProcessingHistoryState,
  type ContentProcessingHistoryState,
} from "@/features/teacher/shared/route-kit/ContentProcessingHistoryState";
import { defaultContentDraft } from "@/features/teacher/shared/route-kit/defaultContentDraft";
import { isEditableTextContent } from "@/features/teacher/shared/route-kit/isEditableTextContent";
import { normalizeRouteError } from "@/features/teacher/shared/route-kit/normalizeRouteError";
import {
  deleteTeacherCourseContentItem,
  loadTeacherContentProcessingHistory,
  processTeacherCourseContentItem,
} from "../api/courseContentApi";
import {
  isContentDraftDirty,
  type CourseContentActionArgs,
  runContentAction,
  submitTextContent,
  submitUploadContent,
} from "./courseContentActionHelpers";
import { updateContentPublicationStatus } from "./contentPublicationAction";

export function useCourseContentActions(args: CourseContentActionArgs) {
  const [deleteConfirmContentId, setDeleteConfirmContentId] = useState<number | null>(null);
  const [editingContentId, setEditingContentId] = useState<number | null>(null);
  const [processingHistoryByContentId, setProcessingHistoryByContentId] =
    useState<Record<number, ContentProcessingHistoryState>>({});
  const [uploadProgress, setUploadProgress] = useState<number | null>(null);

  function forgetProcessingHistory(contentId: number) {
    setProcessingHistoryByContentId((current) => {
      const next = { ...current };
      delete next[contentId];
      return next;
    });
  }

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
      forgetProcessingHistory(content.id);
    });
  }

  async function inspectProcessingHistory(content: TeacherCourseWorkspaceContent) {
    const chapter = findContentChapter(args.workspace, content);
    const token = readBrowserSessionToken();
    if (!token || !chapter) {
      args.setActionMessage("Chapter not found for this content item.");
      return;
    }
    setProcessingHistoryByContentId((current) => ({
      ...current,
      [content.id]: { ...idleProcessingHistoryState, status: "loading" },
    }));
    try {
      const history = await loadTeacherContentProcessingHistory({
        chapterId: chapter.id,
        contentId: content.id,
        courseId: args.courseId,
        token,
      });
      setProcessingHistoryByContentId((current) => ({
        ...current,
        [content.id]: { history, message: null, status: "success" },
      }));
    } catch (nextError) {
      setProcessingHistoryByContentId((current) => ({
        ...current,
        [content.id]: {
          history: null,
          message: normalizeRouteError(nextError).message,
          status: "error",
        },
      }));
    }
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
      forgetProcessingHistory(content.id);
      return response.message;
    });
  }

  async function setContentPublicationStatus(
    content: TeacherCourseWorkspaceContent,
    publicationStatus: "published" | "unpublished",
  ) {
    await updateContentPublicationStatus(args, content, publicationStatus);
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
    inspectProcessingHistory,
    isContentDraftDirty: isContentDraftDirty(args.contentDraft, editingContentId),
    processingHistoryByContentId,
    setContentPublicationStatus,
    submitContent,
    triggerProcessing,
    uploadProgress,
  };
}

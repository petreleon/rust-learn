import { type Dispatch, type SetStateAction } from "react";
import { readStoredSessionToken } from "@/lib/session";
import {
  deleteTeacherContent,
  processContent,
  type TeacherCourseWorkspaceContent,
  type TeacherCourseWorkspaceResponse,
} from "@/lib/teacher";
import { type ActionState } from "./ActionState";
import { findContentChapter } from "./contentAuthoringHelpers";
import { type ContentDraft } from "./ContentDraft";
import { defaultContentDraft } from "./defaultContentDraft";
import { isEditableTextContent } from "./isEditableTextContent";
import { normalizeRouteError } from "./normalizeRouteError";

type Args = {
  courseId: string;
  deleteConfirmContentId: number | null;
  editingContentId: number | null;
  loadContentRoute: () => Promise<void>;
  setActionMessage: Dispatch<SetStateAction<string | null>>;
  setActionState: Dispatch<SetStateAction<ActionState>>;
  setContentDraft: Dispatch<SetStateAction<ContentDraft>>;
  setDeleteConfirmContentId: Dispatch<SetStateAction<number | null>>;
  setEditingContentId: Dispatch<SetStateAction<number | null>>;
  workspace: TeacherCourseWorkspaceResponse | null;
};

export function createContentLifecycleActions(args: Args) {
  function cancelContentEdit() {
    args.setEditingContentId(null);
    args.setContentDraft(defaultContentDraft);
    args.setActionMessage(null);
  }

  function editContent(content: TeacherCourseWorkspaceContent) {
    const chapter = findContentChapter(args.workspace, content);
    if (!chapter) {
      args.setActionMessage("Chapter not found for this content item.");
      return;
    }
    if (!isEditableTextContent(content.content_type)) {
      args.setActionMessage("Only text and article content can be edited here.");
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
    args.setEditingContentId(content.id);
    args.setDeleteConfirmContentId(null);
    args.setActionMessage("Editing selected content item.");
  }

  async function deleteContent(content: TeacherCourseWorkspaceContent) {
    const token = readStoredSessionToken();
    const chapter = findContentChapter(args.workspace, content);
    if (!token || !chapter) {
      args.setActionMessage("Chapter not found for this content item.");
      return;
    }
    if (args.deleteConfirmContentId !== content.id) {
      args.setDeleteConfirmContentId(content.id);
      args.setActionMessage("Select delete again to remove this content item.");
      return;
    }

    args.setActionState("saving");
    args.setActionMessage(null);
    try {
      await deleteTeacherContent({ chapterId: chapter.id, contentId: content.id, courseId: args.courseId, token });
      if (args.editingContentId === content.id) {
        args.setEditingContentId(null);
        args.setContentDraft(defaultContentDraft);
      }
      args.setDeleteConfirmContentId(null);
      args.setActionMessage("Content item deleted.");
      await args.loadContentRoute();
    } catch (nextError) {
      args.setActionMessage(normalizeRouteError(nextError).message);
    } finally {
      args.setActionState("idle");
    }
  }

  async function triggerProcessing(content: TeacherCourseWorkspaceContent) {
    const token = readStoredSessionToken();
    const chapter = findContentChapter(args.workspace, content);
    if (!token || !chapter) {
      args.setActionMessage("Chapter not found for this content item.");
      return;
    }

    args.setActionState("saving");
    args.setActionMessage(null);
    try {
      const response = await processContent({
        chapterId: chapter.id,
        contentId: content.id,
        courseId: Number(args.courseId),
        token,
      });
      args.setActionMessage(response.message || "Processing queued. Refresh to check status.");
      await args.loadContentRoute();
    } catch (nextError) {
      args.setActionMessage(normalizeRouteError(nextError).message);
    } finally {
      args.setActionState("idle");
    }
  }

  return { cancelContentEdit, deleteContent, editContent, triggerProcessing };
}

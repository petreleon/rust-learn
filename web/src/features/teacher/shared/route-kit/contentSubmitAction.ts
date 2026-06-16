import { type Dispatch, type FormEvent, type SetStateAction } from "react";
import { readStoredSessionToken } from "@/lib/session";
import {
  createTeacherContent,
  fetchUploadUrl,
  updateTeacherContent,
  type TeacherCourseWorkspaceResponse,
} from "@/lib/teacher";
import { type ActionState } from "./ActionState";
import { contentDraftOrder } from "./contentAuthoringHelpers";
import { type ContentDraft } from "./ContentDraft";
import { defaultContentDraft } from "./defaultContentDraft";
import { normalizeRouteError } from "./normalizeRouteError";

type Args = {
  contentDraft: ContentDraft;
  courseId: string;
  editingContentId: number | null;
  loadContentRoute: () => Promise<void>;
  setActionMessage: Dispatch<SetStateAction<string | null>>;
  setActionState: Dispatch<SetStateAction<ActionState>>;
  setContentDraft: Dispatch<SetStateAction<ContentDraft>>;
  setEditingContentId: Dispatch<SetStateAction<number | null>>;
  workspace: TeacherCourseWorkspaceResponse | null;
};

async function submitTextContent(args: Args, token: string, chapterId: string, order: number) {
  const data = args.contentDraft.data.trim();
  if (!data) {
    args.setActionMessage("Lesson body is required for text content.");
    return;
  }

  args.setActionState("saving");
  args.setActionMessage(null);
  try {
    if (args.editingContentId) {
      await updateTeacherContent({
        chapterId,
        contentId: args.editingContentId,
        courseId: args.courseId,
        payload: { content_type: args.contentDraft.contentType, data, order },
        token,
      });
      args.setEditingContentId(null);
      args.setActionMessage("Content item updated.");
    } else {
      await createTeacherContent({
        chapterId,
        courseId: args.courseId,
        payload: { content_type: args.contentDraft.contentType, data, order },
        token,
      });
      args.setActionMessage("Content item created.");
    }
    args.setContentDraft(defaultContentDraft);
    await args.loadContentRoute();
  } catch (nextError) {
    args.setActionMessage(normalizeRouteError(nextError).message);
  } finally {
    args.setActionState("idle");
  }
}

async function submitUploadContent(args: Args, token: string, chapterId: string, order: number) {
  if (args.editingContentId) {
    args.setActionMessage("Cancel text editing before creating uploaded content.");
    return;
  }
  if (!args.contentDraft.file) {
    args.setActionMessage("Select a file to upload.");
    return;
  }
  const filename = args.contentDraft.filename.trim();
  if (!filename) {
    args.setActionMessage("Filename is required for uploaded content.");
    return;
  }

  args.setActionState("saving");
  args.setActionMessage(null);
  try {
    const { object_key, upload_url } = await fetchUploadUrl({
      chapterId: Number(chapterId),
      contentType: args.contentDraft.file.type || "application/octet-stream",
      courseId: Number(args.courseId),
      filename,
      token,
    });
    const putRes = await fetch(upload_url, { body: args.contentDraft.file, method: "PUT" });
    if (!putRes.ok) {
      args.setActionMessage(`Upload failed: ${putRes.status} ${putRes.statusText}`);
      return;
    }
    await createTeacherContent({
      chapterId,
      courseId: args.courseId,
      payload: {
        content_type: args.contentDraft.file.type || "application/octet-stream",
        data: object_key,
        order,
      },
      token,
    });
    args.setContentDraft(defaultContentDraft);
    args.setActionMessage("Upload succeeded and content record created.");
    await args.loadContentRoute();
  } catch (nextError) {
    args.setActionMessage(normalizeRouteError(nextError).message);
  } finally {
    args.setActionState("idle");
  }
}

export function createContentSubmitAction(args: Args) {
  return async function submitContent(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const token = readStoredSessionToken();
    const chapterId = args.contentDraft.chapterId;
    const { order, orderText } = contentDraftOrder(args.contentDraft);
    if (!token || !chapterId || !orderText || !Number.isFinite(order) || order < 0) {
      args.setActionMessage("Chapter and a non-negative order are required.");
      return;
    }
    if (args.contentDraft.uploadKind === "text") {
      await submitTextContent(args, token, chapterId, order);
      return;
    }
    await submitUploadContent(args, token, chapterId, order);
  };
}

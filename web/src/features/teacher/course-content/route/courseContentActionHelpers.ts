import { type Dispatch, type SetStateAction } from "react";
import { type TeacherCourseWorkspaceResponse } from "@/lib/teacher";
import { type ActionState } from "@/features/teacher/shared/route-kit/ActionState";
import { contentDraftOrder } from "@/features/teacher/shared/route-kit/contentAuthoringHelpers";
import { type ContentDraft } from "@/features/teacher/shared/route-kit/ContentDraft";
import { defaultContentDraft } from "@/features/teacher/shared/route-kit/defaultContentDraft";
import { normalizeRouteError } from "@/features/teacher/shared/route-kit/normalizeRouteError";
import {
  createTeacherCourseContentItem,
  requestTeacherContentUploadUrl,
  updateTeacherCourseContentItem,
  uploadTeacherContentFile,
} from "../api/courseContentApi";

export type CourseContentActionArgs = {
  contentDraft: ContentDraft;
  courseId: string;
  loadContentRoute: () => Promise<void>;
  setActionMessage: Dispatch<SetStateAction<string | null>>;
  setActionState: Dispatch<SetStateAction<ActionState>>;
  setContentDraft: Dispatch<SetStateAction<ContentDraft>>;
  workspace: TeacherCourseWorkspaceResponse | null;
};

export async function runContentAction(
  args: CourseContentActionArgs,
  fallback: string,
  action: () => Promise<string | void>,
) {
  args.setActionState("saving");
  args.setActionMessage(null);
  try {
    const message = await action();
    args.setActionMessage(message || fallback);
    await args.loadContentRoute();
  } catch (nextError) {
    args.setActionMessage(normalizeRouteError(nextError).message);
  } finally {
    args.setActionState("idle");
  }
}

export function isContentDraftDirty(draft: ContentDraft, editingContentId: number | null) {
  return draft.data.trim() !== "" || draft.file !== null || Boolean(editingContentId) || contentDraftOrder(draft).orderText !== "1";
}

export async function submitTextContent(
  args: CourseContentActionArgs,
  token: string,
  chapterId: string,
  order: number,
  editingContentId: number | null,
  setEditingContentId: Dispatch<SetStateAction<number | null>>,
) {
  const data = args.contentDraft.data.trim();
  if (!data) {
    args.setActionMessage("Lesson body is required for text content.");
    return;
  }
  await runContentAction(args, editingContentId ? "Content item updated." : "Content item created.", async () => {
    const payload = { content_type: args.contentDraft.contentType, data, order };
    if (editingContentId) {
      await updateTeacherCourseContentItem({ chapterId, contentId: editingContentId, courseId: args.courseId, payload, token });
      setEditingContentId(null);
    } else {
      await createTeacherCourseContentItem({ chapterId, courseId: args.courseId, payload, token });
    }
    args.setContentDraft(defaultContentDraft);
  });
}

export async function submitUploadContent(
  args: CourseContentActionArgs,
  token: string,
  chapterId: string,
  order: number,
  setUploadProgress: Dispatch<SetStateAction<number | null>>,
) {
  if (!args.contentDraft.file || !args.contentDraft.filename.trim()) {
    args.setActionMessage(args.contentDraft.file ? "Filename is required for uploaded content." : "Select a file to upload.");
    return;
  }
  await runContentAction(args, "Upload succeeded and content record created.", async () => {
    const contentType = args.contentDraft.file?.type || "application/octet-stream";
    const { object_key, upload_url } = await requestTeacherContentUploadUrl({
      chapterId,
      contentType,
      courseId: args.courseId,
      filename: args.contentDraft.filename.trim(),
      token,
    });
    await uploadTeacherContentFile({ file: args.contentDraft.file as File, onProgress: setUploadProgress, uploadUrl: upload_url });
    await createTeacherCourseContentItem({
      chapterId,
      courseId: args.courseId,
      payload: { content_type: contentType, data: object_key, order },
      token,
    });
    args.setContentDraft(defaultContentDraft);
  });
}

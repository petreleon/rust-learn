import { type Dispatch, type SetStateAction, useState } from "react";
import { type TeacherCourseWorkspaceResponse } from "@/lib/teacher";
import { type ActionState } from "./ActionState";
import { contentDraftOrder } from "./contentAuthoringHelpers";
import { createContentLifecycleActions } from "./contentLifecycleActions";
import { createContentSubmitAction } from "./contentSubmitAction";
import { type ContentDraft } from "./ContentDraft";

type Args = {
  contentDraft: ContentDraft;
  courseId: string;
  loadContentRoute: () => Promise<void>;
  setActionMessage: Dispatch<SetStateAction<string | null>>;
  setActionState: Dispatch<SetStateAction<ActionState>>;
  setContentDraft: Dispatch<SetStateAction<ContentDraft>>;
  workspace: TeacherCourseWorkspaceResponse | null;
};

export function useContentAuthoringActions(args: Args) {
  const [deleteConfirmContentId, setDeleteConfirmContentId] = useState<number | null>(null);
  const [editingContentId, setEditingContentId] = useState<number | null>(null);

  const submitContent = createContentSubmitAction({
    ...args,
    editingContentId,
    setEditingContentId,
  });
  const lifecycleActions = createContentLifecycleActions({
    ...args,
    deleteConfirmContentId,
    editingContentId,
    setDeleteConfirmContentId,
    setEditingContentId,
  });

  return {
    ...lifecycleActions,
    deleteConfirmContentId,
    editingContentId,
    isContentDraftDirty:
      args.contentDraft.data.trim() !== "" ||
      args.contentDraft.file !== null ||
      Boolean(editingContentId) ||
      contentDraftOrder(args.contentDraft).orderText !== "1",
    submitContent,
  };
}

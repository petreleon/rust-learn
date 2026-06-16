"use client";

import { AlertCircle, Loader2, LogIn, RefreshCw } from "lucide-react";
import Link from "next/link";
import { type Dispatch, type FormEvent, type ReactNode, type SetStateAction } from "react";
import { type TeacherCourseWorkspaceContent, type TeacherCourseWorkspaceResponse } from "@/lib/teacher";
import styles from "@/features/teacher/shared/teacher-routes.module.css";
import { type ActionState } from "./ActionState";
import { type ChapterDraft } from "./ChapterDraft";
import { type ContentProcessingHistoryState } from "./ContentProcessingHistoryState";
import { ContentAuthoringView } from "./ContentAuthoringView";
import { type ContentDraft } from "./ContentDraft";
import { type LoadState } from "./LoadState";
import { type RouteError } from "./RouteError";
import { StatePanel } from "./StatePanel";
type ContentActions = {
  cancelContentEdit: () => void;
  deleteConfirmContentId: number | null;
  deleteContent: (content: TeacherCourseWorkspaceContent) => void;
  editContent: (content: TeacherCourseWorkspaceContent) => void;
  editingContentId: number | null;
  inspectProcessingHistory: (content: TeacherCourseWorkspaceContent) => void;
  isContentDraftDirty: boolean;
  processingHistoryByContentId: Record<number, ContentProcessingHistoryState>;
  setContentPublicationStatus: (content: TeacherCourseWorkspaceContent, status: "published" | "unpublished") => void;
  submitContent: (event: FormEvent<HTMLFormElement>) => void;
  triggerProcessing: (content: TeacherCourseWorkspaceContent) => void;
  uploadProgress: number | null;
};

export function TeacherCourseContentPanels({
  actionMessage,
  actionState,
  assessmentPanel,
  chapterDraft,
  contentActions,
  contentDraft,
  courseId,
  error,
  loadContentRoute,
  loadState,
  setChapterDraft,
  setContentDraft,
  submitChapter,
  workspace,
}: {
  actionMessage: string | null;
  actionState: ActionState;
  assessmentPanel: ReactNode;
  chapterDraft: ChapterDraft;
  contentActions: ContentActions;
  contentDraft: ContentDraft;
  courseId: string;
  error: RouteError | null;
  loadContentRoute: () => Promise<void>;
  loadState: LoadState;
  setChapterDraft: Dispatch<SetStateAction<ChapterDraft>>;
  setContentDraft: Dispatch<SetStateAction<ContentDraft>>;
  submitChapter: (event: FormEvent<HTMLFormElement>) => void;
  workspace: TeacherCourseWorkspaceResponse | null;
}) {
  if (loadState === "loading") {
    return (
      <StatePanel
        detail="Loading course content structure and authoring permissions."
        icon={<Loader2 className={styles.spin} size={22} aria-hidden />}
        title="Loading content authoring"
      />
    );
  }
  if (loadState === "idle") {
    return (
      <StatePanel
        action={
          <Link className={styles.primaryLink} href={`/login?redirect=/teach/courses/${courseId}/content`}>
            <LogIn size={18} aria-hidden />
            Sign in
          </Link>
        }
        detail="Content authoring loads from your signed-in teaching session."
        icon={<LogIn size={22} aria-hidden />}
        title="Sign in required"
      />
    );
  }
  if (loadState === "error" && error) {
    return (
      <section className={`${styles.errorBox} ${styles.singlePanel}`} role="status">
        <AlertCircle size={18} aria-hidden />
        <span>
          <strong>{error.code}</strong> {error.message}
        </span>
        <button className={styles.secondaryButton} type="button" onClick={() => void loadContentRoute()}>
          <RefreshCw size={17} aria-hidden />
          Retry
        </button>
      </section>
    );
  }
  if (loadState !== "success" || !workspace) {
    return null;
  }
  return (
    <ContentAuthoringView
      actionMessage={actionMessage}
      actionState={actionState}
      assessmentPanel={assessmentPanel}
      chapterDraft={chapterDraft}
      contentDraft={contentDraft}
      deleteConfirmContentId={contentActions.deleteConfirmContentId}
      editingContentId={contentActions.editingContentId}
      onCancelEdit={contentActions.cancelContentEdit}
      onChapterDraftChange={setChapterDraft}
      onContentDraftChange={setContentDraft}
      onDeleteContent={contentActions.deleteContent}
      onEditContent={contentActions.editContent}
      onInspectProcessingHistory={contentActions.inspectProcessingHistory}
      onSetContentPublicationStatus={contentActions.setContentPublicationStatus}
      onSubmitChapter={submitChapter}
      onSubmitContent={contentActions.submitContent}
      onTriggerProcessing={contentActions.triggerProcessing}
      processingHistoryByContentId={contentActions.processingHistoryByContentId}
      uploadProgress={contentActions.uploadProgress}
      workspace={workspace}
    />
  );
}

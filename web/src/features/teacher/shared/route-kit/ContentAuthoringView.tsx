"use client";

import { AlertCircle, ArrowLeft } from "lucide-react";
import Link from "next/link";
import { type FormEvent, type ReactNode } from "react";
import { type TeacherCourseWorkspaceContent, type TeacherCourseWorkspaceResponse } from "@/lib/teacher";
import styles from "@/features/teacher/shared/teacher-routes.module.css";
import { ChapterAuthoringForm } from "./ChapterAuthoringForm";
import { ChapterList } from "./ChapterList";
import { ContentAuthoringForm } from "./ContentAuthoringForm";
import { statusLabel } from "./statusLabel";
import { type ActionState } from "./ActionState";
import { type ChapterDraft } from "./ChapterDraft";
import { type ContentProcessingHistoryState } from "./ContentProcessingHistoryState";
import { type ContentDraft } from "./ContentDraft";

export function ContentAuthoringView({
  actionMessage,
  actionState,
  assessmentPanel,
  chapterDraft,
  contentDraft,
  deleteConfirmContentId,
  editingContentId,
  onCancelEdit,
  onChapterDraftChange,
  onContentDraftChange,
  onDeleteContent,
  onEditContent,
  onInspectProcessingHistory,
  onSubmitChapter,
  onSubmitContent,
  onTriggerProcessing,
  processingHistoryByContentId,
  uploadProgress,
  workspace,
}: {
  actionMessage: string | null;
  actionState: ActionState;
  assessmentPanel: ReactNode;
  chapterDraft: ChapterDraft;
  contentDraft: ContentDraft;
  deleteConfirmContentId: number | null;
  editingContentId: number | null;
  onCancelEdit: () => void;
  onChapterDraftChange: (draft: ChapterDraft) => void;
  onContentDraftChange: (draft: ContentDraft) => void;
  onDeleteContent: (content: TeacherCourseWorkspaceContent) => void;
  onEditContent: (content: TeacherCourseWorkspaceContent) => void;
  onInspectProcessingHistory: (content: TeacherCourseWorkspaceContent) => void;
  onSubmitChapter: (event: FormEvent<HTMLFormElement>) => void;
  onSubmitContent: (event: FormEvent<HTMLFormElement>) => void;
  onTriggerProcessing: (content: TeacherCourseWorkspaceContent) => void;
  processingHistoryByContentId: Record<number, ContentProcessingHistoryState>;
  uploadProgress: number | null;
  workspace: TeacherCourseWorkspaceResponse;
}) {
  const canManageContent = workspace.course.permissions.can_manage_content;
  return (
    <>
      <section className={styles.workspaceHero}>
        <Link className={styles.secondaryLink} href={`/teach/courses/${workspace.course.id}`}>
          <ArrowLeft size={17} aria-hidden />
          Course workspace
        </Link>
        <div className={styles.workspaceTitleBlock}>
          <p className={styles.eyebrow}>{statusLabel(workspace.course.lifecycle_status)}</p>
          <h2>Content authoring</h2>
          <p className={styles.muted}>
            Create chapters, text lessons, uploads, processing retries, and content edits from one workspace.
          </p>
        </div>
      </section>
      {actionMessage ? (
        <section className={`${styles.warningPanel} ${styles.singlePanel}`} role="status">
          <div className={styles.panelHeader}>
            <AlertCircle size={18} aria-hidden />
            <h2>Authoring update</h2>
          </div>
          <p>{actionMessage}</p>
        </section>
      ) : null}
      <section className={styles.twoColumn}>
        <ChapterAuthoringForm
          actionState={actionState}
          canManageContent={canManageContent}
          chapterDraft={chapterDraft}
          onChapterDraftChange={onChapterDraftChange}
          onSubmitChapter={onSubmitChapter}
        />
        <ContentAuthoringForm
          actionState={actionState}
          canManageContent={canManageContent}
          contentDraft={contentDraft}
          editingContentId={editingContentId}
          onCancelEdit={onCancelEdit}
          onContentDraftChange={onContentDraftChange}
          onSubmitContent={onSubmitContent}
          uploadProgress={uploadProgress}
          workspace={workspace}
        />
      </section>
      <section className={styles.courseSection}>
        <div className={styles.sectionHeader}>
          <div>
            <h2>Current structure</h2>
            <p className={styles.muted}>New chapters and content appear here after the workspace refreshes.</p>
          </div>
        </div>
        <ChapterList
          actionState={actionState}
          canManageContent={canManageContent}
          chapters={workspace.chapters}
          deleteConfirmContentId={deleteConfirmContentId}
          editingContentId={editingContentId}
          onDeleteContent={onDeleteContent}
          onEditContent={onEditContent}
          onInspectProcessingHistory={onInspectProcessingHistory}
          onTriggerProcessing={onTriggerProcessing}
          processingHistoryByContentId={processingHistoryByContentId}
        />
      </section>
      {assessmentPanel}
    </>
  );
}

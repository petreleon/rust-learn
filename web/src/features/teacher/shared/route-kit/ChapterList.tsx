"use client";

import { BookOpen } from "lucide-react";
import { type TeacherCourseWorkspaceContent, type TeacherCourseWorkspaceResponse } from "@/lib/teacher";
import styles from "@/features/teacher/shared/teacher-routes.module.css";
import { type ActionState } from "./ActionState";
import { type ContentProcessingHistoryState } from "./ContentProcessingHistoryState";
import { ContentRow } from "./ContentRow";

export function ChapterList({
  actionState = "idle",
  canManageContent = false,
  chapters,
  deleteConfirmContentId = null,
  editingContentId = null,
  onDeleteContent,
  onEditContent,
  onInspectProcessingHistory,
  onSetContentPublicationStatus,
  onTriggerProcessing,
  processingHistoryByContentId = {},
}: {
  actionState?: ActionState;
  canManageContent?: boolean;
  chapters: TeacherCourseWorkspaceResponse["chapters"];
  deleteConfirmContentId?: number | null;
  editingContentId?: number | null;
  onDeleteContent?: (content: TeacherCourseWorkspaceContent) => void;
  onEditContent?: (content: TeacherCourseWorkspaceContent) => void;
  onInspectProcessingHistory?: (content: TeacherCourseWorkspaceContent) => void;
  onSetContentPublicationStatus?: (content: TeacherCourseWorkspaceContent, status: "published" | "unpublished") => void;
  onTriggerProcessing?: (content: TeacherCourseWorkspaceContent) => void;
  processingHistoryByContentId?: Record<number, ContentProcessingHistoryState>;
}) {
  if (!chapters.length) {
    return (
      <section className={`${styles.panel} ${styles.singlePanel}`}>
        <div className={styles.panelHeader}>
          <BookOpen size={20} aria-hidden />
          <h2>No chapters</h2>
        </div>
        <p className={styles.muted}>Create the first chapter with the form above.</p>
      </section>
    );
  }

  return (
    <div className={styles.chapterList}>
      {chapters.map((chapter) => (
        <article className={styles.chapterCard} key={chapter.id}>
          <div className={styles.courseTop}>
            <div>
              <p className={styles.eyebrow}>Chapter {chapter.order}</p>
              <h3>{chapter.title}</h3>
            </div>
            <span className={`${styles.statusPill} ${styles.neutral}`}>{chapter.contents.length} item{chapter.contents.length === 1 ? "" : "s"}</span>
          </div>
          {chapter.contents.length ? (
            <div className={styles.contentList}>
              {chapter.contents.map((content) => (
                <ContentRow
                  actionState={actionState}
                  canManageContent={canManageContent}
                  content={content}
                  deleteConfirmContentId={deleteConfirmContentId}
                  editingContentId={editingContentId}
                  key={content.id}
                  onDeleteContent={onDeleteContent}
                  onEditContent={onEditContent}
                  onInspectProcessingHistory={onInspectProcessingHistory}
                  onSetContentPublicationStatus={onSetContentPublicationStatus}
                  onTriggerProcessing={onTriggerProcessing}
                  processingHistory={processingHistoryByContentId[content.id]}
                />
              ))}
            </div>
          ) : (
            <p className={styles.muted}>No content items in this chapter yet.</p>
          )}
        </article>
      ))}
    </div>
  );
}

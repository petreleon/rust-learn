"use client";

import { History, Pencil, Trash2, Video } from "lucide-react";
import { type TeacherCourseWorkspaceContent } from "@/lib/teacher";
import styles from "@/features/teacher/shared/teacher-routes.module.css";
import { type ActionState } from "./ActionState";
import { ContentProcessingHistoryPanel } from "./ContentProcessingHistoryPanel";
import { type ContentProcessingHistoryState } from "./ContentProcessingHistoryState";
import { isEditableTextContent } from "./isEditableTextContent";
import { statusLabel } from "./statusLabel";

export function ContentRow({
  actionState = "idle",
  canManageContent = false,
  content,
  deleteConfirmContentId = null,
  editingContentId = null,
  onDeleteContent,
  onEditContent,
  onInspectProcessingHistory,
  onTriggerProcessing,
  processingHistory,
}: {
  actionState?: ActionState;
  canManageContent?: boolean;
  content: TeacherCourseWorkspaceContent;
  deleteConfirmContentId?: number | null;
  editingContentId?: number | null;
  onDeleteContent?: (content: TeacherCourseWorkspaceContent) => void;
  onEditContent?: (content: TeacherCourseWorkspaceContent) => void;
  onInspectProcessingHistory?: (content: TeacherCourseWorkspaceContent) => void;
  onTriggerProcessing?: (content: TeacherCourseWorkspaceContent) => void;
  processingHistory?: ContentProcessingHistoryState;
}) {
  const tone = content.display_state === "failed_processing" ? "warn" : content.display_state === "ready" ? "good" : "neutral";
  const isVideoLike = content.content_type === "video" || content.content_type.startsWith("video/");
  const canProcess = isVideoLike && content.data_present && (content.display_state === "uploaded" || content.display_state === "failed_processing");
  const canInspect = canManageContent && isVideoLike && content.data_present && Boolean(onInspectProcessingHistory);
  const canEdit = canManageContent && isEditableTextContent(content.content_type) && Boolean(onEditContent);
  const canDelete = canManageContent && Boolean(onDeleteContent);
  const disabled = actionState === "saving";
  const isConfirmingDelete = deleteConfirmContentId === content.id;
  const isEditing = editingContentId === content.id;
  return (
    <article className={styles.contentRow}>
      <div>
        <strong>{statusLabel(content.content_type)}</strong>
        <p>
          Order {content.order} - {content.data_present ? "Data recorded" : "No stored data"} - {statusLabel(content.publication_status)}
        </p>
      </div>
      <div style={{ display: "flex", flexWrap: "wrap", gap: "0.5rem", alignItems: "center" }}>
        <span className={`${styles.statusPill} ${styles[tone]}`}>{statusLabel(content.display_state)}</span>
        {isEditing ? <span className={`${styles.statusPill} ${styles.neutral}`}>Editing</span> : null}
        {canEdit ? (
          <button
            className={styles.secondaryButton}
            disabled={disabled}
            onClick={() => onEditContent?.(content)}
            title="Edit text content"
            type="button"
          >
            <Pencil size={16} aria-hidden />
            Edit
          </button>
        ) : null}
        {canProcess && onTriggerProcessing ? (
          <button
            className={styles.secondaryButton}
            disabled={disabled}
            onClick={() => onTriggerProcessing(content)}
            title="Queue video processing"
            type="button"
          >
            <Video size={16} aria-hidden />
            Process
          </button>
        ) : null}
        {canInspect ? (
          <button
            className={styles.secondaryButton}
            disabled={disabled || processingHistory?.status === "loading"}
            onClick={() => onInspectProcessingHistory?.(content)}
            title="Inspect processing history"
            type="button"
          >
            <History size={16} aria-hidden />
            History
          </button>
        ) : null}
        {canDelete ? (
          <button
            className={styles.secondaryButton}
            disabled={disabled}
            onClick={() => onDeleteContent?.(content)}
            title={isConfirmingDelete ? "Confirm content deletion" : "Delete content"}
            type="button"
          >
            <Trash2 size={16} aria-hidden />
            {isConfirmingDelete ? "Confirm delete" : "Delete"}
          </button>
        ) : null}
      </div>
      {content.processing_status ? <small>{statusLabel(content.processing_status)}</small> : null}
      {content.processing_error ? <p className={styles.reviewNote}>{content.processing_error}</p> : null}
      {processingHistory ? <ContentProcessingHistoryPanel state={processingHistory} /> : null}
    </article>
  );
}

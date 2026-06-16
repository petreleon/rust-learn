"use client";

import { FileText, Send, Upload, X } from "lucide-react";
import { type FormEvent } from "react";
import { type TeacherCourseWorkspaceResponse } from "@/lib/teacher";
import styles from "@/features/teacher/shared/teacher-routes.module.css";
import { type ActionState } from "./ActionState";
import { type ContentDraft } from "./ContentDraft";

export function ContentAuthoringForm({
  actionState,
  canManageContent,
  contentDraft,
  editingContentId,
  onCancelEdit,
  onContentDraftChange,
  onSubmitContent,
  workspace,
}: {
  actionState: ActionState;
  canManageContent: boolean;
  contentDraft: ContentDraft;
  editingContentId: number | null;
  onCancelEdit: () => void;
  onContentDraftChange: (draft: ContentDraft) => void;
  onSubmitContent: (event: FormEvent<HTMLFormElement>) => void;
  workspace: TeacherCourseWorkspaceResponse;
}) {
  const isEditing = editingContentId !== null;
  const hasChapters = workspace.chapters.length > 0;
  const disabled = !canManageContent || actionState === "saving";
  const submitLabel = isEditing ? "Update content" : contentDraft.uploadKind === "file" ? "Upload content" : "Create content";
  const title = isEditing ? "Edit text content" : contentDraft.uploadKind === "file" ? "Upload file content" : "Create text content";
  return (
    <form className={styles.authoringForm} onSubmit={onSubmitContent}>
      <div className={styles.panelHeader}>
        {contentDraft.uploadKind === "file" ? <Upload size={20} aria-hidden /> : <FileText size={20} aria-hidden />}
        <h2>{title}</h2>
      </div>
      <label>
        <span>Chapter</span>
        <select
          disabled={disabled || !hasChapters || isEditing}
          onChange={(event) => onContentDraftChange({ ...contentDraft, chapterId: event.target.value })}
          value={contentDraft.chapterId || workspace.chapters[0]?.id.toString() || ""}
        >
          {hasChapters ? (
            workspace.chapters.map((chapter) => (
              <option key={chapter.id} value={chapter.id}>
                {chapter.title}
              </option>
            ))
          ) : (
            <option value="">Create a chapter first</option>
          )}
        </select>
      </label>
      <label>
        <span>Kind</span>
        <select
          disabled={disabled || isEditing}
          onChange={(event) =>
            onContentDraftChange({
              ...contentDraft,
              contentType: event.target.value === "file" ? "video/mp4" : "article",
              uploadKind: event.target.value as "text" | "file",
            })
          }
          value={contentDraft.uploadKind}
        >
          <option value="text">Text / Article</option>
          <option value="file">File upload</option>
        </select>
      </label>
      {contentDraft.uploadKind === "file" ? (
        <>
          <label>
            <span>File</span>
            <input
              accept="video/*,application/pdf"
              disabled={disabled || !hasChapters}
              onChange={(event) => {
                const file = event.target.files?.[0] ?? null;
                onContentDraftChange({ ...contentDraft, file, filename: file ? file.name : "" });
              }}
              type="file"
            />
          </label>
          <label>
            <span>Object filename</span>
            <input
              disabled={disabled}
              maxLength={240}
              onChange={(event) => onContentDraftChange({ ...contentDraft, filename: event.target.value })}
              placeholder="my-video.mp4"
              value={contentDraft.filename}
            />
          </label>
        </>
      ) : (
        <>
          <label>
            <span>Type</span>
            <select
              disabled={disabled}
              onChange={(event) => onContentDraftChange({ ...contentDraft, contentType: event.target.value })}
              value={contentDraft.contentType}
            >
              <option value="article">Article</option>
              <option value="text">Text lesson</option>
            </select>
          </label>
          <label>
            <span>Lesson body</span>
            <textarea
              disabled={disabled || !hasChapters}
              onChange={(event) => onContentDraftChange({ ...contentDraft, data: event.target.value })}
              placeholder="Write the lesson content"
              rows={5}
              value={contentDraft.data}
            />
          </label>
        </>
      )}
      <label>
        <span>Order</span>
        <input
          disabled={disabled}
          min="0"
          onChange={(event) => onContentDraftChange({ ...contentDraft, order: event.target.value })}
          type="number"
          value={contentDraft.order}
        />
      </label>
      <div className={styles.actionRow}>
        <button className={styles.primaryButton} disabled={disabled || !hasChapters} type="submit">
          <Send size={17} aria-hidden />
          {submitLabel}
        </button>
        {isEditing ? (
          <button className={styles.secondaryButton} disabled={actionState === "saving"} onClick={onCancelEdit} type="button">
            <X size={17} aria-hidden />
            Cancel
          </button>
        ) : null}
      </div>
      {!hasChapters ? <p className={styles.muted}>Create a chapter first so content can be assigned to it.</p> : null}
      {contentDraft.uploadKind === "file" && hasChapters ? (
        <p className={styles.muted}>The presigned upload URL expires after 1 hour. Use Process after the record is created.</p>
      ) : null}
    </form>
  );
}

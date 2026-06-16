"use client";

import { BookOpen, Send } from "lucide-react";
import { type FormEvent } from "react";
import styles from "@/features/teacher/shared/teacher-routes.module.css";
import { type ActionState } from "./ActionState";
import { type ChapterDraft } from "./ChapterDraft";

export function ChapterAuthoringForm({
  actionState,
  canManageContent,
  chapterDraft,
  onChapterDraftChange,
  onSubmitChapter,
}: {
  actionState: ActionState;
  canManageContent: boolean;
  chapterDraft: ChapterDraft;
  onChapterDraftChange: (draft: ChapterDraft) => void;
  onSubmitChapter: (event: FormEvent<HTMLFormElement>) => void;
}) {
  const disabled = !canManageContent || actionState === "saving";
  return (
    <form className={styles.authoringForm} onSubmit={onSubmitChapter}>
      <div className={styles.panelHeader}>
        <BookOpen size={20} aria-hidden />
        <h2>Create chapter</h2>
      </div>
      <label>
        <span>Title</span>
        <input
          disabled={disabled}
          maxLength={120}
          onChange={(event) => onChapterDraftChange({ ...chapterDraft, title: event.target.value })}
          placeholder="Chapter title"
          value={chapterDraft.title}
        />
      </label>
      <label>
        <span>Order</span>
        <input
          disabled={disabled}
          min="0"
          onChange={(event) => onChapterDraftChange({ ...chapterDraft, order: event.target.value })}
          type="number"
          value={chapterDraft.order}
        />
      </label>
      <button className={styles.primaryButton} disabled={disabled} type="submit">
        <Send size={17} aria-hidden />
        Create chapter
      </button>
      {!canManageContent ? (
        <p className={styles.muted}>This session can view content structure but cannot author course content.</p>
      ) : null}
    </form>
  );
}

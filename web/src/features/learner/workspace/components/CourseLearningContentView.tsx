"use client";

import { ArrowLeft, BookOpen, CheckCircle, FileText } from "lucide-react";
import { type CourseLearningContent, type CourseLearningResponse } from "@/lib/learner";
import { CourseAssessmentPanel } from "../assessments/route/CourseAssessmentPanel";
import styles from "../learner-workspace.module.css";
import { EmptyState } from "./EmptyState";
import { LessonContentBody } from "./LessonContentBody";
import { StatusPill } from "./StatusPill";
import { SummaryCard } from "./SummaryCard";
import { contentStateTone } from "./contentStateTone";
import { humanize } from "./humanize";
import { LearningAccessNotice } from "./LearningAccessNotice";

export function CourseLearningContentView({
  courseId,
  learning,
  onSelectContent,
  selectedContent,
  selectedContentId,
}: {
  courseId: number;
  learning: CourseLearningResponse;
  onSelectContent: (contentId: number) => void;
  selectedContent: CourseLearningContent | null;
  selectedContentId: number | null;
}) {
  const allContents = learning.chapters.flatMap((chapter) => chapter.contents);
  const selectedIndex = selectedContent
    ? allContents.findIndex((content) => content.id === selectedContent.id)
    : -1;
  const previousContent = selectedIndex > 0 ? allContents[selectedIndex - 1] : null;
  const nextContent = selectedIndex >= 0 && selectedIndex < allContents.length - 1 ? allContents[selectedIndex + 1] : null;
  const progressLabel = learning.progress_supported ? "Tracked" : "Preview only";

  return (
    <>
      <section className={styles.contentGrid}>
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Chapters" value={learning.chapters.length} />
        <SummaryCard icon={<FileText size={20} aria-hidden />} label="Lessons" value={allContents.length} />
        <SummaryCard
          icon={<CheckCircle size={20} aria-hidden />}
          label="Progress"
          value={progressLabel}
        />
      </section>

      <LearningAccessNotice learning={learning} />

      <section className={styles.learningLayout}>
        <aside className={styles.lessonOutline} aria-label="Course outline">
          <div className={styles.panelHeader}>
            <BookOpen size={20} aria-hidden />
            <h2>Outline</h2>
          </div>
          {learning.chapters.length ? (
            learning.chapters.map((chapter) => (
              <div className={styles.lessonChapter} key={chapter.id}>
                <h3>{chapter.title}</h3>
                {chapter.contents.length ? (
                  <div className={styles.lessonList}>
                    {chapter.contents.map((content) => (
                      <button
                        aria-pressed={selectedContentId === content.id}
                        className={`${styles.lessonButton} ${selectedContentId === content.id ? styles.activeLesson : ""}`}
                        key={content.id}
                        type="button"
                        onClick={() => onSelectContent(content.id)}
                      >
                        <span>{content.order + 1}. {humanize(content.content_type)}</span>
                        <StatusPill label={humanize(content.display_state)} tone={contentStateTone(content.display_state)} />
                      </button>
                    ))}
                  </div>
                ) : (
                  <p className={styles.muted}>No lessons in this chapter yet.</p>
                )}
              </div>
            ))
          ) : (
            <p className={styles.muted}>No course outline is available yet.</p>
          )}
        </aside>

        <section className={styles.lessonReader}>
          {selectedContent ? (
            <>
              <div className={styles.sectionHeader}>
                <h2>{humanize(selectedContent.content_type)}</h2>
                <StatusPill label={humanize(selectedContent.display_state)} tone={contentStateTone(selectedContent.display_state)} />
              </div>
              <LessonContentBody content={selectedContent} courseId={courseId} />
              <div className={styles.actionRow}>
                <button
                  className={styles.secondaryButton}
                  disabled={!previousContent}
                  type="button"
                  onClick={() => previousContent ? onSelectContent(previousContent.id) : undefined}
                >
                  <ArrowLeft size={18} aria-hidden />
                  Previous
                </button>
                <button
                  className={styles.secondaryButton}
                  disabled={!nextContent}
                  type="button"
                  onClick={() => nextContent ? onSelectContent(nextContent.id) : undefined}
                >
                  Next
                </button>
              </div>
            </>
          ) : (
            <EmptyState detail="Course content has not been published yet." title="No lesson selected" />
          )}
        </section>
      </section>
      <CourseAssessmentPanel
        courseId={courseId}
        submissionsEnabled={learning.progress_supported}
      />
    </>
  );
}

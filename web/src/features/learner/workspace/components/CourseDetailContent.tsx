"use client";

import { ArrowLeft, BookOpen, CheckCircle, FileText, Loader2, Trophy } from "lucide-react";
import Link from "next/link";
import { type AssessmentItem, type CourseCatalogDetail, type CourseCatalogItem } from "@/lib/learner";
import { CourseAssessmentEntryPanel } from "../assessments/components/CourseAssessmentEntryPanel";
import styles from "../learner-workspace.module.css";
import { EmptyState } from "./EmptyState";
import { StatusPill } from "./StatusPill";
import { SummaryCard } from "./SummaryCard";
import { humanize } from "./humanize";
import { plural } from "./plural";

export function CourseDetailContent({
  detail,
  assessments,
  joining,
  onRequestJoin,
}: {
  detail: CourseCatalogDetail;
  assessments: AssessmentItem[];
  joining: boolean;
  onRequestJoin: (course: CourseCatalogItem) => void;
}) {
  const course = detail.course;
  const organizationLabel = course.organizations.map((organization) => organization.name).join(", ") || "Independent";
  const teacherLabel = course.teachers.map((teacher) => teacher.name).join(", ") || "Teacher pending";
  const contentTypes = course.content.content_types.map(humanize).join(", ") || "Content pending";
  const rewardLabel = course.rewards.available
    ? `${course.rewards.active_policy_count} active ${plural(course.rewards.active_policy_count)}`
    : "No active policy";

  return (
    <>
      <section className={styles.contentGrid}>
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Chapters" value={course.content.chapter_count} />
        <SummaryCard icon={<FileText size={20} aria-hidden />} label="Content items" value={course.content.content_count} />
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Rewards" value={rewardLabel} />
      </section>

      <section className={styles.catalogPanel}>
        <div className={styles.panelHeader}>
          <BookOpen size={20} aria-hidden />
          <h2>Overview</h2>
          <StatusPill label={humanize(course.lifecycle_status)} tone={course.lifecycle_status === "published" ? "good" : "neutral"} />
        </div>
        <div className={styles.metaRow}>
          <span>{organizationLabel}</span>
          <span>{teacherLabel}</span>
          <span>{contentTypes}</span>
        </div>
        <div className={styles.detailList}>
          <span>{course.enrollment.reason || humanize(course.enrollment.state)}</span>
          {course.rewards.available ? (
            <span>{course.rewards.event_types.map(humanize).join(", ")} rewards</span>
          ) : null}
          {detail.prerequisites.length ? <span>{detail.prerequisites.join(", ")}</span> : null}
        </div>
        <div className={styles.actionRow}>
          <Link className={styles.secondaryLink} href="/courses">
            <ArrowLeft size={18} aria-hidden />
            Courses
          </Link>
          {course.enrollment.can_request_join ? (
            <button className={styles.primaryLink} disabled={joining} type="button" onClick={() => onRequestJoin(course)}>
              {joining ? <Loader2 className={styles.spin} size={18} aria-hidden /> : <CheckCircle size={18} aria-hidden />}
              Request join
            </button>
          ) : null}
          {course.access.can_view_content && course.content.has_content ? (
            <Link className={styles.primaryLink} href={`/courses/${course.id}/learn`}>
              <BookOpen size={18} aria-hidden />
              Start learning
            </Link>
          ) : null}
          {course.access.can_view_rewards ? (
            <Link className={styles.secondaryLink} href="/rewards">
              <Trophy size={18} aria-hidden />
              Rewards
            </Link>
          ) : null}
        </div>
      </section>

      <CourseAssessmentEntryPanel assessments={assessments} course={course} />

      <section className={styles.section}>
        <div className={styles.sectionHeader}>
          <h2>Syllabus</h2>
          <StatusPill label={`${detail.chapters.length} chapters`} tone="neutral" />
        </div>
        {detail.chapters.length ? (
          <div className={styles.itemGrid}>
            {detail.chapters.map((chapter) => (
              <article className={styles.itemCard} key={chapter.id}>
                <div className={styles.itemHeader}>
                  <h3>{chapter.title}</h3>
                  <StatusPill label={`#${chapter.order + 1}`} tone="neutral" />
                </div>
                {chapter.contents.length ? (
                  <div className={styles.detailList}>
                    {chapter.contents.map((content) => (
                      <span key={content.id}>
                        {content.order + 1}. {humanize(content.content_type)}
                      </span>
                    ))}
                  </div>
                ) : (
                  <p className={styles.muted}>No content items yet.</p>
                )}
              </article>
            ))}
          </div>
        ) : (
          <EmptyState detail="Syllabus content has not been added yet." title="No syllabus yet" />
        )}
      </section>
    </>
  );
}

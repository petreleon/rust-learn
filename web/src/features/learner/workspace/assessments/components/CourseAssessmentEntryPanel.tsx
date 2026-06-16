"use client";

import { ClipboardCheck } from "lucide-react";
import Link from "next/link";
import { type AssessmentItem, type CourseCatalogItem } from "@/lib/learner";
import { StatusPill } from "../../components/StatusPill";
import { humanize } from "../../components/humanize";
import styles from "../../learner-workspace.module.css";

export function CourseAssessmentEntryPanel({
  assessments,
  course,
}: {
  assessments: AssessmentItem[];
  course: CourseCatalogItem;
}) {
  if (!assessments.length) return null;

  const actionLabel = course.enrollment.state === "enrolled"
    ? "Open assessments"
    : "Preview assessments";

  return (
    <section className={styles.section}>
      <div className={styles.sectionHeader}>
        <h2>Assessments</h2>
        <StatusPill label={`${assessments.length} published`} tone="neutral" />
      </div>
      <div className={styles.assessmentGrid}>
        {assessments.map((assessment) => (
          <article className={styles.assessmentCard} key={assessment.id}>
            <div className={styles.itemHeader}>
              <div>
                <h3>{assessment.title}</h3>
                {assessment.description ? <p className={styles.muted}>{assessment.description}</p> : null}
              </div>
              <StatusPill label={`${assessment.questions.length} questions`} tone="neutral" />
            </div>
            <div className={styles.metaRow}>
              <span>{assessment.passing_score}% passing</span>
              <span>{assessment.max_attempts} attempts</span>
              <span>{humanize(course.enrollment.state)}</span>
            </div>
          </article>
        ))}
      </div>
      {course.access.can_view_content ? (
        <Link className={styles.primaryLink} href={`/courses/${course.id}/learn`}>
          <ClipboardCheck size={18} aria-hidden />
          {actionLabel}
        </Link>
      ) : (
        <p className={styles.assessmentNotice}>Request course access before opening assessments.</p>
      )}
    </section>
  );
}

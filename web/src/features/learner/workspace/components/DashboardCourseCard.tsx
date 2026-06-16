"use client";

import { BookOpen } from "lucide-react";
import Link from "next/link";
import { type CourseCatalogItem, type CourseProgress } from "@/lib/learner";
import styles from "../learner-workspace.module.css";
import { StatusPill } from "./StatusPill";
import { courseContentLabel } from "./courseContentLabel";
import { courseOrganizationLabel } from "./courseOrganizationLabel";
import { courseTeacherLabel } from "./courseTeacherLabel";
import { enrollmentTone } from "./enrollmentTone";
import { humanize } from "./humanize";
import { lifecycleTone } from "./lifecycleTone";

function progressLabel(progress: CourseProgress | undefined) {
  if (progress === undefined) return null;
  if (!progress) return "No saved progress yet";
  return `Progress saved ${new Date(progress.viewed_at).toLocaleDateString()}`;
}

export function DashboardCourseCard({ course, progress }: { course: CourseCatalogItem; progress?: CourseProgress }) {
  const savedProgressLabel = progressLabel(progress);
  return (
    <article className={styles.itemCard}>
      <div className={styles.itemHeader}>
        <h3>{course.title}</h3>
        <StatusPill label={humanize(course.enrollment.state)} tone={enrollmentTone(course.enrollment.state)} />
      </div>
      <div className={styles.metaRow}>
        <StatusPill label={humanize(course.lifecycle_status)} tone={lifecycleTone(course.lifecycle_status)} />
        <span>{courseOrganizationLabel(course)}</span>
        <span>{courseTeacherLabel(course)}</span>
        <span>{courseContentLabel(course)}</span>
        {savedProgressLabel ? <span>{savedProgressLabel}</span> : null}
      </div>
      <p className={styles.muted}>{course.enrollment.reason || "Open course details for enrollment and reward requirements."}</p>
      <div className={styles.actionRow}>
        {course.access.can_view_content && course.content.has_content ? (
          <Link className={styles.primaryLink} href={`/courses/${course.id}/learn`}>
            <BookOpen size={18} aria-hidden />
            Learn
          </Link>
        ) : null}
        <Link className={styles.secondaryLink} href={`/courses/${course.id}`}>
          Details
        </Link>
      </div>
    </article>
  );
}

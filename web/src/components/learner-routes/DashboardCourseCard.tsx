"use client";

import { BookOpen } from "lucide-react";
import Link from "next/link";
import { type CourseCatalogItem } from "@/lib/learner";
import styles from "../learner-routes.module.css";
import { StatusPill } from "./StatusPill";
import { courseContentLabel } from "./courseContentLabel";
import { courseOrganizationLabel } from "./courseOrganizationLabel";
import { courseTeacherLabel } from "./courseTeacherLabel";
import { enrollmentTone } from "./enrollmentTone";
import { humanize } from "./humanize";
import { lifecycleTone } from "./lifecycleTone";

export function DashboardCourseCard({ course }: { course: CourseCatalogItem }) {
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

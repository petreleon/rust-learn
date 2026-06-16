"use client";

import { BookOpen, CheckCircle, Loader2 } from "lucide-react";
import Link from "next/link";
import { type CourseCatalogItem } from "@/lib/learner";
import styles from "../learner-workspace.module.css";
import { StatusPill } from "./StatusPill";
import { enrollmentTone } from "./enrollmentTone";
import { humanize } from "./humanize";
import { lifecycleTone } from "./lifecycleTone";
import { plural } from "./plural";

export function CourseCatalogCard({
  course,
  emailVerified,
  joining,
  onRequestJoin,
}: {
  course: CourseCatalogItem;
  emailVerified?: boolean;
  joining: boolean;
  onRequestJoin: (course: CourseCatalogItem) => void;
}) {
  const organizationLabel = course.organizations.map((organization) => organization.name).join(", ") || "Independent";
  const teacherLabel = course.teachers.map((teacher) => teacher.name).join(", ") || "Teacher pending";
  const contentLabel = course.content.has_content
    ? `${course.content.chapter_count} chapters, ${course.content.content_count} items`
    : "Content pending";
  const rewardLabel = course.rewards.available
    ? `${course.rewards.active_policy_count} reward ${plural(course.rewards.active_policy_count)}`
    : "No active rewards";
  const emailBlocked = course.enrollment.can_request_join && emailVerified === false;

  return (
    <article className={styles.itemCard}>
      <div className={styles.itemHeader}>
        <h3>{course.title}</h3>
        <StatusPill label={humanize(course.enrollment.state)} tone={enrollmentTone(course.enrollment.state)} />
      </div>
      <div className={styles.metaRow}>
        <StatusPill label={humanize(course.lifecycle_status)} tone={lifecycleTone(course.lifecycle_status)} />
        <span>{organizationLabel}</span>
        <span>{teacherLabel}</span>
      </div>
      <div className={styles.detailList}>
        <span>{contentLabel}</span>
        <span>{rewardLabel}</span>
        {course.enrollment.reason ? <span>{course.enrollment.reason}</span> : null}
        {emailBlocked ? <span className={styles.muted}>Verify your email address before requesting enrollment.</span> : null}
      </div>
      <div className={styles.actionRow}>
        <Link className={styles.secondaryLink} href={`/courses/${course.id}`}>
          <BookOpen size={18} aria-hidden />
          Details
        </Link>
        {course.enrollment.can_request_join ? (
          <button
            className={styles.primaryLink}
            disabled={joining || emailBlocked}
            type="button"
            onClick={() => onRequestJoin(course)}
          >
            {joining ? <Loader2 className={styles.spin} size={18} aria-hidden /> : <CheckCircle size={18} aria-hidden />}
            {emailBlocked ? "Verify email first" : "Request join"}
          </button>
        ) : null}
      </div>
    </article>
  );
}

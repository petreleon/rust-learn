"use client";

import { type CourseSessionScope } from "@/lib/session";
import styles from "../page.module.css";
import { ScopeSummary } from "./ScopeSummary";

export function CourseCard({ course }: { course: CourseSessionScope }) {
  return (
    <article className={styles.scopeCard}>
      <h3>{course.title}</h3>
      <p className={styles.muted}>{course.lifecycle_status}</p>
      <ScopeSummary title="Access" scope={course} />
    </article>
  );
}

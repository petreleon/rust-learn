"use client";

import { BookOpen } from "lucide-react";
import { type TeacherCourseDashboardItem } from "@/lib/teacher";
import styles from "../teacher-routes.module.css";
import { CourseCard } from "./CourseCard";

export function CourseGrid({
  courses,
  emptyDetail,
}: {
  courses: TeacherCourseDashboardItem[];
  emptyDetail: string;
}) {
  if (!courses.length) {
    return (
      <section className={`${styles.panel} ${styles.singlePanel}`}>
        <div className={styles.panelHeader}>
          <BookOpen size={20} aria-hidden />
          <h2>No courses</h2>
        </div>
        <p className={styles.muted}>{emptyDetail}</p>
      </section>
    );
  }

  return (
    <div className={styles.courseGrid}>
      {courses.map((course) => (
        <CourseCard course={course} key={course.id} />
      ))}
    </div>
  );
}

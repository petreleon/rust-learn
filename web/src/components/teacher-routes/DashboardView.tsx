"use client";

import { BookOpen, BriefcaseBusiness, FileText, Trophy, Users } from "lucide-react";
import Link from "next/link";
import { type TeacherApplication, type TeacherCourseDashboardItem } from "@/lib/teacher";
import styles from "../teacher-routes.module.css";
import { ApplicationPanel } from "./ApplicationPanel";
import { CourseGrid } from "./CourseGrid";
import { PriorityList } from "./PriorityList";
import { SummaryCard } from "./SummaryCard";
import { dashboardTotals } from "./dashboardTotals";

export function DashboardView({
  application,
  canSubmitApplication,
  courses,
  hasMoreCourses,
  totals,
}: {
  application: TeacherApplication | null;
  canSubmitApplication: boolean;
  courses: TeacherCourseDashboardItem[];
  hasMoreCourses: boolean;
  totals: ReturnType<typeof dashboardTotals>;
}) {
  return (
    <>
      <section className={styles.summaryGrid}>
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Teaching courses" value={totals.courseCount} />
        <SummaryCard icon={<Users size={20} aria-hidden />} label="Enrollment requests" value={totals.pendingEnrollmentCount} tone={totals.pendingEnrollmentCount ? "warn" : "neutral"} />
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Reward reviews" value={totals.pendingRewardCount} tone={totals.pendingRewardCount ? "warn" : "neutral"} />
        <SummaryCard icon={<FileText size={20} aria-hidden />} label="Course content" value={totals.contentCount} />
      </section>

      <section className={styles.twoColumn}>
        <ApplicationPanel application={application} canSubmitApplication={canSubmitApplication} />
        <section className={styles.panel}>
          <div className={styles.panelHeader}>
            <BriefcaseBusiness size={20} aria-hidden />
            <h2>Next teaching work</h2>
          </div>
          <PriorityList totals={totals} />
        </section>
      </section>

      <section className={styles.courseSection}>
        <div className={styles.sectionHeader}>
          <div>
            <h2>Course health</h2>
            <p className={styles.muted}>Lifecycle state, content readiness, enrollment pressure, and reward queues.</p>
          </div>
          {hasMoreCourses ? (
            <Link className={styles.secondaryLink} href="/teach/courses">
              <BookOpen size={17} aria-hidden />
              View all courses
            </Link>
          ) : null}
        </div>
        <CourseGrid courses={courses} emptyDetail="Approved or delegated teaching courses will appear here." />
      </section>
    </>
  );
}

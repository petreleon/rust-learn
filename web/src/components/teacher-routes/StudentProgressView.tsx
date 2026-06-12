"use client";

import { AlertCircle, ArrowLeft, Clock3, FileText, Trophy, Users } from "lucide-react";
import Link from "next/link";
import { type TeacherCourseStudentsResponse } from "@/lib/teacher";
import styles from "../teacher-routes.module.css";
import { StatePanel } from "./StatePanel";
import { StudentProgressCard } from "./StudentProgressCard";
import { SummaryCard } from "./SummaryCard";
import { statusLabel } from "./statusLabel";

export function StudentProgressView({ students }: { students: TeacherCourseStudentsResponse }) {
  const pendingRewardCount = students.students.reduce(
    (total, student) => total + student.rewards.pending_teacher_count,
    0,
  );
  return (
    <>
      <section className={styles.workspaceHero}>
        <Link className={styles.secondaryLink} href={`/teach/courses/${students.course.id}`}>
          <ArrowLeft size={17} aria-hidden />
          Course workspace
        </Link>
        <div className={styles.workspaceTitleBlock}>
          <p className={styles.eyebrow}>{statusLabel(students.course.lifecycle_status)}</p>
          <h2>Student progress</h2>
          <p className={styles.muted}>
            Review enrolled learners, progress support, reward eligibility, and evidence without pretending lesson completion is stored.
          </p>
        </div>
      </section>

      <section className={styles.summaryGrid}>
        <SummaryCard icon={<Users size={20} aria-hidden />} label="Learners" value={students.total} tone={students.total ? "good" : "neutral"} />
        <SummaryCard icon={<FileText size={20} aria-hidden />} label="Course content" value={students.course.content.content_count} tone={students.course.content.has_content ? "good" : "warn"} />
        <SummaryCard icon={<Clock3 size={20} aria-hidden />} label="Pending rewards" value={pendingRewardCount} tone={pendingRewardCount ? "warn" : "neutral"} />
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Reward policies" value={students.reward_eligibility.active_policy_count} tone={students.reward_eligibility.active_policy_count ? "good" : "neutral"} />
      </section>

      <section className={`${styles.warningPanel} ${styles.singlePanel}`}>
        <div className={styles.panelHeader}>
          {students.progress_supported ? <Clock3 size={18} aria-hidden /> : <AlertCircle size={18} aria-hidden />}
          <h2>Progress tracking</h2>
        </div>
        <p>
          {students.progress_supported
            ? "Latest viewed lessons are persisted from learner course routes and refreshed here for course staff."
            : "Lesson completion is not persisted yet. This view shows enrolled learners, reward eligibility, and evidence that already exists."}
        </p>
      </section>

      <section className={styles.courseSection}>
        <div className={styles.sectionHeader}>
          <div>
            <h2>Learners</h2>
            <p className={styles.muted}>Each card separates access, progress support, reward eligibility, and evidence.</p>
          </div>
          <span className={`${styles.statusPill} ${students.total ? styles.good : styles.neutral}`}>
            {students.total} total
          </span>
        </div>
        {students.students.length ? (
          <div className={styles.enrollmentList}>
            {students.students.map((student) => (
              <StudentProgressCard key={student.user.id} student={student} />
            ))}
          </div>
        ) : (
          <StatePanel
            detail="No enrolled learners are visible for this course."
            icon={<Users size={22} aria-hidden />}
            title="No students yet"
          />
        )}
      </section>
    </>
  );
}

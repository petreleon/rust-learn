"use client";

import { type TeacherCourseStudentProgressItem } from "@/lib/teacher";
import styles from "../teacher-routes.module.css";
import { DetailLine } from "./DetailLine";
import { Metric } from "./Metric";
import { formatDateTime } from "./formatDateTime";
import { statusLabel } from "./statusLabel";

export function StudentProgressCard({ student }: { student: TeacherCourseStudentProgressItem }) {
  const latestCandidate = student.rewards.latest_candidate;
  return (
    <article className={styles.enrollmentCard}>
      <div className={styles.courseTop}>
        <div>
          <p className={styles.eyebrow}>{student.user.email}</p>
          <h3>{student.user.name}</h3>
        </div>
        <span className={`${styles.statusPill} ${styles.good}`}>{statusLabel(student.access_state)}</span>
      </div>

      <div className={styles.metricGrid}>
        <Metric label="Content total" value={student.progress.total_content_count} />
        <Metric label="Pending rewards" value={student.rewards.pending_teacher_count} tone={student.rewards.pending_teacher_count ? "warn" : "neutral"} />
        <Metric label="Approved" value={student.rewards.teacher_approved_count + student.rewards.completed_count} />
        <Metric label="Failed" value={student.rewards.failed_count} tone={student.rewards.failed_count ? "warn" : "neutral"} />
      </div>

      <div className={styles.detailList}>
        <DetailLine label="Roles" value={student.roles.join(", ") || "Learner"} />
        <DetailLine label="Latest request" value={student.latest_join_request_status ? statusLabel(student.latest_join_request_status) : "No request history"} />
        <DetailLine label="Lesson progress" value={student.progress.supported ? "Tracked" : student.progress.note} />
        <DetailLine label="Completion" value={student.progress.completion_percentage === null ? "Not tracked yet" : `${student.progress.completion_percentage}%`} />
        <DetailLine label="Reward candidates" value={String(student.rewards.reward_candidate_count)} />
        {latestCandidate ? (
          <>
            <DetailLine label="Latest evidence" value={`${statusLabel(latestCandidate.event_type)} - ${statusLabel(latestCandidate.status)}`} />
            <DetailLine label="Evidence updated" value={formatDateTime(latestCandidate.updated_at)} />
          </>
        ) : (
          <DetailLine label="Latest evidence" value="No reward evidence yet" />
        )}
      </div>
    </article>
  );
}

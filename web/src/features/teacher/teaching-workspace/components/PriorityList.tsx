"use client";

import { ShieldCheck, Trophy, Users } from "lucide-react";
import styles from "@/components/teacher-routes.module.css";
import { type TeacherDashboardTotals } from "../model/dashboardTotals";

export function PriorityList({ totals }: { totals: TeacherDashboardTotals }) {
  const items = [
    {
      detail: totals.pendingEnrollmentCount
        ? "Enrollment requests are waiting for a teacher decision."
        : "Enrollment queue is clear.",
      icon: <Users size={17} aria-hidden />,
      label: "Enrollment queue",
      tone: totals.pendingEnrollmentCount ? "warn" : "good",
      value: totals.pendingEnrollmentCount,
    },
    {
      detail: totals.pendingRewardCount
        ? "Reward candidates are waiting for course-scoped review."
        : "Reward review queue is clear.",
      icon: <Trophy size={17} aria-hidden />,
      label: "Reward review",
      tone: totals.pendingRewardCount ? "warn" : "good",
      value: totals.pendingRewardCount,
    },
    {
      detail: totals.unhealthyCourseCount
        ? "Archived, suspended, or empty courses need attention before learners rely on them."
        : "Visible courses have usable lifecycle and content signals.",
      icon: <ShieldCheck size={17} aria-hidden />,
      label: "Course health",
      tone: totals.unhealthyCourseCount ? "warn" : "good",
      value: totals.unhealthyCourseCount,
    },
  ];

  return (
    <div className={styles.priorityList}>
      {items.map((item) => (
        <article className={styles.priorityItem} key={item.label}>
          <span className={`${styles.smallIcon} ${styles[item.tone]}`}>{item.icon}</span>
          <div>
            <strong>
              {item.value} {item.label}
            </strong>
            <p>{item.detail}</p>
          </div>
        </article>
      ))}
    </div>
  );
}

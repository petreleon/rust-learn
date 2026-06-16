"use client";

import { ShieldCheck } from "lucide-react";
import { type TeacherCourseDashboardItem } from "@/lib/teacher/TeacherCourseDashboardItem";
import styles from "@/features/teacher/shared/teacher-routes.module.css";
import { rewardPolicyCoverageSummary } from "../model/rewardPolicyCoverage";

export function RewardPolicyCoveragePanel({ course }: { course: TeacherCourseDashboardItem }) {
  return (
    <section className={`${styles.warningPanel} ${styles.singlePanel}`}>
      <div className={styles.panelHeader}>
        <ShieldCheck size={18} aria-hidden />
        <h2>Policy coverage</h2>
      </div>
      <p>{rewardPolicyCoverageSummary(course)}</p>
    </section>
  );
}

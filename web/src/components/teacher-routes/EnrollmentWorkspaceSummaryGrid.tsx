"use client";

import { CheckCircle2, Clock3, ShieldCheck, Users } from "lucide-react";
import { type TeacherCourseEnrollmentWorkspaceResponse } from "@/lib/teacher";
import styles from "../teacher-routes.module.css";
import { SummaryCard } from "./SummaryCard";

export function EnrollmentWorkspaceSummaryGrid({
  openRequestCount,
  workspace,
}: {
  openRequestCount: number;
  workspace: TeacherCourseEnrollmentWorkspaceResponse;
}) {
  const pendingShownCount = workspace.join_requests.requests.filter((request) => request.status === "pending").length;
  const activePolicyCount = workspace.reward_eligibility.active_policy_count;

  return (
    <section className={styles.summaryGrid}>
      <SummaryCard
        icon={<Clock3 size={20} aria-hidden />}
        label="Open requests"
        value={openRequestCount}
        tone={openRequestCount ? "warn" : "neutral"}
      />
      <SummaryCard
        icon={<Users size={20} aria-hidden />}
        label="Enrolled learners"
        value={workspace.roster.total}
        tone={workspace.roster.total ? "good" : "neutral"}
      />
      <SummaryCard
        icon={<CheckCircle2 size={20} aria-hidden />}
        label="Pending shown"
        value={pendingShownCount}
        tone="neutral"
      />
      <SummaryCard
        icon={<ShieldCheck size={20} aria-hidden />}
        label="Reward policies"
        value={activePolicyCount}
        tone={activePolicyCount ? "good" : "neutral"}
      />
    </section>
  );
}

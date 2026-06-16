"use client";

import { CheckCircle2, FileText, Search, UserPlus } from "lucide-react";
import { type OrganizationTeacherApplicationList } from "@/lib/organization/OrganizationTeacherApplicationList";
import { SummaryCard } from "@/features/organization/shared/route-kit/SummaryCard";
import styles from "@/features/organization/shared/organization-routes.module.css";

export function TeacherApplicationSummaryGrid({
  applications,
}: {
  applications: OrganizationTeacherApplicationList;
}) {
  return (
    <section className={styles.summaryGrid}>
      <SummaryCard icon={<UserPlus size={20} aria-hidden />} label="All applications" value={applications.summary.total} />
      <SummaryCard icon={<Search size={20} aria-hidden />} label="Matching rows" value={applications.total} />
      <SummaryCard icon={<FileText size={20} aria-hidden />} label="Submitted" value={applications.summary.submitted} />
      <SummaryCard icon={<CheckCircle2 size={20} aria-hidden />} label="Approved" value={applications.summary.approved} />
    </section>
  );
}

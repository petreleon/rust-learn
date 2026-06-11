"use client";

import { CheckCircle2, Clock3, FileText, UserCheck, XCircle } from "lucide-react";
import { type PlatformTeacherApplicationsResponse } from "@/lib/admin";
import styles from "../admin-routes.module.css";
import { SummaryCard } from "./SummaryCard";
import { type SectionState } from "./SectionState";

export function TeacherApplicationSummaryPanel({
  response,
  state,
}: {
  response: PlatformTeacherApplicationsResponse | null;
  state: SectionState;
}) {
  const summary = response?.summary;

  return (
    <section className={styles.summaryGrid} aria-label="Teacher application review summary">
      <SummaryCard icon={<UserCheck size={20} aria-hidden />} label="Submitted" value={summary?.submitted || 0} />
      <SummaryCard icon={<Clock3 size={20} aria-hidden />} label="Needs changes" value={summary?.needs_changes || 0} />
      <SummaryCard icon={<CheckCircle2 size={20} aria-hidden />} label="Approved" value={summary?.approved || 0} />
      <SummaryCard icon={<XCircle size={20} aria-hidden />} label="Rejected" value={summary?.rejected || 0} />
      <SummaryCard icon={<FileText size={20} aria-hidden />} label={state === "loading" ? "Loading" : "Total"} value={summary?.total || 0} />
    </section>
  );
}

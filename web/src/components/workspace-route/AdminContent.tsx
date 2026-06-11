"use client";

import { BookOpen, BriefcaseBusiness, ShieldCheck } from "lucide-react";
import { countDelegatedPermissions, hasPlatformAdminAccess } from "@/lib/access";
import { type CurrentSession } from "@/lib/session";
import styles from "../workspace-route.module.css";
import { ScopeList } from "./ScopeList";
import { SummaryCard } from "./SummaryCard";

export function AdminContent({ session }: { session: CurrentSession }) {
  const hasAdmin = hasPlatformAdminAccess(session);
  return (
    <>
      <section className={styles.contentGrid}>
        <SummaryCard icon={<ShieldCheck size={20} aria-hidden />} label="Platform admin" value={hasAdmin ? "Available" : "Unavailable"} />
        <SummaryCard icon={<BriefcaseBusiness size={20} aria-hidden />} label="Platform permissions" value={session.platform.effective_permissions.length} />
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Delegations" value={countDelegatedPermissions(session)} />
      </section>
      <ScopeList
        empty="Platform permissions will appear here after admin access is granted."
        scopes={[session.platform]}
        title="Platform scope"
      />
    </>
  );
}

import { BookOpen, Building2, ShieldCheck } from "lucide-react";
import Link from "next/link";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import styles from "@/features/session/account-settings/page.module.css";
import { MetricCard } from "./MetricCard";

export function WorkspaceAccessPanel({
  session,
  workspaceSummary,
}: {
  session: CurrentSession;
  workspaceSummary: string;
}) {
  return (
    <article className={styles.panel}>
      <div className={styles.panelHeader}>
        <Building2 size={22} aria-hidden />
        <h2>Workspace access</h2>
      </div>
      <div className={styles.metricGrid}>
        <MetricCard icon={<Building2 size={18} aria-hidden />} label="Organizations" value={session.organizations.length} />
        <MetricCard icon={<BookOpen size={18} aria-hidden />} label="Courses" value={session.courses.length} />
        <MetricCard icon={<ShieldCheck size={18} aria-hidden />} label="Delegations" value={session.delegated_permissions.length} />
      </div>
      <p className={styles.muted}>{workspaceSummary} available from your current session.</p>
      <Link className={styles.secondaryLink} href="/session">View access details</Link>
    </article>
  );
}

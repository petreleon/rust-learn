"use client";

import { ArrowRight, BriefcaseBusiness, RotateCcw } from "lucide-react";
import Link from "next/link";
import { type ReactNode } from "react";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type TeacherApplication } from "@/lib/teacher/TeacherApplication";
import styles from "@/features/teacher/application/page.module.css";
import { ApplicationSummary } from "./ApplicationSummary";
import { ContextRow } from "./ContextRow";

export function ApplicationOverview({
  application,
  canSubmitApplication,
  onStartNewApplication,
  showRejectedForm,
  statusConfig,
  workspaceSummary,
  session,
}: {
  application: TeacherApplication | null;
  canSubmitApplication: boolean;
  onStartNewApplication: () => void;
  showRejectedForm: boolean;
  statusConfig: {
    detail: string;
    heading: string;
    icon: ReactNode;
  };
  workspaceSummary: string;
  session: CurrentSession;
}) {
  return (
    <section className={styles.overviewGrid}>
      <article className={styles.statusPanel}>
        <div className={styles.panelHeader}>
          {statusConfig.icon}
          <h2>{statusConfig.heading}</h2>
        </div>
        <p className={styles.muted}>{statusConfig.detail}</p>
        {application ? <ApplicationSummary application={application} /> : null}
        {application?.status === "rejected" && canSubmitApplication && !showRejectedForm ? (
          <button className={styles.primaryButton} type="button" onClick={onStartNewApplication}>
            <RotateCcw size={17} aria-hidden />
            Start a new application
          </button>
        ) : null}
        {application?.status === "approved" ? (
          <Link className={styles.primaryLink} href="/teach">
            <ArrowRight size={18} aria-hidden />
            Open teaching workspace
          </Link>
        ) : null}
      </article>
      <article className={styles.statusPanel}>
        <div className={styles.panelHeader}>
          <BriefcaseBusiness size={22} aria-hidden />
          <h2>Scope context</h2>
        </div>
        <p className={styles.muted}>
          {workspaceSummary}. Organization and course choices use your session context, so you do not need internal ids.
        </p>
        <div className={styles.contextList}>
          <ContextRow label="Organizations" value={String(session.organizations.length)} />
          <ContextRow label="Courses" value={String(session.courses.length)} />
          <ContextRow label="Delegations" value={String(session.delegated_permissions.length)} />
        </div>
      </article>
    </section>
  );
}

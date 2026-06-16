"use client";

import { AlertCircle, CheckCircle2, Clock3, FileText, Send } from "lucide-react";
import Link from "next/link";
import { type ReactNode } from "react";
import { type TeacherApplication } from "@/lib/teacher/TeacherApplication";
import styles from "@/components/teacher-routes.module.css";

type ApplicationPanelConfig = {
  action: string;
  detail: string;
  href: string | null;
  icon: ReactNode;
  title: string;
};

export function ApplicationPanel({
  application,
  canSubmitApplication,
}: {
  application: TeacherApplication | null;
  canSubmitApplication: boolean;
}) {
  const config = applicationPanelConfig(application, canSubmitApplication);

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        {config.icon}
        <h2>{config.title}</h2>
      </div>
      <p className={styles.muted}>{config.detail}</p>
      {application?.decision_reason ? <p className={styles.reviewNote}>{application.decision_reason}</p> : null}
      {config.href ? (
        <Link className={styles.primaryLink} href={config.href}>
          <Send size={17} aria-hidden />
          {config.action}
        </Link>
      ) : null}
    </section>
  );
}

function applicationPanelConfig(
  application: TeacherApplication | null,
  canSubmitApplication: boolean,
): ApplicationPanelConfig {
  if (!application) {
    return {
      action: "Apply to teach",
      detail: canSubmitApplication
        ? "No teacher application is currently on file for this account."
        : "No teacher application is visible for this account.",
      href: canSubmitApplication ? "/teach/apply" : null,
      icon: <Send size={20} aria-hidden />,
      title: "Teacher application",
    };
  }

  if (application.status === "approved") {
    return {
      action: "View application",
      detail: "Your teacher application is approved. Course-scoped teaching work appears in this dashboard.",
      href: "/teach/apply",
      icon: <CheckCircle2 size={20} aria-hidden />,
      title: "Application approved",
    };
  }

  if (application.status === "rejected") {
    return {
      action: canSubmitApplication ? "Start again" : "View application",
      detail: "The latest teacher application was rejected. Review the decision before starting a fresh application.",
      href: "/teach/apply",
      icon: <AlertCircle size={20} aria-hidden />,
      title: "Application rejected",
    };
  }

  if (application.status === "needs_changes") {
    return {
      action: "View feedback",
      detail: "A reviewer requested changes. The application route shows the reason and current resubmission limits.",
      href: "/teach/apply",
      icon: <FileText size={20} aria-hidden />,
      title: "Changes requested",
    };
  }

  return {
    action: "Track review",
    detail: "Your application is in review. Teaching courses will appear here after scope approval.",
    href: "/teach/apply",
    icon: <Clock3 size={20} aria-hidden />,
    title: "Application submitted",
  };
}

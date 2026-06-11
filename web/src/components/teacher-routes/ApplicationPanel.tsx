"use client";

import { Send } from "lucide-react";
import Link from "next/link";
import { type TeacherApplication } from "@/lib/teacher";
import styles from "../teacher-routes.module.css";
import { applicationConfig } from "./applicationConfig";

export function ApplicationPanel({
  application,
  canSubmitApplication,
}: {
  application: TeacherApplication | null;
  canSubmitApplication: boolean;
}) {
  const config = applicationConfig(application, canSubmitApplication);
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

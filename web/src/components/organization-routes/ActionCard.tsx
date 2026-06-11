"use client";

import { BookOpen, CheckCircle2, CreditCard, FileText, Settings, UserPlus, Users } from "lucide-react";
import Link from "next/link";
import { missingOrganizationPermissions, type OrganizationCapability } from "@/lib/organization";
import styles from "../organization-routes.module.css";
import { actionDescriptions } from "./actionDescriptions";
import { actionIcons } from "./actionIcons";

export function ActionCard({
  capability,
  enabled,
  organizationId,
}: {
  capability: OrganizationCapability;
  enabled: boolean;
  organizationId: number;
}) {
  const coursesHref = `/organizations/${organizationId}/courses`;
  const membersHref = `/organizations/${organizationId}/members`;
  const reportHref = `/organizations/${organizationId}/reports`;
  const settingsHref = `/organizations/${organizationId}/settings`;
  const teacherApplicationsHref = `/organizations/${organizationId}/teacher-applications`;
  const walletHref = `/organizations/${organizationId}/wallet`;

  return (
    <article className={`${styles.actionCard} ${enabled ? styles.enabledAction : styles.deniedAction}`}>
      <div className={styles.actionHeader}>
        <span className={styles.smallIcon}>{actionIcons[capability.key]}</span>
        <div>
          <h3>{capability.label}</h3>
          <p>{actionDescriptions[capability.key]}</p>
        </div>
      </div>
      {enabled ? (
        <span className={styles.statusPill}>
          <CheckCircle2 size={16} aria-hidden />
          Permission available
        </span>
      ) : (
        <div className={styles.missingList}>
          <strong>Missing scoped permission</strong>
          {missingOrganizationPermissions(capability).slice(0, 3).map((permission) => (
            <span key={permission}>{permission}</span>
          ))}
        </div>
      )}
      {enabled && capability.key === "courses" ? (
        <Link className={styles.primaryLink} href={coursesHref}>
          <BookOpen size={17} aria-hidden />
          Open courses
        </Link>
      ) : enabled && capability.key === "members" ? (
        <Link className={styles.primaryLink} href={membersHref}>
          <Users size={17} aria-hidden />
          Open members
        </Link>
      ) : enabled && capability.key === "reports" ? (
        <Link className={styles.primaryLink} href={reportHref}>
          <FileText size={17} aria-hidden />
          Open reports
        </Link>
      ) : enabled && capability.key === "teacher_applications" ? (
        <Link className={styles.primaryLink} href={teacherApplicationsHref}>
          <UserPlus size={17} aria-hidden />
          Open teacher nominations
        </Link>
      ) : enabled && capability.key === "wallet" ? (
        <Link className={styles.primaryLink} href={walletHref}>
          <CreditCard size={17} aria-hidden />
          Open wallet
        </Link>
      ) : enabled && capability.key === "settings" ? (
        <Link className={styles.primaryLink} href={settingsHref}>
          <Settings size={17} aria-hidden />
          Open settings
        </Link>
      ) : (
        <button className={styles.secondaryButton} disabled type="button">
          Contract pending
        </button>
      )}
    </article>
  );
}

"use client";

import { ExternalLink } from "lucide-react";
import { type OrganizationTeacherApplicationItem } from "@/lib/organization";
import styles from "../organization-routes.module.css";
import { Metric } from "./Metric";
import { StatusPill } from "./StatusPill";
import { formatDateTime } from "./formatDateTime";
import { formatUnderscoreLabel } from "./formatUnderscoreLabel";
import { safeExternalHref } from "./safeExternalHref";
import { teacherApplicationStatusTone } from "./teacherApplicationStatusTone";

export function OrganizationTeacherApplicationCard({
  application,
}: {
  application: OrganizationTeacherApplicationItem;
}) {
  const requestedScope =
    application.requested_course?.title ||
    application.requested_organization?.name ||
    formatUnderscoreLabel(application.requested_scope);
  const latestAudit = application.audit.latest_event_type
    ? `${formatUnderscoreLabel(application.audit.latest_event_type)}${
        application.audit.latest_event_at ? ` at ${formatDateTime(application.audit.latest_event_at)}` : ""
      }`
    : "No audit event";

  return (
    <article className={styles.applicationCard}>
      <div className={styles.sectionHeader}>
        <div>
          <h3>{application.applicant.name}</h3>
          <p className={styles.muted}>{application.applicant.email}</p>
        </div>
        <StatusPill label={formatUnderscoreLabel(application.status)} tone={teacherApplicationStatusTone(application.status)} />
      </div>

      <div className={styles.metricGrid}>
        <Metric label="Audit events" value={application.audit.event_count} />
        <Metric label="Portfolio links" value={application.portfolio_links.length} />
        <Metric label="Decided" value={application.decided_at ? "Yes" : "No"} />
      </div>

      <div className={styles.compactList}>
        <div className={styles.compactRow}>
          <span>Requested scope</span>
          <strong>{requestedScope}</strong>
        </div>
        <div className={styles.compactRow}>
          <span>Requested organization</span>
          <strong>{application.requested_organization?.name || "Platform-wide"}</strong>
        </div>
        <div className={styles.compactRow}>
          <span>Latest audit</span>
          <strong>{latestAudit}</strong>
        </div>
        <div className={styles.compactRow}>
          <span>Updated</span>
          <strong>{formatDateTime(application.updated_at)}</strong>
        </div>
      </div>

      <p className={styles.muted}>{application.experience_summary}</p>

      {application.decision_reason || application.reviewer ? (
        <div className={styles.compactList}>
          {application.reviewer ? (
            <div className={styles.compactRow}>
              <span>Reviewer</span>
              <strong>{application.reviewer.name}</strong>
            </div>
          ) : null}
          {application.decision_reason ? (
            <div className={styles.compactRow}>
              <span>Decision reason</span>
              <strong>{application.decision_reason}</strong>
            </div>
          ) : null}
        </div>
      ) : null}

      <div className={styles.permissionRow}>
        {application.sponsored_by_this_organization ? <span className={styles.permissionChip}>Sponsored here</span> : null}
        {application.requested_for_this_organization ? <span className={styles.permissionChip}>Requested here</span> : null}
        {application.audit.latest_reason ? <span className={styles.permissionChip}>Audit reason attached</span> : null}
      </div>

      {application.portfolio_links.length ? (
        <div className={styles.portfolioList}>
          {application.portfolio_links.slice(0, 3).map((link, index) => {
            const href = safeExternalHref(link);
            return href ? (
              <a
                className={styles.secondaryLink}
                href={href}
                key={`${link}-${index}`}
                rel="noreferrer"
                target="_blank"
                title={link}
              >
                <ExternalLink size={16} aria-hidden />
                Portfolio {index + 1}
              </a>
            ) : (
              <span className={styles.permissionChip} key={`${link}-${index}`}>
                Portfolio link unavailable
              </span>
            );
          })}
          {application.portfolio_links.length > 3 ? (
            <span className={styles.permissionChip}>+{application.portfolio_links.length - 3} more</span>
          ) : null}
        </div>
      ) : null}
    </article>
  );
}

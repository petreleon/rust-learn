"use client";

import { type TeacherApplication } from "@/lib/teacher";
import styles from "../page.module.css";
import { ContextRow } from "./ContextRow";
import { PortfolioLink } from "./PortfolioLink";
import { formatDate } from "./formatDate";
import { normalizePortfolioLinks } from "./normalizePortfolioLinks";
import { scopeLabel } from "./scopeLabel";

export function ApplicationSummary({ application }: { application: TeacherApplication }) {
  const portfolioLinks = normalizePortfolioLinks(application.portfolio_links);

  return (
    <div className={styles.applicationSummary}>
      <ContextRow label="Scope" value={scopeLabel(application.requested_scope)} />
      <ContextRow label="Submitted" value={formatDate(application.created_at)} />
      <ContextRow label="Updated" value={formatDate(application.updated_at)} />
      {application.decision_reason ? <ContextRow label="Reviewer note" value={application.decision_reason} /> : null}
      {portfolioLinks.length ? (
        <div className={styles.portfolioBlock}>
          <strong>Portfolio</strong>
          <div className={styles.portfolioLinks}>
            {portfolioLinks.map((link) => (
              <PortfolioLink key={link} link={link} />
            ))}
          </div>
        </div>
      ) : null}
    </div>
  );
}

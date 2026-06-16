"use client";

import { type TeacherApplication } from "@/lib/teacher";
import styles from "@/app/teach/apply/page.module.css";
import { formatDate } from "../model/formatDate";
import { normalizePortfolioLinks } from "../model/normalizePortfolioLinks";
import { scopeLabel } from "../model/scopeLabel";
import { ContextRow } from "./ContextRow";
import { PortfolioLink } from "./PortfolioLink";

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

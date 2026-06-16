"use client";

import { Building2, ClipboardCheck, MailPlus } from "lucide-react";
import Link from "next/link";
import { type ReactNode } from "react";
import { type CurrentSession } from "@/lib/session";
import styles from "@/features/organization/shared/organization-routes.module.css";
import {
  buildOrganizationAccessActions,
  type OrganizationAccessAction,
  type OrganizationAccessActionKey,
} from "../model/organizationAccessPaths";

const actionIcons: Record<OrganizationAccessActionKey, ReactNode> = {
  request_creation: <Building2 size={20} aria-hidden />,
  request_invite: <MailPlus size={20} aria-hidden />,
  review_session: <ClipboardCheck size={20} aria-hidden />,
};

export function OrganizationAccessEmptyState({ session }: { session: CurrentSession }) {
  const actions = buildOrganizationAccessActions(session);

  return (
    <section className={styles.organizationSection} aria-label="Organization access paths" role="status">
      <div className={styles.workspaceHero}>
        <span className={styles.smallIcon}>
          <Building2 size={20} aria-hidden />
        </span>
        <div className={styles.workspaceTitleBlock}>
          <span className={styles.eyebrow}>Access needed</span>
          <h2>No organization workspace yet</h2>
          <p className={styles.muted}>
            Organization workspaces appear after an owner, operator, or platform admin grants access to
            your signed-in account.
          </p>
        </div>
      </div>

      <div className={styles.actionGrid}>
        {actions.map((action) => (
          <OrganizationAccessActionCard action={action} key={action.key} />
        ))}
      </div>
    </section>
  );
}

function OrganizationAccessActionCard({ action }: { action: OrganizationAccessAction }) {
  return (
    <article className={`${styles.actionCard} ${action.primary ? styles.enabledAction : styles.deniedAction}`}>
      <div className={styles.actionHeader}>
        <span className={styles.smallIcon}>{actionIcons[action.key]}</span>
        <div>
          <h3>{action.title}</h3>
          <p>{action.detail}</p>
        </div>
      </div>
      <ActionLink action={action} />
    </article>
  );
}

function ActionLink({ action }: { action: OrganizationAccessAction }) {
  const className = action.primary ? styles.primaryLink : styles.secondaryLink;

  if (action.href.startsWith("mailto:")) {
    return (
      <a className={className} href={action.href}>
        {actionIcons[action.key]}
        {action.cta}
      </a>
    );
  }

  return (
    <Link className={className} href={action.href}>
      {actionIcons[action.key]}
      {action.cta}
    </Link>
  );
}

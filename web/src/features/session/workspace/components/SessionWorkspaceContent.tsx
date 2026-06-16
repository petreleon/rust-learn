"use client";

import { BookOpen, Building2, LogOut, RefreshCw, ShieldCheck, UserCircle } from "lucide-react";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import styles from "@/app/session/page.module.css";
import { CourseCard } from "./CourseCard";
import { EmptyState } from "./EmptyState";
import { OrganizationCard } from "./OrganizationCard";
import { ScopeSummary } from "./ScopeSummary";
import { SectionTitle } from "./SectionTitle";
import { StatusPill } from "./StatusPill";
import { expiryLabel } from "../model/expiryLabel";
import { formatAccessLabel } from "../model/formatAccessLabel";

export function SessionWorkspaceContent({
  onRefresh,
  onSignOut,
  session,
}: {
  onRefresh: () => void;
  onSignOut: () => void;
  session: CurrentSession;
}) {
  return (
    <>
      <section className={styles.grid}>
        <WorkspaceStatusPanel onRefresh={onRefresh} onSignOut={onSignOut} />
        <ProfilePanel session={session} />
      </section>
      <section className={styles.workspaces}>
        <SectionTitle icon={<Building2 size={20} aria-hidden />} title="Organizations" />
        {session.organizations.length ? (
          <div className={styles.cardGrid}>
            {session.organizations.map((organization) => (
              <OrganizationCard key={organization.id} organization={organization} />
            ))}
          </div>
        ) : (
          <EmptyState title="No organization access" detail="Organization scopes will appear here." />
        )}
      </section>
      <section className={styles.workspaces}>
        <SectionTitle icon={<BookOpen size={20} aria-hidden />} title="Courses" />
        {session.courses.length ? (
          <div className={styles.cardGrid}>
            {session.courses.map((course) => (
              <CourseCard key={course.id} course={course} />
            ))}
          </div>
        ) : (
          <EmptyState title="No course access" detail="Course enrollments and teaching scopes will appear here." />
        )}
      </section>
      <DelegatedPermissions session={session} />
    </>
  );
}

function WorkspaceStatusPanel({ onRefresh, onSignOut }: { onRefresh: () => void; onSignOut: () => void }) {
  return (
    <section className={styles.panel} aria-label="Workspace status">
      <div className={styles.panelHeader}>
        <ShieldCheck size={20} aria-hidden />
        <h2>Access status</h2>
      </div>
      <p className={styles.muted}>
        Use this page to inspect the account and workspace scopes that product routes use for navigation and permission gates.
      </p>
      <div className={styles.buttonRow}>
        <button className={styles.secondaryButton} type="button" onClick={onRefresh}>
          <RefreshCw size={18} aria-hidden />
          Refresh
        </button>
        <button className={styles.secondaryButton} type="button" onClick={onSignOut}>
          <LogOut size={18} aria-hidden />
          Sign out
        </button>
      </div>
    </section>
  );
}

function ProfilePanel({ session }: { session: CurrentSession }) {
  return (
    <section className={styles.panel} aria-label="Current user">
      <div className={styles.panelHeader}>
        <UserCircle size={20} aria-hidden />
        <h2>Profile</h2>
      </div>
      <div className={styles.profileBlock}>
        <div>
          <p className={styles.profileName}>{session.user.name}</p>
          <p className={styles.muted}>{session.user.email}</p>
        </div>
        <div className={styles.badgeRow}>
          <StatusPill label={session.user.email_verified ? "Email verified" : "Email pending"} tone={session.user.email_verified ? "good" : "warn"} />
          <StatusPill label={session.user.kyc_verified ? "KYC verified" : "KYC pending"} tone={session.user.kyc_verified ? "good" : "neutral"} />
        </div>
        <ScopeSummary title="Platform" scope={session.platform} />
      </div>
    </section>
  );
}

function DelegatedPermissions({ session }: { session: CurrentSession }) {
  return (
    <section className={styles.workspaces}>
      <SectionTitle icon={<ShieldCheck size={20} aria-hidden />} title="Delegated permissions" />
      {session.delegated_permissions.length ? (
        <div className={styles.delegationList}>
          {session.delegated_permissions.map((delegation) => (
            <article className={styles.delegationItem} key={delegation.id}>
              <div>
                <strong>{formatAccessLabel(delegation.permission)}</strong>
                <p className={styles.muted}>{delegation.organization_name || delegation.course_title || delegation.scope_type}</p>
              </div>
              <StatusPill label={delegation.expires_at ? expiryLabel(delegation.expires_at) : "No expiry"} tone="neutral" />
            </article>
          ))}
        </div>
      ) : (
        <EmptyState title="No active delegations" detail="Temporary permissions will appear with scope context." />
      )}
    </section>
  );
}

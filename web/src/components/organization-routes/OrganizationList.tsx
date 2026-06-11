"use client";

import { Building2, Filter } from "lucide-react";
import Link from "next/link";
import { type OrganizationWorkspaceItem } from "@/lib/organization";
import styles from "../organization-routes.module.css";
import { CapabilityRow } from "./CapabilityRow";
import { Metric } from "./Metric";
import { StatusPill } from "./StatusPill";
import { workspaceSubtitle } from "./workspaceSubtitle";

export function OrganizationList({
  organizations,
  total,
}: {
  organizations: OrganizationWorkspaceItem[];
  total: number;
}) {
  return (
    <section className={styles.organizationSection}>
      <div className={styles.sectionHeader}>
        <h2>Visible organizations</h2>
        <StatusPill label={`${organizations.length} of ${total}`} tone="neutral" />
      </div>
      {organizations.length ? (
        <div className={styles.organizationGrid}>
          {organizations.map((organization) => (
            <article className={styles.organizationCard} key={organization.id}>
              <div className={styles.organizationTop}>
                <span className={styles.smallIcon}>
                  <Building2 size={18} aria-hidden />
                </span>
                <div>
                  <h3>{organization.name}</h3>
                  <p className={styles.muted}>{workspaceSubtitle(organization)}</p>
                </div>
              </div>
              <div className={styles.metricGrid}>
                <Metric label="Permissions" value={organization.effectivePermissionCount} />
                <Metric label="Direct" value={organization.directPermissionCount} />
                <Metric label="Delegated" value={organization.delegatedPermissionCount} />
              </div>
              <CapabilityRow organization={organization} />
              <div className={styles.permissionRow}>
                {organization.roles.length ? (
                  organization.roles.slice(0, 3).map((role) => <span className={styles.permissionChip} key={role}>{role}</span>)
                ) : (
                  <span className={styles.permissionChip}>No role label</span>
                )}
              </div>
              <Link className={styles.primaryLink} href={`/organizations/${organization.id}`}>
                <Building2 size={18} aria-hidden />
                Open dashboard
              </Link>
            </article>
          ))}
        </div>
      ) : (
        <section className={styles.panel}>
          <div className={styles.panelHeader}>
            <Filter size={20} aria-hidden />
            <h2>No matching organizations</h2>
          </div>
          <p className={styles.muted}>
            Adjust the search or capability filter. Your underlying organization access was not
            changed.
          </p>
        </section>
      )}
    </section>
  );
}

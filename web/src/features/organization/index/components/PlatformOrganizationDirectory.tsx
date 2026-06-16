"use client";

import { Building2, RefreshCw, Search } from "lucide-react";
import Link from "next/link";
import { type OrganizationDetail } from "@/lib/organization";
import { type LoadState } from "@/features/organization/shared/route-kit/LoadState";
import { type RouteError } from "@/features/organization/shared/route-kit/RouteError";
import styles from "@/features/organization/shared/organization-routes.module.css";
import { StatusPill } from "@/features/organization/shared/route-kit/StatusPill";

export function PlatformOrganizationDirectory({
  error,
  onChangeSearch,
  onRetry,
  organizations,
  search,
  state,
  total,
}: {
  error: RouteError | null;
  onChangeSearch: (value: string) => void;
  onRetry: () => void;
  organizations: OrganizationDetail[];
  search: string;
  state: LoadState;
  total: number;
}) {
  return (
    <section className={styles.organizationSection} aria-label="Platform organization directory">
      <div className={styles.sectionHeader}>
        <h2>Platform directory</h2>
        <StatusPill label={`${organizations.length} of ${total}`} tone="neutral" />
      </div>
      <section className={styles.filterPanel} aria-label="Platform organization filters">
        <label>
          <span>Search platform organizations</span>
          <span className={styles.inputWithIcon}>
            <Search size={17} aria-hidden />
            <input
              onChange={(event) => onChangeSearch(event.target.value)}
              placeholder="Name, id, or URL"
              type="search"
              value={search}
            />
          </span>
        </label>
        <button className={styles.secondaryButton} onClick={onRetry} type="button">
          <RefreshCw size={17} aria-hidden />
          Refresh
        </button>
      </section>
      {state === "loading" || state === "idle" ? (
        <section className={styles.panel}>
          <div className={styles.panelHeader}>
            <Building2 size={20} aria-hidden />
            <h2>Loading organizations</h2>
          </div>
          <p className={styles.muted}>Resolving the platform organization directory.</p>
        </section>
      ) : null}
      {state === "error" ? (
        <section className={styles.errorBox} role="alert">
          <Building2 size={20} aria-hidden />
          <div>
            <strong>{error?.code ?? "organization_directory_error"}</strong>
            <p>{error?.message ?? "Platform organizations could not be loaded."}</p>
          </div>
        </section>
      ) : null}
      {state === "success" && organizations.length ? (
        <div className={styles.organizationGrid}>
          {organizations.map((organization) => (
            <article className={styles.organizationCard} key={organization.id}>
              <div className={styles.organizationTop}>
                <span className={styles.smallIcon}>
                  <Building2 size={18} aria-hidden />
                </span>
                <div>
                  <h3>{organization.name}</h3>
                  <p className={styles.muted}>Organization {organization.id}</p>
                </div>
              </div>
              <div className={styles.permissionRow}>
                {organization.website_link ? <span className={styles.permissionChip}>{organization.website_link}</span> : null}
                {organization.profile_url ? <span className={styles.permissionChip}>{organization.profile_url}</span> : null}
                {!organization.website_link && !organization.profile_url ? (
                  <span className={styles.permissionChip}>Profile pending</span>
                ) : null}
              </div>
              <Link className={styles.primaryLink} href={`/organizations/${organization.id}`}>
                <Building2 size={18} aria-hidden />
                Open organization
              </Link>
            </article>
          ))}
        </div>
      ) : null}
      {state === "success" && !organizations.length ? (
        <section className={styles.panel}>
          <div className={styles.panelHeader}>
            <Search size={20} aria-hidden />
            <h2>No matching platform organizations</h2>
          </div>
          <p className={styles.muted}>Adjust the directory search or refresh the organization list.</p>
        </section>
      ) : null}
    </section>
  );
}

"use client";

import { Building2, FileText, RefreshCw, Search, ShieldCheck, Users } from "lucide-react";
import { useMemo, useState } from "react";
import { ProductShell } from "@/components/product-shell";
import { buildOrganizationWorkspace, filterOrganizationWorkspace } from "@/lib/organization";
import styles from "@/features/organization/shared/organization-routes.module.css";
import { DeniedState } from "@/components/organization-routes/DeniedState";
import { ErrorState } from "@/components/organization-routes/ErrorState";
import { LoadingState } from "@/components/organization-routes/LoadingState";
import { OrganizationList } from "@/components/organization-routes/OrganizationList";
import { OrganizationStatus } from "@/components/organization-routes/OrganizationStatus";
import { SignedOutState } from "@/components/organization-routes/SignedOutState";
import { SummaryCard } from "@/components/organization-routes/SummaryCard";
import { capabilityFilters } from "@/components/organization-routes/capabilityFilters";
import { emptyWorkspace } from "@/components/organization-routes/emptyWorkspace";
import { type CapabilityFilter } from "@/components/organization-routes/CapabilityFilter";
import { organizationNotice } from "@/components/organization-routes/organizationNotice";
import { useOrganizationSession } from "@/components/organization-routes/useOrganizationSession";

export function OrganizationIndexRoute() {
  const route = useOrganizationSession();
  const [capability, setCapability] = useState<CapabilityFilter>("all");
  const [search, setSearch] = useState("");
  const workspace = useMemo(
    () => (route.session ? buildOrganizationWorkspace(route.session) : emptyWorkspace),
    [route.session],
  );
  const visibleOrganizations = useMemo(
    () => filterOrganizationWorkspace(workspace.organizations, { capability, search }),
    [capability, search, workspace.organizations],
  );
  const notice = organizationNotice(route.error);

  return (
    <ProductShell
      activeNav="organizations"
      breadcrumbs={[{ href: "/session", label: "Workspace" }, { label: "Organizations" }]}
      description="Choose an organization workspace, inspect scoped permissions, and see which operator actions are ready for your session."
      eyebrow="Organization"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={notice}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={<OrganizationStatus workspace={workspace} />}
      title="Organizations"
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState redirect="/organizations" /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <ErrorState error={route.error} redirect="/organizations" /> : null}
      {route.session && workspace.total === 0 ? <DeniedState /> : null}
      {route.session && workspace.total > 0 ? (
        <>
          <section className={styles.summaryGrid}>
            <SummaryCard icon={<Building2 size={20} aria-hidden />} label="Organizations" value={workspace.total} />
            <SummaryCard icon={<FileText size={20} aria-hidden />} label="Report scopes" value={workspace.reportScopeCount} />
            <SummaryCard icon={<Users size={20} aria-hidden />} label="Management scopes" value={workspace.managementScopeCount} />
            <SummaryCard icon={<ShieldCheck size={20} aria-hidden />} label="Delegated org access" value={workspace.delegatedOrganizationCount} />
          </section>
          <section className={styles.filterPanel} aria-label="Organization filters">
            <label>
              <span>Search organizations</span>
              <span className={styles.inputWithIcon}>
                <Search size={17} aria-hidden />
                <input
                  onChange={(event) => setSearch(event.target.value)}
                  placeholder="Name, role, or permission"
                  type="search"
                  value={search}
                />
              </span>
            </label>
            <label>
              <span>Capability</span>
              <select
                aria-label="Organization capability filter"
                onChange={(event) => setCapability(event.target.value as CapabilityFilter)}
                value={capability}
              >
                {capabilityFilters.map((filter) => (
                  <option key={filter.value} value={filter.value}>
                    {filter.label}
                  </option>
                ))}
              </select>
            </label>
            <button
              className={styles.secondaryButton}
              onClick={() => {
                setCapability("all");
                setSearch("");
              }}
              type="button"
            >
              <RefreshCw size={17} aria-hidden />
              Reset
            </button>
          </section>
          <OrganizationList organizations={visibleOrganizations} total={workspace.organizations.length} />
        </>
      ) : null}
    </ProductShell>
  );
}

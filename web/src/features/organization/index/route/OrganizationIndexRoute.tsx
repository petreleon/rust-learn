"use client";

import { Building2, FileText, RefreshCw, Search, ShieldCheck, Users } from "lucide-react";
import { ProductShell } from "@/components/product-shell";
import styles from "@/features/organization/shared/organization-routes.module.css";
import { DeniedState } from "@/features/organization/shared/route-kit/DeniedState";
import { ErrorState } from "@/features/organization/shared/route-kit/ErrorState";
import { LoadingState } from "@/features/organization/shared/route-kit/LoadingState";
import { OrganizationList } from "@/features/organization/shared/route-kit/OrganizationList";
import { OrganizationStatus } from "@/features/organization/shared/route-kit/OrganizationStatus";
import { SignedOutState } from "@/features/organization/shared/route-kit/SignedOutState";
import { SummaryCard } from "@/features/organization/shared/route-kit/SummaryCard";
import { capabilityFilters } from "@/features/organization/shared/route-kit/capabilityFilters";
import { type CapabilityFilter } from "@/features/organization/shared/route-kit/CapabilityFilter";
import { organizationNotice } from "@/features/organization/shared/route-kit/organizationNotice";
import { PlatformOrganizationDirectory } from "../components/PlatformOrganizationDirectory";
import { useOrganizationIndexRoute } from "./useOrganizationIndexRoute";

export function OrganizationIndexRoute() {
  const route = useOrganizationIndexRoute();
  const workspace = route.workspace;
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
      {route.session && workspace.total === 0 && !route.canBrowseDirectory ? <DeniedState /> : null}
      {route.session && route.canBrowseDirectory ? (
        <PlatformOrganizationDirectory
          error={route.directoryError}
          organizations={route.visiblePlatformOrganizations}
          search={route.directorySearch}
          state={route.directoryState}
          total={route.platformOrganizations.length}
          onChangeSearch={route.setDirectorySearch}
          onRetry={() => route.loadPlatformDirectory()}
        />
      ) : null}
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
                  onChange={(event) => route.setSearch(event.target.value)}
                  placeholder="Name, role, or permission"
                  type="search"
                  value={route.search}
                />
              </span>
            </label>
            <label>
              <span>Capability</span>
              <select
                aria-label="Organization capability filter"
                onChange={(event) => route.setCapability(event.target.value as CapabilityFilter)}
                value={route.capability}
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
              onClick={route.clearWorkspaceFilters}
              type="button"
            >
              <RefreshCw size={17} aria-hidden />
              Reset
            </button>
          </section>
          <OrganizationList organizations={route.visibleOrganizations} total={workspace.organizations.length} />
        </>
      ) : null}
    </ProductShell>
  );
}

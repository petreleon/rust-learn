"use client";

import { BriefcaseBusiness, Building2, ShieldCheck } from "lucide-react";
import { countDelegatedPermissions, hasOrganizationAccess } from "@/lib/access";
import { type CurrentSession } from "@/lib/session";
import styles from "../workspace-route.module.css";
import { ScopeList } from "./ScopeList";
import { SummaryCard } from "./SummaryCard";

export function OrganizationContent({ session }: { session: CurrentSession }) {
  const organizations = session.organizations.filter(hasOrganizationAccess);
  return (
    <>
      <section className={styles.contentGrid}>
        <SummaryCard icon={<Building2 size={20} aria-hidden />} label="Organizations" value={organizations.length} />
        <SummaryCard
          icon={<ShieldCheck size={20} aria-hidden />}
          label="Report scopes"
          value={organizations.filter((organization) => organization.effective_permissions.includes("VIEW_ORG_REWARD_REPORTS")).length}
        />
        <SummaryCard icon={<BriefcaseBusiness size={20} aria-hidden />} label="Delegations" value={countDelegatedPermissions(session)} />
      </section>
      <ScopeList
        empty="Organization memberships and delegated organization scopes will appear here."
        scopes={organizations}
        title="Organization scopes"
      />
    </>
  );
}

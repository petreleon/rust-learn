"use client";

import { Building2, FileText, ShieldCheck } from "lucide-react";
import { type OrganizationWorkspaceSummary } from "@/lib/organization";
import { StatusPill } from "./StatusPill";

export function OrganizationStatus({ workspace }: { workspace: OrganizationWorkspaceSummary }) {
  return (
    <>
      <StatusPill
        icon={<Building2 size={16} aria-hidden />}
        label={`${workspace.total} organization${workspace.total === 1 ? "" : "s"}`}
        tone={workspace.total ? "good" : "neutral"}
      />
      <StatusPill
        icon={<FileText size={16} aria-hidden />}
        label={`${workspace.reportScopeCount} report scope${workspace.reportScopeCount === 1 ? "" : "s"}`}
        tone={workspace.reportScopeCount ? "good" : "neutral"}
      />
      <StatusPill
        icon={<ShieldCheck size={16} aria-hidden />}
        label={`${workspace.delegatedOrganizationCount} delegated`}
        tone={workspace.delegatedOrganizationCount ? "warn" : "neutral"}
      />
    </>
  );
}

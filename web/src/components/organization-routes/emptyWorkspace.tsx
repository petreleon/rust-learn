"use client";

import { type OrganizationWorkspaceSummary } from "@/lib/organization";

export const emptyWorkspace: OrganizationWorkspaceSummary = {
  courseRewardScopeCount: 0,
  delegatedOrganizationCount: 0,
  managementScopeCount: 0,
  organizations: [],
  reportScopeCount: 0,
  teacherNominationScopeCount: 0,
  total: 0,
  walletScopeCount: 0,
};

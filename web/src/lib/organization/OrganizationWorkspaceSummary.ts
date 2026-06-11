import { type OrganizationWorkspaceItem } from "./OrganizationWorkspaceItem";

export type OrganizationWorkspaceSummary = {
  courseRewardScopeCount: number;
  delegatedOrganizationCount: number;
  managementScopeCount: number;
  organizations: OrganizationWorkspaceItem[];
  reportScopeCount: number;
  teacherNominationScopeCount: number;
  total: number;
  walletScopeCount: number;
};

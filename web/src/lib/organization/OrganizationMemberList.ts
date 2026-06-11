import { type OrganizationMemberListItem } from "./OrganizationMemberListItem";
import { type OrganizationMemberOperatorPermissions } from "./OrganizationMemberOperatorPermissions";

export type OrganizationMemberList = {
  limit: number;
  members: OrganizationMemberListItem[];
  offset: number;
  operator_permissions: OrganizationMemberOperatorPermissions;
  organization: {
    id: number;
    name: string;
  };
  permission: string | null;
  role: string | null;
  search: string | null;
  total: number;
};

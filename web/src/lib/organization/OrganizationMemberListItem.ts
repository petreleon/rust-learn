export type OrganizationMemberListItem = {
  delegated_permission_count: number;
  delegated_permissions: string[];
  direct_permission_count: number;
  direct_permissions: string[];
  effective_permission_count: number;
  effective_permissions: string[];
  email: string;
  email_verified: boolean;
  id: number;
  joined_at: string;
  kyc_verified: boolean;
  name: string;
  roles: string[];
};

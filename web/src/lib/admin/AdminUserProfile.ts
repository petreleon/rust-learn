export type AdminUserProfile = {
  created_at: string;
  date_of_birth: string | null;
  email: string;
  email_verified: boolean;
  id: number;
  kyc_verified: boolean;
  name: string;
  platform_permissions: string[];
  platform_roles: string[];
};

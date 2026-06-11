export type DelegatedPermissionSession = {
  id: number;
  grantor_user_id: number;
  permission: string;
  scope_type: string;
  organization_id: number | null;
  organization_name: string | null;
  course_id: number | null;
  course_title: string | null;
  course_lifecycle_status: string | null;
  expires_at: string | null;
};

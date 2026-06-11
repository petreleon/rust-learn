import { type CourseSessionScope } from "./CourseSessionScope";
import { type DelegatedPermissionSession } from "./DelegatedPermissionSession";
import { type OrganizationSessionScope } from "./OrganizationSessionScope";
import { type PlatformSessionScope } from "./PlatformSessionScope";

export type CurrentSession = {
  user: {
    id: number;
    name: string;
    email: string;
    email_verified: boolean;
    kyc_verified: boolean;
  };
  platform: PlatformSessionScope;
  organizations: OrganizationSessionScope[];
  courses: CourseSessionScope[];
  delegated_permissions: DelegatedPermissionSession[];
};

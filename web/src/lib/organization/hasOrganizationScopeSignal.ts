import { type OrganizationSessionScope } from "@/lib/session";

export function hasOrganizationScopeSignal(organization: OrganizationSessionScope) {
  return (
    organization.roles.length > 0 ||
    organization.direct_permissions.length > 0 ||
    organization.delegated_permissions.length > 0 ||
    organization.effective_permissions.length > 0
  );
}

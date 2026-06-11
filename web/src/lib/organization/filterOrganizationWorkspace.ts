import { organizationMatchesCapability } from "./organizationMatchesCapability";
import { type OrganizationFilter } from "./OrganizationFilter";
import { type OrganizationWorkspaceItem } from "./OrganizationWorkspaceItem";

export function filterOrganizationWorkspace(
  organizations: OrganizationWorkspaceItem[],
  filter: OrganizationFilter,
) {
  const normalizedSearch = filter.search.trim().toLocaleLowerCase();

  return organizations.filter((organization) => {
    const matchesCapability = organizationMatchesCapability(organization, filter.capability);
    const matchesSearch =
      normalizedSearch.length === 0 ||
      organization.name.toLocaleLowerCase().includes(normalizedSearch) ||
      organization.roles.some((role) => role.toLocaleLowerCase().includes(normalizedSearch)) ||
      organization.effectivePermissions.some((permission) =>
        permission.toLocaleLowerCase().includes(normalizedSearch),
      );

    return matchesCapability && matchesSearch;
  });
}

import { type CurrentSession } from "@/lib/session";
import { type OrganizationDetail } from "@/lib/organization";

export function canBrowsePlatformOrganizations(session: CurrentSession | null) {
  return Boolean(
    session?.access.platform_admin &&
      session.platform.effective_permissions.includes("VIEW_ORGANIZATION"),
  );
}

export function filterPlatformOrganizations(
  organizations: OrganizationDetail[],
  search: string,
) {
  const normalizedSearch = search.trim().toLocaleLowerCase();
  if (!normalizedSearch) return organizations;

  return organizations.filter((organization) =>
    [
      organization.name,
      String(organization.id),
      organization.website_link ?? "",
      organization.profile_url ?? "",
    ].some((value) => value.toLocaleLowerCase().includes(normalizedSearch)),
  );
}

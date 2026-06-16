import { fetchCurrentSession, type CurrentSession } from "@/lib/session";
import { type OrganizationDetail } from "@/lib/organization";
import { organizationJsonRequest } from "@/lib/organization/organizationJsonRequest";

export function loadOrganizationIndexSession({ token }: { token: string }): Promise<CurrentSession> {
  return fetchCurrentSession({ token });
}

export function loadPlatformOrganizations({ token }: { token: string }): Promise<OrganizationDetail[]> {
  return organizationJsonRequest({
    apiRoot: "/api",
    path: "/organizations",
    token,
  });
}

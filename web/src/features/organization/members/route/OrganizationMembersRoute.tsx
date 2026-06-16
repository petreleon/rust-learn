"use client";

import { OrganizationMembersView } from "../view/OrganizationMembersView";
import { useOrganizationMembersRoute } from "./useOrganizationMembersRoute";

export function OrganizationMembersRoute({ organizationId }: { organizationId: string }) {
  const route = useOrganizationMembersRoute(organizationId);
  return <OrganizationMembersView organizationId={organizationId} route={route} />;
}

"use client";

import { OrganizationSettingsView } from "../view/OrganizationSettingsView";
import { useOrganizationSettingsRoute } from "./useOrganizationSettingsRoute";

export function OrganizationSettingsRoute({ organizationId }: { organizationId: string }) {
  const route = useOrganizationSettingsRoute(organizationId);
  return <OrganizationSettingsView organizationId={organizationId} route={route} />;
}

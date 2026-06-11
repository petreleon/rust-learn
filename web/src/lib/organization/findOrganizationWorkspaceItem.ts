import { type CurrentSession } from "@/lib/session";
import { buildOrganizationWorkspace } from "./buildOrganizationWorkspace";
import { type OrganizationWorkspaceItem } from "./OrganizationWorkspaceItem";

export function findOrganizationWorkspaceItem(
  session: CurrentSession,
  organizationId: number,
): OrganizationWorkspaceItem | null {
  return buildOrganizationWorkspace(session).organizations.find((organization) => organization.id === organizationId) || null;
}

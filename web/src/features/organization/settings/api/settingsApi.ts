import { fetchCurrentSession } from "@/lib/session/fetchCurrentSession";
import { deleteOrganization } from "@/lib/organization/deleteOrganization";
import { fetchOrganization } from "@/lib/organization/fetchOrganization";
import { updateOrganization } from "@/lib/organization/updateOrganization";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type OrganizationDetail } from "@/lib/organization/OrganizationDetail";
import { type UpdateOrganizationPayload } from "@/lib/organization/UpdateOrganizationPayload";

export async function loadCurrentOrganizationSession({ token }: { token: string }): Promise<CurrentSession> {
  return fetchCurrentSession({ token });
}

export async function loadOrganizationSettings({
  organizationId,
  token,
}: {
  organizationId: number;
  token: string;
}): Promise<OrganizationDetail> {
  return fetchOrganization({ organizationId, token });
}

export async function saveOrganizationSettings({
  organizationId,
  payload,
  token,
}: {
  organizationId: number;
  payload: UpdateOrganizationPayload;
  token: string;
}): Promise<OrganizationDetail> {
  return updateOrganization({ organizationId, payload, token });
}

export async function deleteOrganizationSettings({
  organizationId,
  token,
}: {
  organizationId: number;
  token: string;
}): Promise<void> {
  return deleteOrganization({ organizationId, token });
}

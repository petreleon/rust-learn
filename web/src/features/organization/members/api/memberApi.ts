import { fetchCurrentSession } from "@/lib/session/fetchCurrentSession";
import { addOrganizationMemberByEmail } from "@/lib/organization/addOrganizationMemberByEmail";
import { assignOrganizationRole } from "@/lib/organization/assignOrganizationRole";
import { fetchOrganizationMembers } from "@/lib/organization/fetchOrganizationMembers";
import { removeOrganizationMember } from "@/lib/organization/removeOrganizationMember";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type OrganizationMemberList } from "@/lib/organization/OrganizationMemberList";

export async function loadCurrentOrganizationSession({ token }: { token: string }): Promise<CurrentSession> {
  return fetchCurrentSession({ token });
}

export async function loadOrganizationMembers({
  limit,
  offset,
  organizationId,
  permission,
  role,
  search,
  token,
}: {
  limit: number;
  offset: number;
  organizationId: number;
  permission: string;
  role: string;
  search: string;
  token: string;
}): Promise<OrganizationMemberList> {
  return fetchOrganizationMembers({ limit, offset, organizationId, permission, role, search, token });
}

export async function assignMemberRole({
  memberId,
  organizationId,
  roleName,
  token,
}: {
  memberId: number;
  organizationId: number;
  roleName: string;
  token: string;
}) {
  return assignOrganizationRole({ organizationId, payload: { roleName, userId: memberId }, token });
}

export async function removeMemberFromOrganization({
  memberId,
  organizationId,
  token,
}: {
  memberId: number;
  organizationId: number;
  token: string;
}) {
  return removeOrganizationMember({ organizationId, token, userId: memberId });
}

export async function addMemberByEmail({
  email,
  organizationId,
  roleName,
  token,
}: {
  email: string;
  organizationId: number;
  roleName?: string;
  token: string;
}) {
  return addOrganizationMemberByEmail({ email, organizationId, roleName, token });
}

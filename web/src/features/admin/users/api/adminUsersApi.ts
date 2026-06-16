import {
  assignAdminUserRole,
  fetchAdminPlatformRoles,
  fetchAdminUser,
  fetchAdminUsers,
  type AdminRole,
  type AdminUserProfile,
  type AdminUsersResponse,
} from "@/lib/admin";
import { fetchCurrentSession, type CurrentSession } from "@/lib/session";

export function loadAdminUsersSession({ token }: { token: string }): Promise<CurrentSession> {
  return fetchCurrentSession({ token });
}

export function searchAdminUsers({
  search,
  token,
}: {
  search: string;
  token: string;
}): Promise<AdminUsersResponse> {
  return fetchAdminUsers({ search, token });
}

export function loadAdminUserProfile({
  token,
  userId,
}: {
  token: string;
  userId: number;
}): Promise<AdminUserProfile> {
  return fetchAdminUser({ token, userId });
}

export function loadAdminPlatformRoles({ token }: { token: string }): Promise<AdminRole[]> {
  return fetchAdminPlatformRoles({ token });
}

export function assignRoleToAdminUser({
  roleName,
  token,
  userId,
}: {
  roleName: string;
  token: string;
  userId: number;
}): Promise<string> {
  return assignAdminUserRole({ roleName, token, userId });
}

import { adminJsonRequest } from "./adminJsonRequest";
import { type AdminUserProfile } from "./AdminUserProfile";

export function fetchAdminUser({
  token,
  userId,
}: {
  token: string;
  userId: number;
}): Promise<AdminUserProfile> {
  return adminJsonRequest({
    path: `/user/${userId}`,
    token,
  });
}

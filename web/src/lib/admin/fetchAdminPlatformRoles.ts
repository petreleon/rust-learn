import { adminJsonRequest } from "./adminJsonRequest";
import { type AdminRole } from "./AdminRole";

export function fetchAdminPlatformRoles({ token }: { token: string }): Promise<AdminRole[]> {
  return adminJsonRequest({
    path: "/roles",
    token,
  });
}

import { adminJsonRequest } from "./adminJsonRequest";
import { type AdminUsersResponse } from "./AdminUsersResponse";

export function fetchAdminUsers({
  search,
  token,
}: {
  search: string;
  token: string;
}): Promise<AdminUsersResponse> {
  const params = new URLSearchParams();
  const trimmedSearch = search.trim();
  if (trimmedSearch) params.set("search", trimmedSearch);
  const suffix = params.toString();

  return adminJsonRequest({
    path: `/user${suffix ? `?${suffix}` : ""}`,
    token,
  });
}

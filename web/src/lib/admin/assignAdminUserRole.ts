import { adminRawRequest } from "./adminRawRequest";

export async function assignAdminUserRole({
  roleName,
  token,
  userId,
}: {
  roleName: string;
  token: string;
  userId: number;
}): Promise<string> {
  const response = await adminRawRequest({
    accept: "text/plain",
    body: JSON.stringify({ role_name: roleName }),
    method: "POST",
    path: `/user/${userId}/role`,
    token,
  });

  return response.text();
}

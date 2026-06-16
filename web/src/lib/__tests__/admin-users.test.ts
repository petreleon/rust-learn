import { afterEach, describe, expect, it, vi } from "vitest";
import { fetchAdminUserRoleAssignmentAudit } from "@/lib/admin/fetchAdminUserRoleAssignmentAudit";

function mockJson(body: unknown) {
  const fetchMock = vi.fn(async () => new Response(JSON.stringify(body), { status: 200 }));
  vi.stubGlobal("fetch", fetchMock);
  return fetchMock;
}

describe("admin user helpers", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("fetches role assignment audit history for one user", async () => {
    const audit = [
      {
        actor_user_id: 1,
        created_at: "2026-06-16T10:00:00Z",
        event_type: "role_assigned",
        id: 11,
        platform_role_id: 3,
        role_name: "PLATFORM_ADMIN",
        target_user_id: 7,
      },
    ];
    const fetchMock = mockJson(audit);

    const result = await fetchAdminUserRoleAssignmentAudit({
      apiRoot: "http://api.test",
      token: "admin-token",
      userId: 7,
    });

    expect(result).toEqual(audit);
    expect(fetchMock).toHaveBeenCalledWith(
      "http://api.test/user/7/role/audit",
      expect.objectContaining({
        headers: expect.objectContaining({ Authorization: "Bearer admin-token" }),
        method: "GET",
      }),
    );
  });
});

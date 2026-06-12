import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { AdminRequestError } from "@/lib/admin/AdminRequestError";
import { fetchDelegations } from "@/lib/admin/fetchDelegations";
import { type DelegationItem } from "@/lib/admin/DelegationItem";

function makeDelegation(overrides: Partial<DelegationItem> = {}): DelegationItem {
  return {
    course_id: null,
    created_at: "2026-06-12T09:00:00Z",
    expires_at: null,
    grantee_user_id: 12,
    grantor_user_id: 4,
    id: 1,
    organization_id: null,
    permission: "VIEW_REWARD_AUDIT",
    reason: null,
    revoked_at: null,
    revoked_by_user_id: null,
    revoke_reason: null,
    scope_type: "platform",
    updated_at: "2026-06-12T09:00:00Z",
    ...overrides,
  };
}

function mockJson(body: unknown) {
  const fetchMock = vi.fn(async () => new Response(JSON.stringify(body), { status: 200 }));
  vi.stubGlobal("fetch", fetchMock);
  return fetchMock;
}

describe("fetchDelegations", () => {
  beforeEach(() => {
    vi.useRealTimers();
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("normalizes the backend raw-array response into the route contract", async () => {
    const row = makeDelegation({ id: 7 });
    const fetchMock = mockJson([row]);

    const result = await fetchDelegations({
      active: true,
      apiRoot: "http://api.test",
      limit: 2,
      offset: 4,
      token: "admin-token",
    });

    expect(result).toEqual({ delegations: [row], limit: 2, offset: 4, total: 1 });
    expect(fetchMock).toHaveBeenCalledWith(
      "http://api.test/delegated-permissions?active=true&limit=2&offset=4",
      expect.objectContaining({
        headers: expect.objectContaining({ Authorization: "Bearer admin-token" }),
        method: "GET",
      }),
    );
  });

  it("preserves paginated delegation responses when the API returns metadata", async () => {
    const row = makeDelegation({ id: 3, scope_type: "course" });
    mockJson({ delegations: [row], limit: 50, offset: 10, total: 72 });

    const result = await fetchDelegations({
      apiRoot: "http://api.test",
      limit: 50,
      offset: 10,
      token: "admin-token",
    });

    expect(result).toEqual({ delegations: [row], limit: 50, offset: 10, total: 72 });
  });

  it("normalizes empty raw-array responses", async () => {
    mockJson([]);

    const result = await fetchDelegations({
      apiRoot: "http://api.test",
      limit: 100,
      token: "admin-token",
    });

    expect(result).toEqual({ delegations: [], limit: 100, offset: 0, total: 0 });
  });

  it("rejects malformed delegation responses before the route can crash", async () => {
    mockJson({ items: [makeDelegation({ id: 9 })] });

    await expect(
      fetchDelegations({
        apiRoot: "http://api.test",
        token: "admin-token",
      }),
    ).rejects.toMatchObject<Partial<AdminRequestError>>({
      code: "invalid_response",
      message: "Delegation list response was malformed.",
    });
  });
});

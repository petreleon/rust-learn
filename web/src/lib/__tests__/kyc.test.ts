import { afterEach, describe, expect, it, vi } from "vitest";
import { fetchKycStatus, submitKyc } from "@/lib/kyc";

function mockJson(body: unknown, ok = true, status = 200) {
  return vi.fn().mockResolvedValue({
    json: async () => body,
    ok,
    status,
    text: async () => (typeof body === "string" ? body : JSON.stringify(body)),
  });
}

describe("KYC helpers", () => {
  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("fetches the signed-in user's KYC status", async () => {
    const fetchMock = mockJson({ next_action: "submit", submission: null, user_kyc_verified: false });
    vi.stubGlobal("fetch", fetchMock);

    const status = await fetchKycStatus({ apiRoot: "http://api.test", token: "token" });

    expect(status.next_action).toBe("submit");
    expect(fetchMock).toHaveBeenCalledWith("http://api.test/kyc/me", expect.objectContaining({
      headers: { Authorization: "Bearer token" },
      method: "GET",
    }));
  });

  it("submits KYC details as JSON", async () => {
    const fetchMock = mockJson({ next_action: "wait_for_review", submission: { id: 1 }, user_kyc_verified: false }, true, 201);
    vi.stubGlobal("fetch", fetchMock);

    await submitKyc({
      apiRoot: "http://api.test",
      payload: { country_code: "US", document_type: "passport", legal_name: "Learner User" },
      token: "token",
    });

    expect(fetchMock).toHaveBeenCalledWith("http://api.test/kyc/me", expect.objectContaining({
      body: JSON.stringify({ country_code: "US", document_type: "passport", legal_name: "Learner User" }),
      headers: { Authorization: "Bearer token", "Content-Type": "application/json" },
      method: "POST",
    }));
  });

  it("raises request errors with status", async () => {
    vi.stubGlobal("fetch", mockJson("bad request", false, 400));

    await expect(fetchKycStatus({ token: "token" })).rejects.toMatchObject({
      message: "bad request",
      status: 400,
    });
  });
});

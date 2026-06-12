import { afterEach, describe, expect, it, vi } from "vitest";
import { decideKycSubmission } from "@/lib/admin/decideKycSubmission";
import { fetchKycReviewQueue } from "@/lib/admin/fetchKycReviewQueue";
import { type KycSubmission } from "@/lib/admin/KycSubmission";

function submission(overrides: Partial<KycSubmission> = {}): KycSubmission {
  return {
    country_code: "US",
    created_at: "2026-06-12T09:00:00Z",
    document_last4: "1234",
    document_type: "passport",
    evidence_reference: "s3://kyc/evidence-1",
    id: 7,
    legal_name: "Learner User",
    provider_reference: null,
    rejection_reason: null,
    reviewed_at: null,
    reviewer_user_id: null,
    status: "submitted",
    submitted_at: "2026-06-12T09:00:00Z",
    updated_at: "2026-06-12T09:00:00Z",
    user_id: 42,
    ...overrides,
  };
}

function mockJson(body: unknown, status = 200) {
  const fetchMock = vi.fn(async () => new Response(JSON.stringify(body), { status }));
  vi.stubGlobal("fetch", fetchMock);
  return fetchMock;
}

describe("admin KYC helpers", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("fetches the platform KYC review queue", async () => {
    const row = submission();
    const fetchMock = mockJson({ submissions: [row] });

    const queue = await fetchKycReviewQueue({ apiRoot: "http://api.test", token: "admin-token" });

    expect(queue.submissions).toEqual([row]);
    expect(fetchMock).toHaveBeenCalledWith(
      "http://api.test/kyc/review",
      expect.objectContaining({
        headers: expect.objectContaining({ Authorization: "Bearer admin-token" }),
        method: "GET",
      }),
    );
  });

  it("sends approve and reject decisions to the submission endpoint", async () => {
    const fetchMock = mockJson(submission({ status: "verified" }));

    await decideKycSubmission({
      apiRoot: "http://api.test",
      rejectionReason: "  not needed  ",
      status: "verified",
      submissionId: 7,
      token: "admin-token",
    });

    expect(fetchMock).toHaveBeenCalledWith(
      "http://api.test/kyc/review/7",
      expect.objectContaining({
        body: JSON.stringify({ rejection_reason: null, status: "verified" }),
        method: "PUT",
      }),
    );

    await decideKycSubmission({
      apiRoot: "http://api.test",
      rejectionReason: "ID expired",
      status: "rejected",
      submissionId: 7,
      token: "admin-token",
    });

    expect(fetchMock).toHaveBeenLastCalledWith(
      "http://api.test/kyc/review/7",
      expect.objectContaining({
        body: JSON.stringify({ rejection_reason: "ID expired", status: "rejected" }),
        method: "PUT",
      }),
    );
  });
});

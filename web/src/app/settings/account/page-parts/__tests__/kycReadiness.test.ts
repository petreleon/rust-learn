import { describe, expect, it } from "vitest";
import { type KycStatusResponse } from "@/lib/kyc";
import { kycReadinessCopy } from "../kycReadiness";

function status(submissionStatus: string, reason: string | null = null): KycStatusResponse {
  return {
    next_action: ["rejected", "expired", "provider_error"].includes(submissionStatus) ? "resubmit" : "wait_for_review",
    submission: {
      country_code: "US",
      document_last4: null,
      document_type: "passport",
      evidence_reference: null,
      id: 1,
      legal_name: "Learner User",
      provider_reference: null,
      rejection_reason: reason,
      status: submissionStatus,
    },
    user_kyc_verified: false,
  };
}

describe("kycReadinessCopy", () => {
  it.each([
    [null, false, "KYC not started", "neutral"],
    [status("submitted"), false, "KYC submitted", "neutral"],
    [status("under_review"), false, "KYC under review", "neutral"],
    [status("rejected", "Document expired"), false, "KYC rejected", "warn"],
    [status("expired"), false, "KYC expired", "warn"],
    [status("provider_error"), false, "KYC provider issue", "warn"],
    [null, true, "KYC is verified", "good"],
  ])("returns visible copy for %s", (input, verified, label, tone) => {
    expect(kycReadinessCopy(input, verified)).toMatchObject({ label, tone });
  });

  it("surfaces the latest rejection reason", () => {
    expect(kycReadinessCopy(status("rejected", "Passport is unreadable."), false).detail).toBe("Passport is unreadable.");
  });
});

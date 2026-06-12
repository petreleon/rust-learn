import { type KycStatusResponse } from "@/lib/kyc";

export type KycReadinessCopy = {
  detail: string;
  label: string;
  tone: "good" | "neutral" | "warn";
};

export function kycReadinessCopy(
  status: KycStatusResponse | null,
  verified: boolean,
): KycReadinessCopy {
  if (verified || status?.user_kyc_verified || status?.next_action === "verified") {
    return {
      detail: "Identity checks are complete for gated wallet or payout flows.",
      label: "KYC is verified",
      tone: "good",
    };
  }

  switch (status?.submission?.status) {
    case "submitted":
      return {
        detail: "Your KYC request was submitted and is waiting for platform intake.",
        label: "KYC submitted",
        tone: "neutral",
      };
    case "under_review":
      return {
        detail: "Your KYC request is under platform review.",
        label: "KYC under review",
        tone: "neutral",
      };
    case "rejected":
      return {
        detail: status.submission.rejection_reason || "Update and resubmit your KYC details.",
        label: "KYC rejected",
        tone: "warn",
      };
    case "expired":
      return {
        detail: "Your previous identity check expired. Submit updated details before wallet actions.",
        label: "KYC expired",
        tone: "warn",
      };
    case "provider_error":
      return {
        detail: "The identity provider could not complete the check. Review the details and resubmit.",
        label: "KYC provider issue",
        tone: "warn",
      };
    default:
      return {
        detail: "Submit identity details for platform review before KYC-gated wallet operations need them.",
        label: "KYC not started",
        tone: "neutral",
      };
  }
}

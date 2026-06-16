"use client";

import { useSearchParams } from "next/navigation";
import { VerifyEmailView } from "../view/VerifyEmailView";
import { useResendVerificationController } from "./useResendVerificationController";
import { useVerifyEmailController } from "./useVerifyEmailController";

export function VerifyEmailContent() {
  const searchParams = useSearchParams();
  const queryToken = searchParams.get("token") || "";
  const verification = useVerifyEmailController(queryToken);
  const resendVerification = useResendVerificationController();
  return <VerifyEmailView {...verification} resendVerification={resendVerification} />;
}

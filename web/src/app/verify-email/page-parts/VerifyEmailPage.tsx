"use client";

import { Suspense } from "react";
import { VerifyEmailContent } from "./VerifyEmailContent";
import { VerifyEmailFallback } from "./VerifyEmailFallback";

export default function VerifyEmailPage() {
  return (
    <Suspense fallback={<VerifyEmailFallback />}>
      <VerifyEmailContent />
    </Suspense>
  );
}

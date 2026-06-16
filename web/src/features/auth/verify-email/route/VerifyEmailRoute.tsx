"use client";

import { Suspense } from "react";
import { VerifyEmailContent } from "./VerifyEmailContent";
import { VerifyEmailFallback } from "../components/VerifyEmailFallback";

export default function VerifyEmailRoute() {
  return (
    <Suspense fallback={<VerifyEmailFallback />}>
      <VerifyEmailContent />
    </Suspense>
  );
}

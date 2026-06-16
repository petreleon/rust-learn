"use client";

import { type FormEvent, useMemo, useState } from "react";
import { AuthRequestError, requestEmailVerification } from "@/lib/auth";
import { type AuthFormError } from "../../shared/model/AuthFormError";
import { type ResendVerificationSubmitState } from "../model/ResendVerificationSubmitState";

export function useResendVerificationController() {
  const [email, setEmail] = useState("");
  const [submitState, setSubmitState] = useState<ResendVerificationSubmitState>("idle");
  const [message, setMessage] = useState<string | null>(null);
  const [error, setError] = useState<AuthFormError | null>(null);

  const canSubmit = useMemo(() => email.trim().length > 0 && submitState !== "loading", [email, submitState]);

  async function submitResend(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const normalizedEmail = email.trim();
    if (!normalizedEmail) {
      setError({ code: "missing_email", message: "Email is required." });
      setSubmitState("error");
      return;
    }

    setSubmitState("loading");
    setError(null);
    setMessage(null);
    try {
      setMessage(await requestEmailVerification({ email: normalizedEmail }));
      setSubmitState("success");
    } catch (nextError) {
      const requestError =
        nextError instanceof AuthRequestError
          ? nextError
          : new AuthRequestError("Verification email request failed.", 0, "network_error");
      setError({ code: requestError.code, message: requestError.message });
      setSubmitState("error");
    }
  }

  return { canSubmit, email, error, message, setEmail, submitResend, submitState };
}

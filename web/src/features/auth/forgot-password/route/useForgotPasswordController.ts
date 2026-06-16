"use client";

import { type FormEvent, useMemo, useState } from "react";
import { AuthRequestError, requestPasswordReset } from "@/lib/auth";
import { type AuthFormError } from "../../shared/model/AuthFormError";
import { type ForgotPasswordSubmitState } from "../model/ForgotPasswordSubmitState";

export function useForgotPasswordController() {
  const [email, setEmail] = useState("");
  const [submitState, setSubmitState] = useState<ForgotPasswordSubmitState>("idle");
  const [message, setMessage] = useState<string | null>(null);
  const [error, setError] = useState<AuthFormError | null>(null);

  const canSubmit = useMemo(() => {
    return email.trim().length > 0 && submitState !== "loading";
  }, [email, submitState]);

  async function submitRequest(event: FormEvent<HTMLFormElement>) {
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
      setMessage(await requestPasswordReset({ email: normalizedEmail }));
      setSubmitState("success");
    } catch (nextError) {
      const requestError =
        nextError instanceof AuthRequestError
          ? nextError
          : new AuthRequestError("Password reset request failed.", 0, "network_error");
      setError({ code: requestError.code, message: requestError.message });
      setSubmitState("error");
    }
  }

  return { canSubmit, email, error, message, setEmail, submitRequest, submitState };
}

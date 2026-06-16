"use client";

import { type FormEvent, useMemo, useState } from "react";
import { AuthRequestError, resetPassword } from "@/lib/auth";
import { type AuthFormError } from "../../shared/model/AuthFormError";
import { evaluatePasswordPolicy } from "../../shared/model/passwordPolicy";
import { type ResetPasswordSubmitState } from "../model/ResetPasswordSubmitState";

export function useResetPasswordController() {
  const [token, setToken] = useState(() => {
    if (typeof window === "undefined") {
      return "";
    }
    return new URLSearchParams(window.location.search).get("token") || "";
  });
  const [password, setPassword] = useState("");
  const [submitState, setSubmitState] = useState<ResetPasswordSubmitState>("idle");
  const [message, setMessage] = useState<string | null>(null);
  const [error, setError] = useState<AuthFormError | null>(null);

  const policyResults = useMemo(() => evaluatePasswordPolicy(password), [password]);
  const canSubmit = token.trim().length > 0 && password.length > 0 && submitState !== "loading";

  async function submitReset(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!token.trim() || !password) {
      setError({ code: "missing_fields", message: "Reset token and new password are required." });
      setSubmitState("error");
      return;
    }

    setSubmitState("loading");
    setError(null);
    setMessage(null);
    try {
      setMessage(await resetPassword({ password, token: token.trim() }));
      setPassword("");
      setSubmitState("success");
    } catch (nextError) {
      const requestError =
        nextError instanceof AuthRequestError
          ? nextError
          : new AuthRequestError("Password reset failed.", 0, "network_error");
      setError({ code: requestError.code, message: requestError.message });
      setSubmitState("error");
    }
  }

  return {
    canSubmit,
    error,
    message,
    password,
    policyResults,
    setPassword,
    setToken,
    submitReset,
    submitState,
    token,
  };
}

"use client";

import { type FormEvent, useMemo, useState } from "react";
import { useRouter } from "next/navigation";
import { AuthRequestError, loginWithPassword } from "@/lib/auth";
import { storeSessionToken } from "@/lib/session";
import { type AuthFormError } from "../../shared/model/AuthFormError";
import { intendedLoginRoute } from "../model/intendedLoginRoute";
import { type LoginSubmitState } from "../model/LoginSubmitState";

export function useLoginController() {
  const router = useRouter();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [submitState, setSubmitState] = useState<LoginSubmitState>("idle");
  const [error, setError] = useState<AuthFormError | null>(null);

  const canSubmit = useMemo(() => {
    return email.trim().length > 0 && password.length > 0 && submitState !== "loading";
  }, [email, password, submitState]);

  async function submitLogin(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const normalizedEmail = email.trim();

    if (!normalizedEmail || !password) {
      setError({ code: "missing_fields", message: "Email and password are required." });
      setSubmitState("error");
      return;
    }

    setSubmitState("loading");
    setError(null);

    try {
      const token = await loginWithPassword({ email: normalizedEmail, password });
      storeSessionToken(token);
      router.replace(intendedLoginRoute());
    } catch (nextError) {
      const requestError =
        nextError instanceof AuthRequestError
          ? nextError
          : new AuthRequestError("Login failed.", 0, "network_error");
      setError({ code: requestError.code, message: requestError.message });
      setSubmitState("error");
    }
  }

  return {
    canSubmit,
    email,
    error,
    password,
    setEmail,
    setPassword,
    setShowPassword,
    showPassword,
    submitLogin,
    submitState,
  };
}

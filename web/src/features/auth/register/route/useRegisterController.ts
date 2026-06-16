"use client";

import { type FormEvent, useMemo, useState } from "react";
import { AuthRequestError, registerAccount } from "@/lib/auth";
import { type AuthFormError } from "../../shared/model/AuthFormError";
import { evaluatePasswordPolicy } from "../../shared/model/passwordPolicy";
import { type RegisterSubmitState } from "../model/RegisterSubmitState";

export function useRegisterController() {
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [dateOfBirth, setDateOfBirth] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [submitState, setSubmitState] = useState<RegisterSubmitState>("idle");
  const [error, setError] = useState<AuthFormError | null>(null);

  const policyResults = useMemo(() => evaluatePasswordPolicy(password), [password]);
  const passwordMeetsPolicy = policyResults.every((check) => check.met);
  const canSubmit =
    name.trim().length > 0 &&
    email.trim().length > 0 &&
    password.length > 0 &&
    submitState !== "loading";

  async function submitRegistration(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const normalizedName = name.trim();
    const normalizedEmail = email.trim();
    if (!normalizedName || !normalizedEmail || !password) {
      setError({ code: "missing_fields", message: "Name, email, and password are required." });
      setSubmitState("error");
      return;
    }
    if (!passwordMeetsPolicy) {
      setError({ code: "password_policy", message: "Password does not meet the policy." });
      setSubmitState("error");
      return;
    }

    setSubmitState("loading");
    setError(null);
    try {
      await registerAccount({ dateOfBirth, email: normalizedEmail, name: normalizedName, password });
      setSubmitState("success");
      setPassword("");
    } catch (nextError) {
      const requestError =
        nextError instanceof AuthRequestError
          ? nextError
          : new AuthRequestError("Registration failed.", 0, "network_error");
      setError({ code: requestError.code, message: requestError.message });
      setSubmitState("error");
    }
  }

  return {
    canSubmit,
    dateOfBirth,
    email,
    error,
    name,
    password,
    policyResults,
    setDateOfBirth,
    setEmail,
    setName,
    setPassword,
    setShowPassword,
    showPassword,
    submitRegistration,
    submitState,
  };
}

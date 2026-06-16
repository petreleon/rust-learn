"use client";

import { type FormEvent, useCallback, useEffect, useState } from "react";
import { AuthRequestError, verifyEmailToken, type VerifyEmailResult } from "@/lib/auth";
import { type AuthFormError } from "../../shared/model/AuthFormError";
import { type VerifyState } from "../model/VerifyState";

export function useVerifyEmailController(queryToken: string) {
  const [token, setToken] = useState(queryToken);
  const [verifiedToken, setVerifiedToken] = useState("");
  const [verifyState, setVerifyState] = useState<VerifyState>("idle");
  const [result, setResult] = useState<VerifyEmailResult | null>(null);
  const [error, setError] = useState<AuthFormError | null>(null);

  const runVerification = useCallback(async (nextToken: string) => {
    setVerifyState("loading");
    setError(null);
    setResult(null);

    try {
      const nextResult = await verifyEmailToken({ token: nextToken });
      setResult(nextResult);
      setVerifiedToken(nextToken);
      setVerifyState("success");
    } catch (nextError) {
      const requestError =
        nextError instanceof AuthRequestError
          ? nextError
          : new AuthRequestError("Email verification failed.", 0, "network_error");
      setError({ code: requestError.code, message: requestError.message });
      setVerifyState("error");
    }
  }, []);

  useEffect(() => {
    if (!queryToken || queryToken === verifiedToken) {
      return undefined;
    }

    const timeout = window.setTimeout(() => {
      setToken(queryToken);
      void runVerification(queryToken);
    }, 0);

    return () => window.clearTimeout(timeout);
  }, [queryToken, runVerification, verifiedToken]);

  function submitVerification(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    void runVerification(token);
  }

  return { error, result, setToken, submitVerification, token, verifyState };
}

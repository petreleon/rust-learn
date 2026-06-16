"use client";

import { useEffect } from "react";
import { useRouter } from "next/navigation";
import {
  clearStoredSessionToken,
  fetchCurrentSession,
  readStoredSessionToken,
} from "@/lib/session";

export function useProductEntryRoute() {
  const router = useRouter();

  useEffect(() => {
    const timeout = window.setTimeout(async () => {
      const token = readStoredSessionToken();
      if (!token) {
        router.replace("/login");
        return;
      }

      try {
        await fetchCurrentSession({ token });
        router.replace("/session");
      } catch {
        clearStoredSessionToken();
        router.replace("/login");
      }
    }, 0);

    return () => window.clearTimeout(timeout);
  }, [router]);
}

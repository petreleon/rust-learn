"use client";

import { type ShellNotice } from "@/components/product-shell";
import { type RouteError } from "./RouteError";

export function routeNotice(error: RouteError | null): ShellNotice | null {
  if (!error) {
    return null;
  }

  return {
    actionHref: error.status === 401 ? "/login?redirect=/teach" : undefined,
    actionLabel: error.status === 401 ? "Sign in" : undefined,
    message: error.message,
    title: "Teaching workspace unavailable",
    tone: "error",
  };
}

"use client";
export function loginHref(redirect: string) {
  return `/login?redirect=${encodeURIComponent(redirect)}`;
}

"use client";
export function flowStatus(hasAccess: boolean, hasSessionToken: boolean, hasServerDenial = false) {
  if (!hasAccess) {
    return "Locked";
  }
  if (!hasSessionToken) {
    return "Needs JWT";
  }
  return hasServerDenial ? "Limited" : "Ready";
}

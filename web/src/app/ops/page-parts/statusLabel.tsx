"use client";

import { type ApiState } from "./ApiState";

export function statusLabel(status: ApiState) {
  if (status === "online") {
    return "Online";
  }
  if (status === "offline") {
    return "Offline";
  }
  return "Checking";
}

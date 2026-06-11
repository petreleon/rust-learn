"use client";

import { API_REQUEST_TIMEOUT_MS } from "./API_REQUEST_TIMEOUT_MS";

export async function fetchWithTimeout(input: string, init: RequestInit = {}) {
  const controller = new AbortController();
  const timeoutId = window.setTimeout(() => controller.abort(), API_REQUEST_TIMEOUT_MS);

  try {
    return await fetch(input, { ...init, signal: controller.signal });
  } finally {
    window.clearTimeout(timeoutId);
  }
}

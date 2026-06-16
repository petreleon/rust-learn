"use client";

import { type HttpMethod } from "../model/HttpMethod";
import { normalizeRoot } from "../model/normalizeRoot";
import { prettyBody } from "../model/prettyBody";
import { fetchWithTimeout } from "./fetchWithTimeout";

export async function signInWithPassword({
  apiRoot,
  email,
  password,
}: {
  apiRoot: string;
  email: string;
  password: string;
}) {
  const response = await fetchWithTimeout(`${normalizeRoot(apiRoot)}/auth/login`, {
    body: JSON.stringify({ email: email.trim(), password }),
    headers: { Accept: "application/json, text/plain", "Content-Type": "application/json" },
    method: "POST",
  });
  const text = await response.text();
  const body = prettyBody(text);
  return { body, ok: response.ok, status: `HTTP ${response.status}`, token: response.ok ? parseTokenResponse(text) : "" };
}

export async function sendOpsApiRequest({
  apiRoot,
  body,
  method,
  path,
  token,
}: {
  apiRoot: string;
  body?: unknown;
  method: HttpMethod;
  path: string;
  token: string;
}) {
  const headers = new Headers();
  headers.set("Accept", "application/json, text/csv, text/plain");
  headers.set("Authorization", `Bearer ${token.trim()}`);
  if (body !== undefined) {
    headers.set("Content-Type", "application/json");
  }
  const response = await fetchWithTimeout(`${normalizeRoot(apiRoot)}${path}`, {
    body: body === undefined ? undefined : JSON.stringify(body),
    headers,
    method,
  });
  return { body: prettyBody(await response.text()), ok: response.ok, status: response.status };
}

function parseTokenResponse(text: string) {
  let nextToken = text.trim();
  try {
    const parsed: unknown = JSON.parse(text);
    if (typeof parsed === "string") {
      nextToken = parsed;
    } else if (parsed && typeof parsed === "object" && "token" in parsed && typeof parsed.token === "string") {
      nextToken = parsed.token;
    }
  } catch {}
  return nextToken;
}

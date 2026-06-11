"use client";
export function formatHealthMessage(value: string) {
  const trimmed = value.trim();
  if (!trimmed) {
    return "API responded";
  }

  try {
    const parsed: unknown = JSON.parse(trimmed);
    if (parsed && typeof parsed === "object" && "status" in parsed) {
      const status = String((parsed as { status?: unknown }).status ?? "").trim();
      if (status.toLowerCase() === "ok") {
        return "API healthy";
      }
      if (status) {
        return `API status: ${status}`;
      }
    }
  } catch {
    return trimmed;
  }

  return trimmed;
}

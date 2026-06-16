export function lifecycleTone(status: string) {
  if (status === "published" || status === "approved") return "good";
  if (["archived", "suspended", "needs_changes"].includes(status)) return "warn";
  return "neutral";
}

export function statusLabel(value: string) {
  return value.replace(/_/g, " ").replace(/\b\w/g, (letter) => letter.toUpperCase());
}

export function organizationNames(names: string[]) {
  return names.join(", ") || "Personal course";
}

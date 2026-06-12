const ACRONYMS: Record<string, string> = {
  api: "API",
  csv: "CSV",
  id: "ID",
  kyc: "KYC",
  s3: "S3",
};

function formatToken(token: string, index: number) {
  const lower = token.toLowerCase();
  if (ACRONYMS[lower]) {
    return ACRONYMS[lower];
  }
  return index === 0 ? lower.charAt(0).toUpperCase() + lower.slice(1) : lower;
}

export function formatAccessLabel(value: string) {
  return value
    .split("_")
    .filter(Boolean)
    .map((token, index) => formatToken(token, index))
    .join(" ");
}

export function pluralize(count: number, singular: string, plural = `${singular}s`) {
  return `${count} ${count === 1 ? singular : plural}`;
}

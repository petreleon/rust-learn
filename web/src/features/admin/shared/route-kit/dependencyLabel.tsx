"use client";

import { formatUnderscoreLabel } from "./formatUnderscoreLabel";

export function dependencyLabel(name: string) {
  if (name === "postgres") {
    return "PostgreSQL";
  }
  if (name === "s3") {
    return "S3 storage";
  }
  if (name === "ethereum") {
    return "Ethereum";
  }
  return formatUnderscoreLabel(name);
}

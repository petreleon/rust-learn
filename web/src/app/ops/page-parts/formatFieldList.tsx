"use client";
export function formatFieldList(fields: string[]) {
  if (fields.length === 1) {
    return `${fields[0]} is required.`;
  }

  return `${fields.slice(0, -1).join(", ")} and ${fields[fields.length - 1]} are required.`;
}

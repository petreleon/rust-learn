"use client";
export function missingFields(fields: Array<[label: string, complete: boolean]>) {
  return fields.filter(([, complete]) => !complete).map(([label]) => label);
}

"use client";

import { humanize } from "./humanize";
import { sentenceCase } from "./sentenceCase";

export function humanRewardStatus(status: string) {
  return sentenceCase(humanize(status));
}

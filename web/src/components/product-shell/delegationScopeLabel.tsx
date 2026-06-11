"use client";

import { type CurrentSession } from "@/lib/session";

export function delegationScopeLabel(delegation: CurrentSession["delegated_permissions"][number]) {
  return delegation.organization_name || delegation.course_title || delegation.scope_type;
}

import { type DelegationItem } from "@/lib/admin/DelegationItem";
import { type DelegationStatus } from "@/lib/admin/DelegationStatus";
import { formatUnderscoreLabel } from "@/features/admin/shared/route-kit/formatUnderscoreLabel";

export function delegationStatus(delegation: DelegationItem): DelegationStatus {
  if (delegation.revoked_at) return "revoked";
  if (delegation.expires_at && new Date(delegation.expires_at) <= new Date()) return "expired";
  return "active";
}

export function delegationScopeLabel(delegation: DelegationItem) {
  if (delegation.scope_type === "platform") return "Platform";
  if (delegation.scope_type === "organization" && delegation.organization_id) {
    return `Organization ${delegation.organization_id}`;
  }
  if (delegation.scope_type === "course" && delegation.course_id) {
    return `Course ${delegation.course_id}`;
  }
  return formatUnderscoreLabel(delegation.scope_type);
}

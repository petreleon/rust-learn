import { type OrganizationMemberList } from "@/lib/organization/OrganizationMemberList";

export function memberDirectoryCounts(members: OrganizationMemberList, page: number) {
  return {
    canGoBack: members.offset > 0,
    canGoForward: members.offset + members.limit < members.total,
    delegatedCount: members.members.filter((member) => member.delegated_permission_count > 0).length,
    kycReadyCount: members.members.filter((member) => member.kyc_verified).length,
    pageLabel: `Page ${page + 1} of ${Math.max(1, Math.ceil(members.total / members.limit))}`,
    rangeLabel:
      members.total === 0
        ? "0 members"
        : `${members.offset + 1}-${Math.min(members.offset + members.limit, members.total)} of ${members.total}`,
    verifiedEmailCount: members.members.filter((member) => member.email_verified).length,
  };
}

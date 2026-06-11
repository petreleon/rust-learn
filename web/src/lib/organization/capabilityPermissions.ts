import { type OrganizationCapabilityKey } from "./OrganizationCapabilityKey";

export const capabilityPermissions: Array<{
  key: OrganizationCapabilityKey;
  label: string;
  permissions: string[];
}> = [
  {
    key: "courses",
    label: "Courses",
    permissions: ["VIEW_ORGANIZATION"],
  },
  {
    key: "members",
    label: "Members",
    permissions: ["VIEW_ORGANIZATION"],
  },
  {
    key: "reports",
    label: "Reports",
    permissions: ["VIEW_ORG_REWARD_REPORTS"],
  },
  {
    key: "wallet",
    label: "Wallet",
    permissions: ["MANAGE_ORG_BILLING", "MANAGE_ORG_REWARD_BUDGET", "MANAGE_ORG_WALLETS"],
  },
  {
    key: "teacher_applications",
    label: "Teacher nominations",
    permissions: ["NOMINATE_TEACHER_FOR_PLATFORM_REVIEW", "VIEW_ORG_TEACHER_APPLICATIONS"],
  },
  {
    key: "course_rewards",
    label: "Course rewards",
    permissions: ["SUBMIT_ORG_COURSE_REWARD_EVENT"],
  },
  {
    key: "settings",
    label: "Settings",
    permissions: ["MANAGE_ORG_SETTINGS", "VIEW_ORGANIZATION"],
  },
];

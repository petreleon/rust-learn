import { type OrganizationCapabilityKey } from "./OrganizationCapabilityKey";

export type OrganizationCapability = {
  enabled: boolean;
  key: OrganizationCapabilityKey;
  label: string;
  permissions: string[];
};

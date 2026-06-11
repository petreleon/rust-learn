import { type OrganizationCapabilityKey } from "./OrganizationCapabilityKey";

export type OrganizationFilter = {
  capability: OrganizationCapabilityKey | "all" | "delegated";
  search: string;
};

import { type SessionCapability } from "./SessionCapability";

export type PlatformSessionScope = {
  roles: string[];
  direct_permissions: string[];
  delegated_permissions: string[];
  effective_permissions: string[];
  capabilities: SessionCapability[];
};

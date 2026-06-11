import { type PlatformCapabilityKey } from "./PlatformCapabilityKey";

export type PlatformCapability = {
  enabled: boolean;
  key: PlatformCapabilityKey;
  label: string;
  permissions: string[];
};

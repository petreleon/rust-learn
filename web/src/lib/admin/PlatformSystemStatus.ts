import { type SystemLiveness } from "./SystemLiveness";
import { type SystemReadiness } from "./SystemReadiness";

export type PlatformSystemStatus = {
  liveness: SystemLiveness;
  readiness: SystemReadiness;
};

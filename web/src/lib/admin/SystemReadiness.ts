import { type SystemDependencyCheck } from "./SystemDependencyCheck";

export type SystemReadiness = {
  checks: SystemDependencyCheck[];
  status: string;
};

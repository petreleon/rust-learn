import { type PlatformSessionScope } from "./PlatformSessionScope";

export type OrganizationSessionScope = PlatformSessionScope & {
  id: number;
  name: string;
};

import { type PlatformSessionScope } from "./PlatformSessionScope";

export type CourseSessionScope = PlatformSessionScope & {
  id: number;
  title: string;
  lifecycle_status: string;
};

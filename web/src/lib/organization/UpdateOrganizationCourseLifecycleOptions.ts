import { type OrganizationRequestOptions } from "./OrganizationRequestOptions";

export type UpdateOrganizationCourseLifecycleOptions = OrganizationRequestOptions & {
  courseId: number | string;
  status: string;
};

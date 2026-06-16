import { type OrganizationRequestOptions } from "./OrganizationRequestOptions";
import { type UpdateOrganizationCoursePayload } from "./UpdateOrganizationCoursePayload";

export type UpdateOrganizationCourseOptions = OrganizationRequestOptions & {
  courseId: number | string;
  payload: UpdateOrganizationCoursePayload;
};

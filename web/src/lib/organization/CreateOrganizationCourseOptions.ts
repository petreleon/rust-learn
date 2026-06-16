import { type CreateOrganizationCoursePayload } from "./CreateOrganizationCoursePayload";
import { type OrganizationRequestOptions } from "./OrganizationRequestOptions";

export type CreateOrganizationCourseOptions = OrganizationRequestOptions & {
  payload: CreateOrganizationCoursePayload;
};

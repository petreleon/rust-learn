import { type LearnerRequestOptions } from "./LearnerRequestOptions";

export type CourseCatalogOptions = LearnerRequestOptions & {
  enrollmentStatus?: string;
  limit?: number;
  offset?: number;
  organizationId?: number;
  rewardAvailable?: boolean;
  search?: string;
};

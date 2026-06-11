import { type LearnerRequestOptions } from "./LearnerRequestOptions";

export type RewardHistoryOptions = LearnerRequestOptions & {
  courseId?: number;
  limit?: number;
  offset?: number;
  status?: string;
};

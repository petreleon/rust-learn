import { type OrganizationCourseListItem } from "./OrganizationCourseListItem";

export type OrganizationCourseList = {
  courses: OrganizationCourseListItem[];
  lifecycle_status: string | null;
  limit: number;
  offset: number;
  organization: {
    id: number;
    name: string;
  };
  reward_available: boolean | null;
  search: string | null;
  total: number;
};

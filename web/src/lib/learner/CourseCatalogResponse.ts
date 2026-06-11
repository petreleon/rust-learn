import { type CourseCatalogItem } from "./CourseCatalogItem";

export type CourseCatalogResponse = {
  courses: CourseCatalogItem[];
  enrollment_status: string | null;
  lifecycle_status: string | null;
  limit: number;
  offset: number;
  organization_id: number | null;
  reward_available: boolean | null;
  search: string | null;
  total: number;
};

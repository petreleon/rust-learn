import { type CourseAccessSummary } from "./CourseAccessSummary";
import { type CourseCatalogOrganization } from "./CourseCatalogOrganization";
import { type CourseCatalogTeacher } from "./CourseCatalogTeacher";
import { type CourseContentSummary } from "./CourseContentSummary";
import { type CourseEnrollmentSummary } from "./CourseEnrollmentSummary";
import { type CourseRewardSummary } from "./CourseRewardSummary";

export type CourseCatalogItem = {
  access: CourseAccessSummary;
  content: CourseContentSummary;
  description: string | null;
  enrollment: CourseEnrollmentSummary;
  id: number;
  lifecycle_status: string;
  organizations: CourseCatalogOrganization[];
  prerequisites: string[];
  rewards: CourseRewardSummary;
  teachers: CourseCatalogTeacher[];
  title: string;
  topics: string[];
};

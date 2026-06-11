"use client";

import { type TeacherCoursesResponse } from "@/lib/teacher";

export const emptyCourses: TeacherCoursesResponse = {
  courses: [],
  lifecycle_status: null,
  limit: 25,
  offset: 0,
  search: null,
  total: 0,
};

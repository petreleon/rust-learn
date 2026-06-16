export type CourseQuery = {
  lifecycleStatus: string;
  search: string;
};

export const defaultCourseQuery: CourseQuery = {
  lifecycleStatus: "all",
  search: "",
};

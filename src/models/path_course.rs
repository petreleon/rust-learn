use super::schema::paths_courses;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[primary_key(path_id, course_id)]
#[table_name = "paths_courses"]
pub struct PathCourse {
    pub path_id: i32,
    pub course_id: i32,
    pub order: i32,
}
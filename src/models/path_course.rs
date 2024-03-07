use crate::db::schema::paths_courses;
use crate::models::course::Course;
use crate::models::path::Path;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[belongs_to(Path)]
#[belongs_to(Course)]
#[primary_key(path_id, course_id)]
#[diesel(table_name = paths_courses)]
pub struct PathCourse {
    pub path_id: i32,
    pub course_id: i32,
    pub order: i32,
}
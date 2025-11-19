use crate::db::schema::paths_courses;
use crate::models::course::Course;
use crate::models::path::Path;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[diesel(belongs_to(Path))]
#[diesel(belongs_to(Course))]
#[diesel(primary_key(path_id, course_id))]
#[diesel(table_name = paths_courses)]
pub struct PathCourse {
    pub path_id: i32,
    pub course_id: i32,
    pub order: i32,
}
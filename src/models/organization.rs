use crate::db::schema::organizations;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[diesel(table_name = organizations)]
pub struct Organization {
    pub id: i32,
    pub name: String,
    pub website_link: Option<String>,
    pub profile_url: Option<String>,
}

use crate::db::schema::organizations;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Identifiable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = organizations)]
pub struct Organization {
    pub id: i32,
    pub name: String,
    pub website_link: Option<String>,
    pub profile_url: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = organizations)]
pub struct NewOrganization {
    pub name: String,
    pub website_link: Option<String>,
    pub profile_url: Option<String>,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = organizations)]
pub struct UpdateOrganization {
    pub name: Option<String>,
    pub website_link: Option<String>,
    pub profile_url: Option<String>,
}

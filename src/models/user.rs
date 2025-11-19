use diesel::prelude::*;
use crate::db::schema::users;
use chrono::NaiveDate;
use chrono::NaiveDateTime;

#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub date_of_birth: Option<NaiveDate>, // Use Option if the field can be null
    pub created_at: NaiveDateTime,
    pub kyc_verified: bool,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub date_of_birth: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub kyc_verified: bool,
}

impl User {
    // Method to get the user's id
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn find_all(conn: &mut PgConnection) -> QueryResult<Vec<User>> {
        users::table.load::<User>(conn)
    }

    pub fn find_by_id(id: i32, conn: &mut PgConnection) -> QueryResult<User> {
        users::table.find(id).first(conn)
    }

    pub fn find_by_email(email: &str, conn: &mut PgConnection) -> QueryResult<User> {
        users::table.filter(users::email.eq(email)).first(conn)
    }

    pub fn create(new_user: NewUser, conn: &mut PgConnection) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
    }

    pub fn find_with_password_auth(email: &str, conn: &mut PgConnection) -> QueryResult<(User, Option<String>)> {
        use crate::db::schema::authentications;
        users::table
            .filter(users::email.eq(email))
            .inner_join(authentications::table.on(users::id.eq(authentications::user_id)))
            .filter(authentications::type_authentication.eq("password"))
            .select((users::all_columns, authentications::info_auth.nullable()))
            .first(conn)
    }
}
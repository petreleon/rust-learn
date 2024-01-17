use actix_web::{web, HttpResponse, Responder, post, get};
use serde::Deserialize;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::models::authentication::Authentication;
use crate::models::user::User;
use crate::db;
use crate::db::schema::users;
use diesel::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};
use crate::utils::jwt_utils::create_jwt;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
    pub date_of_birth: Option<NaiveDate>,
}

#[derive(Queryable)]
struct LoginQueryResult {
    user: User,
    info_auth: Option<String>,
}

#[post("/login")]
pub async fn login(
    pool: web::Data<db::DbPool>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    use crate::db::schema::{users, authentications};

    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let user_auth_result = users::table
        .filter(users::email.eq(&req.email))
        .inner_join(authentications::table.on(users::id.nullable().eq(authentications::user_id)))
        .filter(authentications::type_authentication.eq("password")) // Ensure type is "password"
        .select((users::all_columns, authentications::info_auth.nullable()))
        .first::<LoginQueryResult>(&mut conn);

    match user_auth_result {
        Ok(LoginQueryResult { user, info_auth }) => {
            if let Some(hash) = info_auth {
                if verify(&req.password, &hash).unwrap_or(false) {
                    match create_jwt(user.id()) {
                        Ok(user_jwt) => {
                            HttpResponse::Ok().json(user_jwt) // Return JWT token in response
                        }
                        Err(_) => {
                            HttpResponse::InternalServerError().body("Failed to create JWT")
                        }
                    }
                } else {
                    HttpResponse::Unauthorized().body("Invalid credentials")
                }
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
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


#[post("/register")]
pub async fn register(
    pool: web::Data<db::DbPool>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // Create new user
    let new_user_data = NewUser {
        name: req.name.to_string(),
        email: req.email.clone(),
        date_of_birth: req.date_of_birth,
        created_at: chrono::Utc::now().naive_utc(),
        kyc_verified: false,
    };

    use crate::db::schema::users;
    let inserted_user = diesel::insert_into(users::table)
        .values(&new_user_data)
        .get_result::<User>(&mut conn)
        .expect("Error saving new user");

    // Hash password and create authentication
    let hashed_password = hash(&req.password, DEFAULT_COST).unwrap();
    let new_auth = Authentication {
        user_id: inserted_user.id(),
        type_authentication: "password".to_string(),
        info_auth: hashed_password,
    };

    use crate::db::schema::authentications;
    diesel::insert_into(authentications::table)
        .values(&new_auth)
        .execute(&mut conn)
        .expect("Error saving new authentication");

    HttpResponse::Ok().body("Registration successful")
}

// hello 
#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// Define the scope for authentication-related routes
pub fn auth_scope() -> actix_web::Scope {
    web::scope("/auth")
    .service(login)
    .service(register)
    .service(hello)
}

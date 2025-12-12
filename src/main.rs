mod models;
pub mod db;
pub mod api;
pub mod utils;
pub mod middlewares;
pub mod config;
use crate::config::db_setup::version_updater;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use infer::Infer;
use crate::utils::minio_utils::MinioState;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/{name}")]
async fn hello2(name: web::Path<String>) -> impl Responder {
    let response_message = format!("Hello, {}!", name);
    HttpResponse::Ok().body(response_message)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/echo_bin")]
async fn echo_bin(req_body: web::Bytes) -> impl Responder {
    let infer = Infer::new();
    let kind = infer.get(&req_body);

    let content_type = kind.map_or("application/octet-stream", |kind| kind.mime_type());

    HttpResponse::Ok()
        .content_type(content_type) // Set the content type to the detected MIME type
        .body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    // Use the establish_connection function from the db module
    let pool = db::establish_connection();
    // Initialize MinIO client state and put into app data
    let minio_state = match MinioState::new_from_env().await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to initialize MinIO client: {:?}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "MinIO init failed"));
        }
    };
    // Initialize notifications state (DB-backed using the pool)
    let notifications_state = crate::utils::notifications::NotificationsState::new(pool.clone());
    {
        let mut conn = pool.get().await.expect("Failed to get DB connection from pool");
        version_updater(&mut conn).await.expect("Failed to update database version");

        // Ensure LearnToken is deployed (idempotent: uses persistent state)
        match crate::utils::eth_utils::deploy_startup(&mut conn, "LearnToken", "LRN", 18).await {
            Ok(addr) => eprintln!("LearnToken available at {}", format!("{:#x}", addr)),
            Err(err) => eprintln!("deploy_startup failed: {:?}", err),
        }
    }
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Use the created pool
            .app_data(web::Data::new(minio_state.clone())) // MinIO client shared state
            .app_data(web::Data::new(notifications_state.clone())) // Notifications shared state
            .route("/hey", web::get().to(manual_hello))
            .service(api::api_scope())
            .service(hello)
            .service(hello2)
            .service(echo)
            .service(echo_bin)
    })
    .bind("0.0.0.0:8080")? // Update the bind address if necessary
    .run()
    .await
}

mod models;
pub mod db;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use infer::Infer;

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
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(hello2)
            .service(echo)
            .service(echo_bin)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

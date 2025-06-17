use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use std::env;

#[get("/api/posts")]
async fn posts() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .insert_header(("Access-Control-Allow-Methods", "GET, POST, OPTIONS"))
        .insert_header(("Access-Control-Allow-Headers", "Content-Type"))
        .content_type("application/json")
        .body("[{\"title\": \"Hello, world!\"}]")
}

#[post("api/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8081);
    info!("server running on port {}", port);
    println!("Server running on {}", port);
    HttpServer::new(|| {
        App::new()
            .service(posts)
            .service(echo)
            .route("/api/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
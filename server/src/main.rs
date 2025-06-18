use actix_web::{App, HttpResponse, HttpServer, Responder, get, middleware::Logger, post, web};
use actix_web_lab::extract::Path;
use log::info;
use std::env;
use std::sync::Arc;

mod broadcast;
use self::broadcast::Broadcaster;

#[get("/api/posts")]
async fn posts() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .insert_header(("Access-Control-Allow-Methods", "GET, POST, OPTIONS"))
        .insert_header(("Access-Control-Allow-Headers", "Content-Type"))
        .content_type("application/json")
        .body("[{\"title\": \"Hello, world!\"}]")
}

#[get("/")]
async fn index() -> impl Responder {
    println!("GET to /");
    web::Html::new(include_str!("static/index.html").to_owned())
}

// test by running: curl -X POST 127.0.0.1:8081/broadcast/my_message
#[get("/events")]
async fn event_stream(broadcaster: web::Data<Broadcaster>) -> impl Responder {
    println!("GET to /events");
    broadcaster.new_client().await
}

#[post("/broadcast/{msg}")]
async fn broadcast_msg(
    broadcaster: web::Data<Broadcaster>,
    Path((msg,)): Path<(String,)>,
) -> impl Responder {
    println!("POST to /broadcast");
    let json_msg = format!("{{\"message\":\"{}\"}}", msg);
    broadcaster.broadcast(&json_msg).await;
    HttpResponse::Ok().body("msg sent")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let data = Broadcaster::create();

    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8081);

    info!("server running on port {}", port);
    println!("Server running on {}", port);

    HttpServer::new(move || {
        use actix_cors::Cors;
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .app_data(web::Data::from(Arc::clone(&data)))
            .service(index)
            .service(event_stream)
            .service(broadcast_msg)
            .service(posts)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", port))?
    .workers(2)
    .run()
    .await
}

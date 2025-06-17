use std::{sync::Arc};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use actix_web_lab::extract::Path;
use log::info;
use std::env;

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

// #[post("api/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

#[get("/")]
async fn index() -> impl Responder {
    web::Html::new(include_str!("index.html").to_owned())
}

// test by running: curl -X POST 127.0.0.1:8081/broadcast/my_message
#[get("/events")]
async fn event_stream(broadcaster: web::Data<Broadcaster>) -> impl Responder {
    broadcaster.new_client().await
}

#[post("/broadcast/{msg}")]
async fn broadcast_msg(
    broadcaster: web::Data<Broadcaster>,
    Path((msg,)): Path<(String,)>,
) -> impl Responder {
    broadcaster.broadcast(&msg).await;
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
                    .allow_any_header()
            )
            .app_data(web::Data::from(Arc::clone(&data)))
            .service(index)
            .service(event_stream)
            .service(broadcast_msg)
            .service(posts)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", port))?
    .workers(2)
    .run()
    .await
}
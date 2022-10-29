use actix_web::{get, middleware, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

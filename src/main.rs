use actix_web::{middleware, App, HttpServer};

mod routes;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("Docker-Distribution-Api-Version", "registry/2.0")),
            )
            .service(routes::home)
            .service(routes::v2_redirect)
            .service(routes::v2)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

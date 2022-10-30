use actix_web::{middleware, web, App, HttpServer};

mod base;
mod push;

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
            .service(base::home)
            .service(base::v2_redirect)
            .service(base::v2)
            .service(web::scope("/v2").service(push::pre_upload))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

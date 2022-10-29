use actix_web::{get, web::Bytes, HttpResponse, Responder};

#[get("/")]
pub async fn home() -> impl Responder {
    HttpResponse::Ok().body(Bytes::new())
}

#[get("/v2")]
pub async fn v2_redirect() -> impl Responder {
    HttpResponse::MovedPermanently()
        .append_header(("Location", "/v2/"))
        .append_header(("Content-Type", "text/html; charset=utf-8"))
        .body("<a href=\"/v2/\">Moved Permanently</a>.\n")
}

#[get("/v2/")]
pub async fn v2() -> impl Responder {
    HttpResponse::Ok()
        .append_header(("X-Content-Type-Options", "nosniff"))
        .append_header(("Content-Type", "application/json; charset=utf-8"))
        .body("{}")
}

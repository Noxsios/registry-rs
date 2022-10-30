use actix_web::{get, web, Responder};

#[get("{namespace:.*}/blobs/uploads/")]
pub async fn pre_upload(namespace: web::Path<String>) -> impl Responder {
    format!("Hello {}!", namespace)
}

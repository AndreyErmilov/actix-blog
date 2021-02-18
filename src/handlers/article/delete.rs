use actix_web::HttpResponse;

pub async fn delete_article() -> HttpResponse {
    HttpResponse::NoContent().finish()
}

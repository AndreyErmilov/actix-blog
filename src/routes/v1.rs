use actix_web::web;

use crate::handlers;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service({
        web::resource("/users/{user_id}/articles/")
            .name("posts")
            .route(web::get().to(handlers::article::read::read_articles))
            .route(web::post().to(handlers::article::read::read_articles))
    })
    .service({
        web::resource("/users/{user_id}/articles/{article_id}/")
            .name("article")
            .route(web::put().to(handlers::article::read::read_articles))
            .route(web::delete().to(handlers::article::read::read_articles))
    });
}

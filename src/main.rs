use actix_web::{middleware, web, App, HttpServer};
use blog_example::routes::v1;
use blog_example_database::DatabaseActor;
use tracing::Level;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let database =
        DatabaseActor::start(String::from("postgres://bloguser:bloguser@localhost/blog")).unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(database.clone())
            .data(web::JsonConfig::default().limit(64 * 1024))
            .service(web::scope("/v1").configure(v1::routes))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
}

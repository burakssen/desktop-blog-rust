use actix_web::{web, App, HttpResponse, HttpServer};
mod controllers;
mod service;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(controllers::get_all_users)
            .service(controllers::create_user)
            .route("/", web::get().to(HttpResponse::Ok))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

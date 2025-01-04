use actix_web::{web, App, HttpServer};
mod models;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/ajouter_utilisateur", web::post().to(api::utilisateur::add_user))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

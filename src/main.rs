use actix_web::web::route;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;

mod api;
mod models;


#[actix_web::main] 
async fn main() -> std::io::Result<()> {
   HttpServer::new(|| {
        App::new()
            .wrap(Logger::default()) // Ajout de la journalisation des requêtes
            .route("/ajouter_utilisateur", web::post().to(api::utilisateur::ajouter_utilisateur))
            .route("/obtenir_utilisateur/{id}", web::get().to(api::utilisateur::recuperer_utilisateur))
            .route("ajouter_projet", web::post().to(api::projet::ajouter_projet))
            .route("/supprimmer_projet", web::post().to(api::projet::supprimer_projet))
    })
    .bind("127.0.0.1:8000")? // Bind le serveur à l'adresse et au port spécifiés
    .run() // Lancer le serveur Actix
    .await 
}

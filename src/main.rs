use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;

mod api;
mod models;

#[actix_web::main] // Attribut pour initialiser Actix avec Tokio
async fn main() -> std::io::Result<()> {
    // Actix Web utilise Tokio en arrière-plan pour gérer l'asynchronicité
   HttpServer::new(|| {
        App::new()
            .wrap(Logger::default()) // Ajout de la journalisation des requêtes
            .route("/ajouter_utilisateur", web::post().to(api::utilisateur::ajouter_utilisateur))
    })
    .bind("127.0.0.1:8000")? // Bind le serveur à l'adresse et au port spécifiés
    .run() // Lancer le serveur Actix
    .await // Attend que le serveur finisse son exécution
}

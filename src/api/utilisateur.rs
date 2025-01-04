use crate::models::utilisateur::Utilisateurs;
use postgres::{Client, NoTls, Error};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use chrono::{DateTime, Utc};




pub async fn add_user(form: web::Json<Utilisateurs>,) -> impl Responder{
    let user = form.into_inner();

    //connexion a la base de données
    let mut client = match Client::connect(
        "host=localhost user=postgres password=root dbname=projet_sga", NoTls
    ){
        Ok(client) => client,
        Err(err) => {
            eprintln!("Erreur de connexion à la base deonnées: {:?}", err);
            return HttpResponse::InternalServerError().body("Erreur de connexion à la base de données");
        }
    };

    //Insérer l'utilsateur
    let query = r#"
    INSERT INTO utilisateurs (nom, role, email, date_creation) VALUES ($1, $2, $3, $4) RETURNING id
    "#;

    match client.query_one(
        query,
        &[
            &user.nom,
            &user.role,
            &user.email,
            &utc::now(),
        ],
    ){
        Ok(row) => {
            let id: i32 = row.get("id");
            HttpResponse::Ok().json(format!("Utilisateur ajouté avec l'ID: {}", id))
        }
        Err(err) => {
            eprintln!("Erreur lors de l'ajout de l'utilisateur: {:?}", err);
            HttpResponse::InternalServerError().body("Erreur lors de l'ajout de l'utilisateur")
        }
    }
}
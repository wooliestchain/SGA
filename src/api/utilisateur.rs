use crate::models::utilisateur::Utilisateurs;
use axum::http::response;
use postgres::{row, Client, NoTls};
use actix_web::{web, HttpResponse, Responder};
use chrono::{Utc};
use super::dbconnect::database_connexion;


pub async fn ajouter_utilisateur(form: web::Json<Utilisateurs>) -> impl Responder{
    let mut user = form.into_inner();

    // Si la date_creation est None, la remplacer par la date actuelle
    if user.date_creation.is_none() {
        user.date_creation = Some(Utc::now());
    }

    // Convertir DateTime<Utc> en NaiveDateTime (qui est compatible avec TIMESTAMP PostgreSQL)
    let date_creation_naive = user.date_creation.unwrap().naive_utc();

    // Convertir NaiveDateTime en chaîne de caractères qui peut être acceptée par PostgreSQL
    let date_creation_str = date_creation_naive.to_string();

    //créer un tahce asynchrone pour la connexoion à la base de données
    let result = tokio::task::spawn_blocking(move ||{
        //connexion à la base de données
        let mut client = database_connexion().map_err(|err| {
            eprintln!("Erreur de connexion à la base de données : {:?}", err);
            "Erreur de connexion à la base de données"
        })?;



        let now = Utc::now();
        let naive_now = now.naive_utc();

        let query = r#"
        INSERT INTO utilisateurs (nom, role, email, date_creation) VALUES ($1, $2, $3, $4) RETURNING id;
        "#;

        match client.query_one(
            query, &[&user.nom, &user.role, &user.email, &date_creation_str],
        ){
            Ok(row) => {
                let id: i32 = row.get("id");
                Ok(format!("Utilisateur ajouté avec l'ID: {}", id))
            }
            Err(err) => {
                eprintln!("Erreur lors de l'ajout de l'utilisateur : {:?}", err);
                Err("Erreur lors de l'ajout de l'utilisateur")
            }
        }
    }).await;

    match result{
        Ok(Ok(response)) => HttpResponse::Ok().json(response),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(err),
        Err(_) => HttpResponse::InternalServerError().body("Erreur interne du serveur"),
    }
}
use crate::models::utilisateur::{self, Utilisateurs};
use axum::http::response;
use postgres::{Client, NoTls};
use actix_web::{web, HttpResponse, Responder, Error};
use actix_web::error::ErrorInternalServerError;
use chrono::Utc;
use super::dbconnect::database_connexion;

/// Fonction pour ajouter un utilisateur
pub async fn ajouter_utilisateur(form: web::Json<Utilisateurs>) -> impl Responder {
    let mut user = form.into_inner();

    // Assurez-vous que la date de création est définie
    if user.date_creation.is_none() {
        user.date_creation = Some(Utc::now().naive_utc());
    }

    let result = tokio::task::spawn_blocking(move || {
        // Connexion à la base de données
        let mut client = database_connexion().map_err(|err| {
            eprintln!("Erreur de connexion à la base de données : {:?}", err);
            "Erreur de connexion à la base de données"
        })?;

        let query = r#"
        INSERT INTO utilisateurs (nom, role, email, date_creation)
        VALUES ($1, $2, $3, $4)
        RETURNING id;
        "#;

        match client.query_one(
            query,
            &[&user.nom, &user.role, &user.email, &user.date_creation],
        ) {
            Ok(row) => {
                let id: i32 = row.get("id");
                Ok(format!("Utilisateur ajouté avec l'ID: {}", id))
            }
            Err(err) => {
                eprintln!("Erreur lors de l'ajout de l'utilisateur : {:?}", err);
                Err("Erreur lors de l'ajout de l'utilisateur")
            }
        }
    })
    .await;

    match result {
        Ok(Ok(response)) => HttpResponse::Ok().json(response),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(err),
        Err(_) => HttpResponse::InternalServerError().body("Erreur interne du serveur"),
    }
}

/// Fonction pour récupérer un utilisateur par ID
pub async fn obtenir_utilisateur(id: web::Path<i32>) -> impl Responder {
    let user_id = id.into_inner();

    let result = tokio::task::spawn_blocking(move || {
        // Connexion à la base de données
        let mut client = database_connexion().map_err(|err| {
            eprintln!("Erreur de connexion à la base de données : {:?}", err);
            ErrorInternalServerError("Erreur de connexion à la base de données")
        })?;

        let query = r#"
        SELECT id, nom, role, email, date_creation 
        FROM utilisateurs 
        WHERE id = $1;
        "#;

        client.query_one(query, &[&user_id]).map(|row| {
            Utilisateurs {
                id: Some(row.get("id")),
                nom: row.get("nom"),
                role: row.get("role"),
                email: row.get("email"),
                date_creation: Some(row.get("date_creation")),
            }
        }).map_err(|err| {
            eprintln!("Erreur lors de la récupération de l'utilisateur : {:?}", err);
            ErrorInternalServerError("Utilisateur non trouvé")
        })
    })
    .await;

    match result {
        Ok(Ok(utilisateur)) => HttpResponse::Ok().json(utilisateur),
        Ok(Err(err_response)) => HttpResponse::InternalServerError().body(err_response.to_string()),
        Err(err) => {
            eprintln!("Erreur lors de l'exécution de la tâche : {:?}", err);
            HttpResponse::InternalServerError().body("Erreur interne du serveur")
        }
    }
}

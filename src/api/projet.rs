use crate::models::projet::{self, Projet};
use axum::http::response;
use postgres::{Client, NoTls};
use actix_web::{web, HttpResponse, Responder, Error};
use actix_web::error::ErrorInternalServerError;
use chrono::Utc;
use tokio::task;
use super::dbconnect::database_connexion;
use serde_json::json;


//Fonction pour ajouter un projet 
pub async fn ajouter_projet (form: web::Json<Projet>) -> impl Responder{
    let mut projet = form.into_inner();

    let result = tokio::task::spawn_blocking(move || {

        let mut client = database_connexion().map_err(|err|{
            eprintln!("Erreur de connexion à la base de données: {:?}", err);
            "Erreur de connexion à la base de données"
        })?;

        //Modifier Projet pour y ajouter code, établir des inter médiations pour d'autres colonnes qui vont etree séparées 
        let query = r#"
        INSERT INTO Projet(nom, code, description, ministere_responsable, referant_projet, referant_presidence, annee_debut, annee_fin, type_projet, source_financement,statut, priorite, objectifs, impact_attendu)
        VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14)
        RETURNING id;
        "#;

        match client.query_one(query, &[&projet.nom, &projet.description, &projet.ministere_responsable, &projet.referant_projet, &projet.referant_presidence, &projet.annee_debut, &projet.annee_fin, &projet.type_projet, &projet.source_financement, &projet.statut, &projet.priorite, &projet.objectifs, &projet.impact_attendu],
        ){
            Ok(row) =>{
                let id: i32 = row.get("id");
                Ok(format!("Projet ajouté avec l'ID: {}", id))
            }
            Err(err) => {
                eprintln!("Erreur lors de l'ajout du projet: {:?}", err);
                Err("Erreur lors de l'ajout du projet")
            }
        }
    })
    .await;

    match result{
        Ok(Ok(response)) =>HttpResponse::Ok().json(response),
        Ok(Err(err))=> HttpResponse::InternalServerError().body(err),
        Err(_) => HttpResponse::InternalServerError().body("Erreur interne du serveur")
    }
}

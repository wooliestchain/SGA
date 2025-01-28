use crate::models::projet::{self, Projet};
use axum::http::response;
use postgres::{row, Client, NoTls};
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
        INSERT INTO Projet(nom, code, description, annee_debut, annee_fin, impact_attendu)
        VALUES($1,$2,$3,$4,$5,$6)
        RETURNING id;
        "#;

        match client.query_one(query, &[&projet.nom, &projet.code,&projet.description, &projet.annee_debut, &projet.annee_fin, &projet.impact_attendu],
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

pub async fn recuperer_projet (code: web::Path<i32>) -> impl Responder{
    let code = code.into_inner();

    let result = tokio::task::spawn_blocking(move || {

        let mut client = database_connexion().map_err(|err|{
            eprintln!("Erreur de connexion à labase de données: {:?}", err);
            "Erreur de connexion à la base de données"
        })?;

        let query = r#"
        SELECT nom, code, description, ministere_responsable, referant_projet, referant_presidence, annee_debut, annee_fin, type_projet, source_financement, date_creation, statut, priorite, objectifs, impact_attendu
        FROM Projet
        WHERE code = $1;
        "#;

        match client.query_one(query, &[&code]){
            Ok(row) => {
                let projet = Projet{
                    id: row.get("id"),
                    nom: row.get("nom"),
                    code: row.get("code"),
                    description: row.get("description"),
                    annee_debut: row.get("annee_debut"),
                    annee_fin: row.get("annee_fin"),
                    date_creation : row.get::<_, Option<String>>("date_creation"),
                    impact_attendu: row.get("impact_attendu")
                };
                Ok(projet)
            }
            Err(err) =>{
                eprintln!("Erreur lors de la récupération du projet : {:?}", err);
                Err("Projet non trouvé")
            }
        }
    })
    .await;

    match result {
        Ok(Ok(projet)) => HttpResponse::Ok().json(projet),
        Ok(Err(err)) => HttpResponse::NotFound().json(json!({"erreur": err})),
        Err(_) => HttpResponse::InternalServerError().body("Erreur interne du serveur"),

    }
}

//Fonction pour supprimer un projet
pub async fn supprimer_projet(code: web::Path<i32>) -> impl Responder{
    let code = code.into_inner();

    let result = tokio::task::spawn_blocking(move|| {

        let mut client = database_connexion().map_err(|err|{
            eprintln!("Erreur de connexion à la base de données: {:?}", err);
            "Erreur de connexion à la base de données"
        })?;

        let query = r#"
        DELETE FROM Projet
        WHERE code = $1;
        "#;

        match client.query_one(query, &[&code]){
            Ok(row) => {
                let projet = Projet{
                    id: row.get("id"),
                    nom: row.get("nom"),
                    code: row.get("code"),
                    description: row.get("description"),
                    ministere_responsable: row.get("ministere_responsable"),
                    referant_projet: row.get("referant_projet"),
                    referant_presidence: row.get("referant_presidence"),
                    annee_debut: row.get("annee_debut"),
                    annee_fin: row.get("annee_fin"),
                    type_projet: row.get("type_projet"),
                    source_financement: row.get("source_financement"),
                    date_creation : row.get::<_, Option<String>>("date_creation"),
                    statut: row.get("statut"),
                    priorite: row.get("priorite"),
                    objectifs: row.get("objectifs"),
                    impact_attendu: row.get("impact_attendu")
                };
                Ok(projet)
            }
            Err(err) => {
                eprintln!("Erreur de connexion lors de la récupération du projet: {:?}", err);
                Err("Utilisateur non trouvé")
            }
        }
    })
    .await;

    match result{
        Ok(Ok(projet)) => HttpResponse::Ok().json(projet),
        Ok(Err(err)) => HttpResponse::NotFound().json(json!({"erreur": err})),
        Err(_) => HttpResponse::InternalServerError().body("Erreur interne du serveur")
    }
}

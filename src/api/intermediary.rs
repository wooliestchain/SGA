use crate::models::projet::{self, Categorie, InfrastructureMobilisee};
use axum::http::response;
use postgres::{row, Client, NoTls};
use actix_web::{web, HttpResponse, Responder, Error};
use actix_web::error::ErrorInternalServerError;
use chrono::Utc;
use tokio::task;
use super::dbconnect::database_connexion;
use serde_json::json;


pub async fn categorie_add(form: web::Json<projet::Categorie>) -> impl Responder{
    let mut categorie = form.into_inner();

    let result = tokio::task::spawn_blocking(move || {

        let mut client = database_connexion().map_err(|err|{
            eprintln!("Erreur de connexion à la base de données {:?}", err);
            "Erreur de connexion à la base de données"
        })?;

        let query = r#"
        INSERT INTO categorie(nom) VALUES ($1) RETURNING id;
        "#;

        match client.query_one(query, &[&categorie.nom]){
            Ok(row)=> {
                let id: i32 = row.get("id");
                Ok(format!("Categorie ajouté avec l'ID: {}", id))
            }
            Err(err)=>{
                eprintln!("Erreur lors de l'ajout du projet {:?}", err);
                Err("Erreur lors de l'ajout de la categorie")
            }
        }
    })
    .await;

    match result{
        Ok(Ok(response)) => HttpResponse::Ok().json(response),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(err),
        Err(_) => HttpResponse::InternalServerError().body("Erreur interne du serveur")
    }

}

pub async fn ville_add(form : web::Json<projet::Ville>) -> impl Responder{
    let ville = form.into_inner();
    let result = tokio::task::spawn_blocking(move ||{

        let mut client  = database_connexion().map_err(|err|{
            eprintln!("Erreur de connexion à la base de données {:?}", err);
            "Erreur de connexion à la base de données"
        })?;

        let query = r#"
        INSERT INTO ville(nom) VALUES ($1) RETURNING id;
        "#;
        match client.query_one(query, &[&ville.nom]){
            Ok(row) => {
                let id: i32 = row.get("id");
                Ok(format!("Ville ajoutée avec l'ID: {}", id))
            }
            Err(err) => {
                eprintln!("Erreur lors de l'ajout de la ville {:?}", err);
                Err("Erreur lors de l'ajout du projet")
            }
        }
    })
    .await;

    match result{
        Ok(Ok(response)) => HttpResponse::Ok().json(response),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(err),
        Err(_) => HttpResponse::InternalServerError().body("Erreur interne du serveur")
    }
}

pub async fn infra_add (form : web::Json<projet::InfrastructureMobilisee>) -> impl Responder{
    let infrastructuremobilisee = form.into_inner();
    let result = tokio::task::spawn_blocking(move ||{

        let mut client  = database_connexion().map_err(|err|{
            eprintln!("Erreur de connexion à la base de données {:?}", err);
            "Erreur de connexion à la base de données"
        })?;

        let query = r#"
        INSERT INTO infrastructuremobilisee(nom) VALUES ($1) RETURNING id;
        "#;
        match client.query_one(query, &[&infrastructuremobilisee.nom]){
            Ok(row) => {
                let id: i32 = row.get("id");
                Ok(format!("Infrastructures ajoutée avec l'ID: {}", id))
            }
            Err(err) => {
                eprintln!("Erreur lors de l'ajout de l'infrastructure {:?}", err);
                Err("Erreur lors de l'ajout de l'infrastructure")
            }
        }
    })
    .await;

    match result{
        Ok(Ok(response)) => HttpResponse::Ok().json(response),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(err),
        Err(_) => HttpResponse::InternalServerError().body("Erreur interne du serveur")
    }
}


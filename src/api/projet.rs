use crate::models::projet::{self, Projet};
use axum::http::response;
use postgres::{Client, NoTls};
use actix_web::{web, HttpResponse, Responder, Error};
use actix_web::error::ErrorInternalServerError;
use chrono::Utc;
use super::dbconnect::database_connexion;
use serde_json::json;

pub async fn ajouter_projet(form: web::Json<Projet>) -> impl Responder {
    let mut projet = form.into_inner();

    let result = tokio::task::spawn_blocking(move ||{
        let mut client = database_connexion().map_err(|err| {
            eprintln!("Erreur de connexion à la base de données: {:?}", err);
            "Erreur de connexion à la base de données"
        })?;

        let query = 
    })
}


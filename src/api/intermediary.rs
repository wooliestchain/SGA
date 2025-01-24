use crate::models::projet::{self, Categorie};
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
        INSERT INTO Categorie
        "#
    })

}
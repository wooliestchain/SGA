use crate::models::utilisateur::{self, Utilisateurs};
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



pub async fn obtenir_utilisateur(id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let user_id = id.into_inner();

    // Créer une tâche asynchrone pour interagir avec la base de données
    let result = task::spawn_blocking(move || {
        // Connexion à la base de données
        let client = database_connexion().map_err(|err| {
            eprintln!("Erreur de connexion à la base de données : {:?}", err);
            HttpResponse::InternalServerError().body("Erreur de connexion à la base de données").into()
        })?;

        // Requête pour récupérer un utilisateur par ID
        let query = r#"
        SELECT id, nom, role, email, date_creation 
        FROM utilisateurs 
        WHERE id = $1;
        "#;

        client.query_one(query, &[&user_id]).map(|row| {
            // Mapper les données de l'utilisateur dans un struct Utilisateurs
            Utilisateurs {
                id: Some(row.get("id")),
                nom: row.get("nom"),
                role: row.get("role"),
                email: row.get("email"),
                date_creation: Some(row.get("date_creation")),
            }
        }).map_err(|err| {
            eprintln!("Erreur lors de la récupération de l'utilisateur : {:?}", err);
            HttpResponse::NotFound().body("Utilisateur non trouvé").into()
        })
    })
    .await;

    // Gestion du résultat
    match result {
        Ok(Ok(utilisateur)) => Ok(HttpResponse::Ok().json(utilisateur)),
        Ok(Err(err_response)) => Err(err_response),
        Err(err) => {
            eprintln!("Erreur lors de l'exécution de la tâche : {:?}", err);
            Err(HttpResponse::InternalServerError().body("Erreur interne du serveur").into())
        }
    }
}



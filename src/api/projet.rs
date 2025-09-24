use crate::models::projet::{Projet, ProjetInput};
use actix_web::{web, HttpResponse, Responder};
use super::dbconnect::database_connexion;
use serde_json::json;


//Fonction pour ajouter un projet 
pub async fn ajouter_projet(form: web::Json<ProjetInput>) -> impl Responder {
    let projet_data = form.into_inner();

    let result = tokio::task::spawn_blocking(move || {
        let mut client = database_connexion().map_err(|err| {
            eprintln!("Erreur de connexion à la base de données: {:?}", err);
            "Erreur de connexion à la base de données"
        })?;

        // 1. Insertion dans Projet
        let query_projet = r#"
            INSERT INTO Projet(nom, code, description, annee_debut, annee_fin, impact_attendu)
            VALUES($1,$2,$3,$4,$5,$6)
            RETURNING id;
        "#;

        let row = client.query_one(
            query_projet,
            &[
                &projet_data.nom,
                &projet_data.code,
                &projet_data.description,
                &projet_data.annee_debut,
                &projet_data.annee_fin,
                &projet_data.impact_attendu,
            ],
        ).map_err(|err| {
            eprintln!("Erreur lors de l'insertion du projet: {:?}", err);
            "Erreur lors de l'insertion du projet"
        })?;

        let projet_id: i32 = row.get("id");

        // 2. Insertion du statut si fourni
        if let Some(statut) = projet_data.statut {
            client.execute(
                "INSERT INTO Statut(projet_id, statut_actuel) VALUES($1, $2)",
                &[&projet_id, &statut],
            ).ok();
        }

        // 3. Insertion du type si fourni
        if let Some(type_projet) = projet_data.type_projet {
            client.execute(
                "INSERT INTO Type(projet_id, type) VALUES($1, $2)",
                &[&projet_id, &type_projet],
            ).ok();
        }

        // 4. Insertion de la priorité si fournie
        if let Some(priorite) = projet_data.priorite {
            client.execute(
                "INSERT INTO Priorite(projet_id, description) VALUES($1, $2)",
                &[&projet_id, &priorite],
            ).ok();
        }

        // 5. Insertion des objectifs si fournis
        if let Some(objectifs) = projet_data.objectifs {
            client.execute(
                "INSERT INTO Objectifs(projet_id, description) VALUES($1, $2)",
                &[&projet_id, &objectifs],
            ).ok();
        }

        // 6. Insertion des utilisateurs liés (table Projet_User)
        if let Some(utilisateurs) = projet_data.utilisateurs {
            for u in utilisateurs {
                client.execute(
                    "INSERT INTO Projet_User(projet_id, user_id, role) VALUES($1, $2, $3)
                     ON CONFLICT (projet_id, user_id) DO NOTHING",
                    &[&projet_id, &u.user_id, &u.role],
                ).ok();
            }
        }

        Ok::<String, &str>(format!("Projet ajouté avec l'ID: {}", projet_id))
    }).await;

    match result {
        Ok(Ok(response)) => HttpResponse::Ok().json(response),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(err.to_string()),
        Err(_) => HttpResponse::InternalServerError().body("Erreur interne du serveur"),
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
                    annee_debut: row.get("annee_debut"),
                    annee_fin: row.get("annee_fin"),
                    date_creation : row.get::<_, Option<String>>("date_creation"),
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Projet {
    pub id: Option<i32>,
    pub nom: String,
    pub code: i32,
    pub description: String,
    pub annee_debut: i32,
    pub annee_fin: i32,
    pub date_creation: Option<String>,
    pub impact_attendu: String,
    pub categories: Option<Vec<i32>>, // Ids des catégories liées au projet
    pub villes: Option<Vec<i32>>, // Ids des villes liées au projet
}

pub async fn ajouter_projet(form: web::Json<Projet>) -> impl Responder {
    let mut projet = form.into_inner();

    let result = tokio::task::spawn_blocking(move || {
        let mut client = database_connexion().map_err(|err| {
            eprintln!("Erreur de connexion à la base de données: {:?}", err);
            "Erreur de connexion à la base de données"
        })?;

        // Insertion dans la table Projet
        let query = r#"
        INSERT INTO Projet(nom, code, description, annee_debut, annee_fin, impact_attendu)
        VALUES($1, $2, $3, $4, $5, $6)
        RETURNING id;
        "#;

        match client.query_one(
            query,
            &[
                &projet.nom,
                &projet.code,
                &projet.description,
                &projet.annee_debut,
                &projet.annee_fin,
                &projet.impact_attendu,
            ],
        ) {
            Ok(row) => {
                let id: i32 = row.get("id");
                projet.id = Some(id);

                // Ajout des relations dans la table ProjetCategorie (pour les catégories liées au projet)
                if let Some(categories) = &projet.categories {
                    for categorie_id in categories {
                        let query_categorie = r#"
                        INSERT INTO ProjetCategorie(projet_id, categorie_id)
                        VALUES($1, $2);
                        "#;

                        if let Err(err) = client.execute(query_categorie, &[&id, &categorie_id]) {
                            eprintln!("Erreur lors de l'ajout de la catégorie : {:?}", err);
                            return Err("Erreur lors de l'ajout des catégories");
                        }
                    }
                }

                // Ajout des relations dans la table ProjetVille (si des villes sont associées)
                if let Some(villes) = &projet.villes {
                    for ville_id in villes {
                        let query_ville = r#"
                        INSERT INTO ProjetVille(projet_id, ville_id)
                        VALUES($1, $2);
                        "#;

                        if let Err(err) = client.execute(query_ville, &[&id, &ville_id]) {
                            eprintln!("Erreur lors de l'ajout de la ville : {:?}", err);
                            return Err("Erreur lors de l'ajout des villes");
                        }
                    }
                }

                Ok(format!("Projet ajouté avec l'ID: {}", id))
            }
            Err(err) => {
                eprintln!("Erreur lors de l'ajout du projet: {:?}", err);
                Err("Erreur lors de l'ajout du projet")
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

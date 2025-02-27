pub fn ajouter_categorie(client: &mut Client, categorie: &str) -> Result<i32, String> {
    let query = r#"
    INSERT INTO Categorie(nom)
    VALUES($1)
    RETURNING id;
    "#;

    match client.query_one(query, &[&categorie]) {
        Ok(row) => {
            let id: i32 = row.get("id");
            Ok(id)
        }
        Err(err) => {
            eprintln!("Erreur lors de l'ajout de la catégorie: {:?}", err);
            Err("Erreur lors de l'ajout de la catégorie".to_string())
        }
    }
}


pub fn ajouter_ville(client: &mut Client, ville: &str) -> Result<i32, String> {
    let query = r#"
    INSERT INTO Ville(nom)
    VALUES($1)
    RETURNING id;
    "#;

    match client.query_one(query, &[&ville]) {
        Ok(row) => {
            let id: i32 = row.get("id");
            Ok(id)
        }
        Err(err) => {
            eprintln!("Erreur lors de l'ajout de la ville: {:?}", err);
            Err("Erreur lors de l'ajout de la ville".to_string())
        }
    }
}


pub fn ajouter_projet_categorie(client: &mut Client, projet_id: i32, categorie_id: i32) -> Result<(), String> {
    let query = r#"
    INSERT INTO ProjetCategorie(projet_id, categorie_id)
    VALUES($1, $2);
    "#;

    match client.execute(query, &[&projet_id, &categorie_id]) {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("Erreur lors de l'ajout de la catégorie au projet: {:?}", err);
            Err("Erreur lors de l'ajout de la catégorie au projet".to_string())
        }
    }
}


pub fn ajouter_projet_ville(client: &mut Client, projet_id: i32, ville_id: i32) -> Result<(), String> {
    let query = r#"
    INSERT INTO ProjetVille(projet_id, ville_id)
    VALUES($1, $2);
    "#;

    match client.execute(query, &[&projet_id, &ville_id]) {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("Erreur lors de l'ajout de la ville au projet: {:?}", err);
            Err("Erreur lors de l'ajout de la ville au projet".to_string())
        }
    }
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

                // Ajout des catégories au projet
                if let Some(categories) = &projet.categories {
                    for categorie_nom in categories {
                        let categorie_id = ajouter_categorie(&mut client, categorie_nom)?;
                        ajouter_projet_categorie(&mut client, id, categorie_id)?;
                    }
                }

                // Ajout des villes au projet
                if let Some(villes) = &projet.villes {
                    for ville_nom in villes {
                        let ville_id = ajouter_ville(&mut client, ville_nom)?;
                        ajouter_projet_ville(&mut client, id, ville_id)?;
                    }
                }

                Ok(format!("Projet ajouté avec l'ID: {}", id))
            }
            Err(err) => {
                eprintln!("Erreur lors de l'ajout du projet: {:?}", err);
                Err("Erreur lors de l'ajout du projet".to_string())
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

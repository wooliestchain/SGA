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
        Ok(Err(err_response)) => err_response,
        Err(_) => HttpResponse::InternalServerError().body("Erreur interne du serveur"),
    }
}
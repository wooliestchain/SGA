use axum::{Router, routing::get};
use postgres::{Client, NoTls, Error};
mod models;
use models::projet::Projet;

mod api;
use api::utilisateur;

/* fn main() -> Result<(), Error> {

    let mut projet1 = Projet {
        id: 1,
        nom: String::from("Projet A"),
        description: String::from("Description du projet A"),
        ministere_responsable: String::from("Ministère A"),
        referant_projet: String::from("Référant Projet A"),
        referant_presidence: String::from("Référant Présidence A"),
        ville: String::from("Ville A"),
        categorie: String::from("Catégorie A"),
        annee_debut: 2022,
        annee_fin: 2025,
        type_projet: String::from("Type A"),
        personnelles_alloues: String::from("Personnelles A"),
        infrastructures_mobilises: String::from("Infrastructures A"),
        fonds_decaisses: 1000.0,
        source_financement: String::from("Source A"),
        date_creation: String::from("2022-01-01"),
        statut: String::from("En cours"),
        priorite: String::from("Haute"),
        risques_associes: String::from("Risques A"),
        objectifs: String::from("Objectifs A"),
        impact_attendu: String::from("Impact A"),
        commentaires: String::from("Commentaires A"),
    };


    let mut client = Client::connect("host=localhost user=postgres password=root", NoTls)?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS person (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL,
            data    BYTEA
        )
    ")?;

    let name = projet1.nom;
    let data: Option<Vec<u8>> = None; // Utilisez Option<Vec<u8>> pour gérer les données binaires nulles
    client.execute(
        "INSERT INTO person (name, data) VALUES ($1, $2)",
        &[&name, &data], // Pas besoin de casting explicite ici
    )?;

    for row in client.query("SELECT id, name, data FROM person", &[])? {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<Vec<u8>> = row.get(2); // Récupérez le champ BYTEA comme Option<Vec<u8>>

        println!("found person: {} {} {:?}", id, name, data);
    }

    Ok(())
}
 */

#[tokio::main]
async fn main(){
    let app = Router::new().route("/", get(|| async{"Hello,"}));

    let listener =tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
 }

 async fn hello() -> &'static str{
    "Hello, world!"
}
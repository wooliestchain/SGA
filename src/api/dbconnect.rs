use postgres::{Client, NoTls, Error};

pub fn database_connexion() -> Result<Client, Error> {
    let client = Client::connect(
        "host=localhost user=postgres password=root dbname=projet_sga",
        NoTls,
    ).map_err(|err| {
        eprintln!("Erreur de connexion à la base de données: {:?}", err);
        err
    })?;

    println!("Connexion à la base de données réussie");
    Ok(client)
}

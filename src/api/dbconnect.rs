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

/* 
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn database_connexion() -> DbPool {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
*/
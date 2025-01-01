use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize,Deserialize)]
pub struct utilisateurs{
    pub id: i32,
    pub nom: String,
    pub role: String,
    pub email: String,
    pub date_creation: DateTime<Utc>,
    pub derniere_connexion: DateTime<Utc>
}
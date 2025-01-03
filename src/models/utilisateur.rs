use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::projet;

#[derive(Debug, Serialize,Deserialize)]
pub struct Utilisateurs {
    pub id: i32,
    pub nom: String,
    pub role: String,
    pub email: String,
    pub date_creation: DateTime<Utc>,
    pub derniere_connexion: DateTime<Utc>
}
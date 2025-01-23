use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime};

#[derive(Debug, Serialize, Deserialize)]

pub struct Commentaire{
    pub id: i32,
    pub projet_id: i32,
    pub utilisateur_id: i32,
    pub contenu: String,
    pub date_creation: Option<NaiveDateTime>
}
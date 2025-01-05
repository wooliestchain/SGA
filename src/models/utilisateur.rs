use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::projet;

#[derive(Debug, Serialize,Deserialize)]
pub struct Utilisateurs {
    pub nom: String,
    pub role: String,
    pub email: String,
    pub date_creation: Option <DateTime<Utc>>,
}
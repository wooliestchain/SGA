use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::utilisateur;

#[derive(Debug, Serialize,Deserialize)]
pub struct logs_activities{
  id : i32,
  utilisateur_id: Utilisateurs::id,
  type_activite: String,
  description: String,
  date_heure: DateTime<Utc>
}

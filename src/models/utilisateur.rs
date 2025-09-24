use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::projet;

#[derive(Debug, Serialize,Deserialize)]
pub struct Utilisateurs {
    pub id: Option<i32>,
    pub nom: String,
    pub role: String,
    pub email: String,
    pub password: String,
    pub date_creation: Option <String>,
}

pub struct Role{
    pub id: i32,
    pub nom: String
}

pub struct UserRole{
    pub user_id: i32,
    pub role_id: i32
}


/* 
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use crate::schema::utilisateurs;

#[derive(Queryable, Serialize)]
pub struct Utilisateur {
    pub id: i32,
    pub nom: String,
    pub role: String,
    pub email: String,
    pub password: String,
    pub date_creation: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = utilisateurs)]
pub struct NewUtilisateur {
    pub nom: String,
    pub role: String,
    pub email: String,
    pub password: String,
    pub date_creation: Option<NaiveDateTime>,
}
*/
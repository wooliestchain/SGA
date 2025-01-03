use crate::models::utilisateur::Utilisateurs;
use postgres::{Client, NoTls, Error};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use chrono::{DateTime, Utc};




pub fn add_user() -> Result<(), Error>{

    let mut client = Client::connect("host=localhost user=postgres password=root database=projet_sga", NoTls);
}
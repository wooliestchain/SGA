use crate::models::projet::{self, Projet};
use axum::http::response;
use postgres::{row, Client, NoTls};
use actix_web::{web, HttpResponse, Responder, Error};
use actix_web::error::ErrorInternalServerError;
use chrono::Utc;
use tokio::task;
use super::dbconnect::database_connexion;
use serde_json::json;
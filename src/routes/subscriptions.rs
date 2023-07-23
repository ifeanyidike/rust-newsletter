//! src/routes/subscriptions.rs
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgConnection;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    _form: web::Form<FormData>,
    _connection: web::Data<PgConnection>,
) -> HttpResponse {
    HttpResponse::Ok().finish()
}

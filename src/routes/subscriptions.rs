//! src/routes/subscriptions.rs
use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{postgres::PgQueryResult, Error, PgPool};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    if let Err(error) = handle_subscription(&form, pool.as_ref()).await {
        tracing::error!("Failed to execute query: {:?}", error);
        return HttpResponse::InternalServerError().finish();
    }

    tracing::info!("New subscriber details have been saved");
    HttpResponse::Ok().finish()
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]

async fn handle_subscription(form: &FormData, pool: &PgPool) -> Result<PgQueryResult, Error> {
    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|err| {
        tracing::error!("Error executing query: {:?}", err);
        err
    })?;

    Ok(result)
}

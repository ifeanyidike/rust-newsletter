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

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // Generate a random unique identifier (request_id or correlation_id) for logging
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );

    let _request_span_guard = request_span.enter();

    tracing::info!(
        "request_id {} - Saving new subscriber details in the database",
        request_id
    );

    if let Err(error) = handle_subscription(&form, pool.as_ref()).await {
        tracing::error!("Failed to execute query: {:?}", error);
        return HttpResponse::InternalServerError().finish();
    }

    tracing::info!("New subscriber details have been saved");
    HttpResponse::Ok().finish()
}

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
        eprintln!("Error executing query: {:?}", err);
        err
    })?;

    Ok(result)
}

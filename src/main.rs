//! main.rs
//!
use newsletter::startup::run;
use newsletter::telemetry::init_subscriber;
use newsletter::{configuration::get_configuration, telemetry::get_subscriber};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let subscriber = get_subscriber(
        "newsletter".to_string(),
        "info".to_string(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        PgPool::connect(&configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres.");

    let addrs = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(addrs)?;

    let server = run(listener, connection_pool).await?;
    server.await
}

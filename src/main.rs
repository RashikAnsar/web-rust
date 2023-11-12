use std::net::TcpListener;

use sqlx::PgPool;
use web_rust_prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address.clone())?;
    println!(
        "server running on: http://{}:{}",
        address, configuration.application_port
    );
    run(listener, connection_pool)?.await
}

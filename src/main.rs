use sqlx::PgPool;
use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};


#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Redirect to subscriber
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);
    
    // Display all spans at "info" level or higher
    
    let configuration = get_configuration().expect("Failed to retrieve configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");

    // Retrieve port from the listener
    let port = listener.local_addr().unwrap().port();
    run(listener, connection_pool)?.await
}

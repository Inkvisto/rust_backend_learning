use std::{io::Result, net::TcpListener};

use zero_to_prod::run;

#[actix_web::main]
async fn main() -> Result<()> {
      let database_url = dotenvy::var("DATABASE_URL")
        .context("DATABASE_URL must be set")?;

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .context("failed to connect to DATABASE_URL")?;
    
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    run(listener)?.await
}

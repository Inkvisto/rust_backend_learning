mod routes;

use routes::create_router;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let app = create_router();
    let server = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(server, app).await?;
    Ok(())
}

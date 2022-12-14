use std::net::SocketAddr;

use anyhow::{Context as _, Result};
use axum::{Extension, Router, Server};
use gracroverse_server_driver::routes;
use gracroverse_server_driver::startup::init_app;
use tower::ServiceBuilder;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<()> {
    let config = init_app()?;

    let repository_module = gracroverse_server_driver::modules::repositories();

    let app = Router::new().nest("/ap", routes::ap::router()).layer(
        ServiceBuilder::new()
            .layer(Extension(repository_module))
            .layer(TraceLayer::new_for_http())
            .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
            .layer(PropagateRequestIdLayer::x_request_id()),
    );

    let host = config.host.parse::<SocketAddr>()?;
    tracing::info!("Listening server on {}", host);
    Server::bind(&host)
        .serve(app.into_make_service())
        .await
        .context("Could not start server")
        .unwrap();

    Ok(())
}

use std::sync::Arc;

use axum::Router;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use tower_http::trace::TraceLayer;

use crate::conf::Conf;

pub struct AppState {
    pub conf: Arc<Conf>,
    pub pool: MySqlPool,
}

impl AppState {
    pub async fn new(conf: &Conf) -> anyhow::Result<Self> {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&conf.db.url())
            .await?;

        Ok(Self {
            conf: Arc::new(conf.clone()),
            pool,
        })
    }
}

pub async fn serve(port: u16, conf: &Conf) -> anyhow::Result<()> {
    let state = Arc::new(AppState::new(conf).await?);
    let app = Router::new()
        // .nest("/api", api::setup())
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::clone(&state));

    let addr = format!("localhost:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}

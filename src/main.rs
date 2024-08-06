use std::sync::Arc;

use anyhow::Result;
use axum::{
    routing::{delete, get, post, put},
    serve, Router,
};
use handlers::{delete_value_from_map, get_value_from_map, set_value_in_map, update_value_in_map};
use hashmap::HashMap;
use simple_logger::SimpleLogger;
use tokio::net::TcpListener;

mod handlers;
mod hashmap;

#[tokio::main]
async fn main() -> Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()?;

    let pg_pool = sqlx::PgPool::connect(env!("DATABASE_URL"))
        .await
        .expect("Failed to connect to database");

    let map = HashMap::new(pg_pool);

    let arc_map = Arc::new(map);

    let app = Router::new()
        .route("/api/map/:key", get(get_value_from_map))
        .route("/api/map/:key", post(set_value_in_map))
        .route("/api/map/:key", put(update_value_in_map))
        .route("/api/map/:key", delete(delete_value_from_map))
        .with_state(arc_map);

    let listner = TcpListener::bind("0.0.0.0:5000").await?;
    serve(listner, app).await?;

    Ok(())
}

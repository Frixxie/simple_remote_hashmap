use std::sync::Arc;

use axum::body::Bytes;
use axum::http::StatusCode;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use log::{error, info};

use crate::hashmap::HashMap;

pub async fn get_value_from_map(
    Path(key): Path<String>,
    State(map): State<Arc<HashMap>>,
) -> impl IntoResponse {
    info!("GET, with key {}", key);
    map.get(&key)
        .await
        .map(|value| (StatusCode::OK, value.clone()))
        .unwrap_or_else(|| (StatusCode::NOT_FOUND, vec![]))
}

pub async fn set_value_in_map(
    Path(key): Path<String>,
    State(map): State<Arc<HashMap>>,
    body: Bytes,
) -> impl IntoResponse {
    info!("POST, with key {}", key);
    map.set(&key, body.to_vec())
        .await
        .map(|_| (StatusCode::ACCEPTED, "OK".to_string()))
        .map_err(|e| {
            let error_string = e.to_string();
            error!("{}", error_string);
            (StatusCode::INTERNAL_SERVER_ERROR, error_string)
        })
}

pub async fn update_value_in_map(
    Path(key): Path<String>,
    State(map): State<Arc<HashMap>>,
    body: Bytes,
) -> impl IntoResponse {
    info!("PUT, with key {}", key);
    map.update(&key, body.to_vec())
        .await
        .map(|_| (StatusCode::ACCEPTED, "OK".to_string()))
        .map_err(|e| {
            let error_string = e.to_string();
            error!("{}", error_string);
            (StatusCode::INTERNAL_SERVER_ERROR, error_string)
        })
}

pub async fn delete_value_from_map(
    Path(key): Path<String>,
    State(map): State<Arc<HashMap>>,
) -> impl IntoResponse {
    info!("DELETE, with key {}", key);
    map.delete(&key)
        .await
        .map(|_| (StatusCode::ACCEPTED, "OK".to_string()))
        .map_err(|e| {
            let error_string = e.to_string();
            error!("{}", error_string);
            (StatusCode::INTERNAL_SERVER_ERROR, error_string)
        })
}

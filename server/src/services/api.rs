use std::sync::{Arc, LazyLock};

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use tokio::sync::{Mutex, MutexGuard};

use crate::{models::{pessoa::Cliente, plano::Plano}, GlobalState};

pub async fn assinar_plano(
    State(global_state): State<Arc<Mutex<GlobalState>>>,
    Path(plano_repr): Path<u8>,
    Json(cliente): Json<Cliente>,
) -> impl IntoResponse {
    let mut global_state_guard = global_state.lock().await;
    let plano = Arc::clone(
        global_state_guard.planos_disponiveis.get(plano_repr as usize).unwrap()
    );
    global_state_guard.assinaturas.insert(cliente, plano);

    StatusCode::OK
}

pub async fn cancelar_plano(
    State(global_state): State<Arc<Mutex<GlobalState>>>,
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn get_plano(
    State(global_state): State<Arc<Mutex<GlobalState>>>,
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn melhorar_plano(
    State(global_state): State<Arc<Mutex<GlobalState>>>,
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn rebaixar_plano(
    State(global_state): State<Arc<Mutex<GlobalState>>>,
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
use std::sync::{Arc, LazyLock};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use tokio::sync::{Mutex, MutexGuard};

use crate::{
    GlobalState,
    models::{pessoa::Cliente, plano::Plano},
};

pub async fn assinar_plano(
    State(global_state): State<Arc<Mutex<GlobalState>>>,
    Path(plano_repr): Path<u8>,
    Json(cliente): Json<Cliente>,
) -> impl IntoResponse {
    let mut global_state_guard = global_state.lock().await;

    if let Some(v) = global_state_guard.assinaturas.get(&cliente) {
        return "Cliente já assinou um plano".to_string()
    }

    let plano = Arc::clone(
        match global_state_guard
            .planos_disponiveis
            .get(plano_repr as usize)
        {
            Some(v) => v,
            None => return "Plano não encontrado".to_string(),
        },
    );

    let plano_repr = match plano_repr {
        0 => "ultra",
        1 => "premium",
        2 => "basico",
        _ => unreachable!(),
    };

    global_state_guard
        .assinaturas
        .insert(cliente, plano);

    format!("Plano assinado: {}", plano_repr)
}

pub async fn cancelar_plano(
    State(global_state): State<Arc<Mutex<GlobalState>>>,
    Path(plano_repr): Path<u8>,
    Json(cliente): Json<Cliente>,
) -> impl IntoResponse {
    let mut global_state_guard = global_state.lock().await;
    let plano = Arc::clone(
        global_state_guard
            .planos_disponiveis
            .get(plano_repr as usize)
            .unwrap(),
    );

    global_state_guard.assinaturas.remove(&cliente);

    StatusCode::NOT_IMPLEMENTED
}

pub async fn get_plano(
    State(global_state): State<Arc<Mutex<GlobalState>>>,
    Json(cliente): Json<Cliente>
) -> impl IntoResponse {
    let mut global_state_guard = global_state.lock().await;

    let plano = global_state_guard.assinaturas.get(&cliente).unwrap();
}

pub async fn melhorar_plano(
    State(global_state): State<Arc<Mutex<GlobalState>>>,
    Json(cliente): Json<Cliente>
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn rebaixar_plano(
    State(global_state): State<Arc<Mutex<GlobalState>>>,
    Json(cliente): Json<Cliente>
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

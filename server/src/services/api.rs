use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use tokio::sync::Mutex;

use crate::{
    models::pessoa::Cliente,
    GlobalState,
};

#[derive(Serialize)]
struct Mensagem {
    mensagem: String,
}

pub async fn assinar_plano(
    State(global_state): State<Arc<Mutex<GlobalState>>>,
    Path(plano_idx): Path<usize>,
    Json(cliente): Json<Cliente>,
) -> impl IntoResponse {
    let mut global_state_guard = global_state.lock().await;

    if global_state_guard.assinaturas.contains_key(&cliente) {
        return (
            StatusCode::BAD_REQUEST,
            Json(Mensagem {
                mensagem: "Cliente já possui um plano ativo.".to_string(),
            }),
        )
            .into_response();
    }

    let plano_a_assinar = match global_state_guard.planos_disponiveis.get(plano_idx) {
        Some(p) => Arc::clone(p),
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(Mensagem {
                    mensagem: "Plano não encontrado.".to_string(),
                }),
            )
                .into_response();
        }
    }; 

    global_state_guard
        .assinaturas
        .insert(cliente, plano_a_assinar.clone());

    let plano_assinado = plano_a_assinar.lock().await;
    (
        StatusCode::OK,
        Json(Mensagem {
            mensagem: format!("Plano assinado: {}", plano_assinado.get_name()),
        }),
    )
        .into_response()
}

pub async fn cancelar_plano(
    State(global_state): State<Arc<Mutex<GlobalState>>>,
    Json(cliente): Json<Cliente>,
) -> impl IntoResponse {
    let mut global_state_guard = global_state.lock().await;

    if global_state_guard.assinaturas.remove(&cliente).is_some() {
        (
            StatusCode::OK,
            Json(Mensagem {
                mensagem: "Plano cancelado com sucesso.".to_string(),
            }),
        )
            .into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(Mensagem {
                mensagem: "Cliente não possui um plano para cancelar.".to_string(),
            }),
        )
            .into_response()
    }
}

pub async fn get_plano(
    State(global_state): State<Arc<Mutex<GlobalState>>>,
    Json(cliente): Json<Cliente>,
) -> impl IntoResponse {
    let global_state_guard = global_state.lock().await;

    match global_state_guard.assinaturas.get(&cliente) {
        Some(plano_mutex) => {
            let plano = plano_mutex.lock().await;
            (
                StatusCode::OK,
                Json(Mensagem {
                    mensagem: format!("{:?}", plano.clone()), 
                }),
            )
                .into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(Mensagem {
                mensagem: "Cliente não possui um plano.".to_string(),
            }),
        )
            .into_response(),
    }
}

pub async fn melhorar_plano(
    State(global_state): State<Arc<Mutex<GlobalState>>>,
    Json(cliente): Json<Cliente>,
) -> impl IntoResponse {
    let mut global_state_guard = global_state.lock().await;

    let plano_atual_mutex = match global_state_guard.assinaturas.get(&cliente) {
        Some(p) => Arc::clone(p),
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(Mensagem {
                    mensagem: "Cliente não encontrado para melhorar o plano.".to_string(),
                }),
            )
                .into_response();
        }
    };

    let plano_atual_id = plano_atual_mutex.lock().await.get_id();

    let novo_plano_idx = match plano_atual_id {
        2 => 1,
        1 => 0,
        0 => {
            return (
                StatusCode::BAD_REQUEST,
                Json(Mensagem {
                    mensagem: "Cliente já possui o plano mais alto.".to_string(),
                }),
            )
                .into_response();
        }
        _ => unreachable!(),
    };

    let novo_plano = Arc::clone(&global_state_guard.planos_disponiveis[novo_plano_idx]);
    global_state_guard
        .assinaturas
        .insert(cliente, novo_plano.clone());
    let plano_info = novo_plano.lock().await;
    (
        StatusCode::OK,
        Json(Mensagem {
            mensagem: format!("Plano melhorado para: {}", plano_info.get_name()),
        }),
    )
        .into_response()
}

pub async fn rebaixar_plano(
    State(global_state): State<Arc<Mutex<GlobalState>>>,
    Json(cliente): Json<Cliente>,
) -> impl IntoResponse {
    let mut global_state_guard = global_state.lock().await;

    let plano_atual_mutex = match global_state_guard.assinaturas.get(&cliente) {
        Some(p) => Arc::clone(p),
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(Mensagem {
                    mensagem: "Cliente não encontrado para rebaixar o plano.".to_string(),
                }),
            )
                .into_response();
        }
    };

    let plano_atual_id = plano_atual_mutex.lock().await.get_id();

    let novo_plano_idx = match plano_atual_id {
        0 => 1,
        1 => 2,
        2 => {
            return (
                StatusCode::BAD_REQUEST,
                Json(Mensagem {
                    mensagem: "Cliente já possui o plano mais básico.".to_string(),
                }),
            )
                .into_response();
        }
        _ => unreachable!(),
    };

    let novo_plano = Arc::clone(&global_state_guard.planos_disponiveis[novo_plano_idx]);
    global_state_guard
        .assinaturas
        .insert(cliente, novo_plano.clone());
    let plano_info = novo_plano.lock().await;
    (
        StatusCode::OK,
        Json(Mensagem {
            mensagem: format!("Plano rebaixado para: {}", plano_info.get_name()),
        }),
    )
        .into_response()
}
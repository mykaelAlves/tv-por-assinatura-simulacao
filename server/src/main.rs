use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};

use axum::{
    Router,
    routing::{delete, get, post, put},
    serve,
};
use tokio::{
    net::TcpListener,
    sync::{Mutex, RwLock},
};
use tower_http::cors::{Any, CorsLayer};

use server::{
    GlobalState,
    models::{
        pessoa::Cliente,
        plano::{Beneficios, Plano, PlanoInfo},
    },
    services,
};

type GenericError = Box<dyn std::error::Error>;

static GLOBAL_STATE: LazyLock<Arc<Mutex<GlobalState>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(GlobalState {
        planos_disponiveis: vec![
            Arc::new(Mutex::new(Plano::Ultra(PlanoInfo {
                beneficios: vec![
                    Beneficios::CanalEsportes,
                    Beneficios::CanalDesenhos,
                    Beneficios::CanalNoticias,
                    Beneficios::CanalAberto,
                    Beneficios::Suporte24Horas,
                    Beneficios::ParticipacaoSorteios,
                ],
            }))),
            Arc::new(Mutex::new(Plano::Premium(PlanoInfo {
                beneficios: vec![
                    Beneficios::CanalDesenhos,
                    Beneficios::CanalNoticias,
                    Beneficios::CanalAberto,
                    Beneficios::ParticipacaoSorteios,
                ],
            }))),
            Arc::new(Mutex::new(Plano::Basico(PlanoInfo {
                beneficios: vec![
                    Beneficios::CanalDesenhos,
                    Beneficios::CanalAberto,
                ],
            }))),
        ],
        assinaturas: HashMap::new(),
    }))
});

#[tokio::main]
async fn main() -> Result<(), GenericError> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    println!("Criando rotas...");
    let app: Router = axum::Router::new()
        .route("/", get(services::root))
        .route("/api/assinarPlano/:plano_repr", post(services::api::assinar_plano))
        .route("/api/cancelarPlano", delete(services::api::cancelar_plano))
        .route("/api/getPlano", get(services::api::get_plano))
        .route("/api/melhorarPlano", put(services::api::melhorar_plano))
        .route("/api/rebaixarPlano", put(services::api::rebaixar_plano))
        .with_state(GLOBAL_STATE.clone())
        .layer(cors);

    println!("Criando listener...");
    let listener = TcpListener::bind("127.0.0.1:9999").await?;

    println!("Servindo rotas...");
    serve(listener, app.into_make_service()).await?;

    Ok(())
}

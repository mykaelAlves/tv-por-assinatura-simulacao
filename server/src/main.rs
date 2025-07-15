use axum::{
    routing::{
        get, post, put, delete
    },
    Router,
    serve
};
use tokio::{net::TcpListener, sync::Mutex};
use tower_http::cors::{Any, CorsLayer};

use server::{models::plano::Plano, services};

type GenericError = Box<dyn std::error::Error>;

struct GlobalState {
    planos_disponiveis: Mutex<Vec<Plano>>,
}

#[tokio::main]
async fn main() -> Result<(), GenericError>{
    let cors = CorsLayer::new()
        .allow_origin(Any) 
        .allow_methods(Any) 
        .allow_headers(Any); 

    println!("Criando rotas...");
    let app: Router = axum::Router::new()
        .route("/", get(services::root))
        .layer(cors);

    println!("Criando listener...");
    let listener = TcpListener::bind("127.0.0.1:9999").await?;
    println!("Servindo rotas...");
    serve(listener, app.into_make_service()).await?;

    Ok(())
}

use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::models::{pessoa::Cliente, plano::Plano};

pub mod models;
pub mod services;

pub struct GlobalState {
    pub assinaturas: HashMap<Cliente, Arc<Mutex<Plano>>>,
    pub planos_disponiveis: Vec<Arc<Mutex<Plano>>>,
}

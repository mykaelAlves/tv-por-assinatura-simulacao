use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Cliente {
    id: u16,
    nome: String,
}

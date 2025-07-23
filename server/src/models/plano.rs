use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
pub enum Plano {
    Ultra(PlanoInfo),
    Premium(PlanoInfo),
    Basico(PlanoInfo),
}

impl Plano {
    pub fn get_id(&self) -> u8 {
        match self {
            Plano::Ultra(info) => info.id,
            Plano::Premium(info) => info.id,
            Plano::Basico(info) => info.id,
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            Plano::Ultra(_) => "Ultra",
            Plano::Premium(_) => "Premium",
            Plano::Basico(_) => "Basico",
        }
    }
}

#[derive(Clone, Serialize, Debug)]
pub struct PlanoInfo {
    pub id: u8,
    pub beneficios: Vec<Beneficios>,
}

#[derive(Clone, Serialize, Debug)]
pub enum Beneficios {
    CanalEsportes,
    CanalNoticias,
    CanalDesenhos,
    CanalAberto,
    Suporte24Horas,
    ParticipacaoSorteios,
}
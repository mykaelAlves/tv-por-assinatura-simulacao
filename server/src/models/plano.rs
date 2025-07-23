use std::fmt::Display;

pub enum Plano {
    Ultra(PlanoInfo),
    Premium(PlanoInfo),
    Basico(PlanoInfo),
}

pub struct PlanoInfo {
    pub id: u8,
    pub beneficios: Vec<Beneficios>,
}

pub enum Beneficios {
    CanalEsportes,
    CanalNoticias,
    CanalDesenhos,
    CanalAberto,
    Suporte24Horas,
    ParticipacaoSorteios,
}

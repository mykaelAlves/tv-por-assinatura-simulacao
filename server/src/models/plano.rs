pub enum Plano {
    Ultra(PlanoInfo),
    Premium(PlanoInfo),
    Basico(PlanoInfo),
}

pub struct PlanoInfo {
    pub beneficios: Vec<Beneficios>,
}

pub enum Beneficios {
    CanalEsportes,
    CanalNoticias,
}
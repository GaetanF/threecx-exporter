use serde::Deserialize;
use once_cell::sync::Lazy;
use arc_swap::ArcSwap;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Server {
    pub host: String,
    pub secret: String,
    pub leoid: String
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Cfg {
    pub interval: u64,
    pub threads: usize,
    pub listen: u16,
    pub servers: Vec<Server>
}

pub static DEFAULT_CFG: &str = r#"
interval = 2
threads = 2
listen = 5667
servers = []
"#;

pub static CONFIG: Lazy<ArcSwap<Cfg>> = Lazy::new(Default::default);
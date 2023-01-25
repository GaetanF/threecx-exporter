mod collector;
mod handlers;
mod config;
mod threecx;
mod errors;
mod model;

#[macro_use]
extern crate lazy_static;
use crate::config::{Cfg, DEFAULT_CFG, CONFIG};

use warp::Filter;
use std::sync::mpsc::{self};
use tokio::task::JoinHandle;
use tokio::sync::oneshot;
use log::{debug, info};
use spirit::prelude::*;
use spirit::{extension, AnyError};
use spirit::{Empty, Spirit};

#[derive(Debug, Clone)]
pub struct ThreeCXConfiguration {
    pub host: String,
    pub username: String,
    pub password: String
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let (main_collector_tx, main_collector_rx) = mpsc::channel();
    let (web_server_tx, web_server_rx) = oneshot::channel();
    collector::register_custom_metrics();

    let _spirit = Spirit::<Empty, Cfg>::new()
        .with(spirit_cfg_helpers::cfg_store(&*CONFIG))
        .config_defaults(DEFAULT_CFG)
        .config_exts(["json"])
        .config_env("3CX_EXPORTER")
        .with(extension::immutable_cfg(
            |cfg: &Cfg| &cfg.listen,
            "listen ports",
        ))
        .on_config(|_opts, cfg| debug!("New config loaded: {:?}", cfg))
        .on_terminate(move || {
            info!("Terminating main collector thread");
            let _ = main_collector_tx.send(());
            let _ = web_server_tx.send(());
        })
        .build(true);

    tokio::task::spawn(async move {
        info!("Start main collector thread");
        collector::main_collector(main_collector_rx).await
    });

    let conf = CONFIG.load();
    let port = conf.listen;

    let metrics_route = warp::path!("metrics").and_then(handlers::metrics_handler);
    let (addr, server) = warp::serve(metrics_route)
        .bind_with_graceful_shutdown(([127, 0, 0, 1], port),  async move {
            web_server_rx.await.ok();
            info!("Graceful shutdown");
        });

    info!("Started on port {}", port);
    tokio::task::spawn(server).await.expect("TODO: panic message");
}
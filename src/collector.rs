use std::error::Error;
use std::sync::Arc;
use crate::config::{Cfg, Server, CONFIG};
use std::time::Duration;
use prometheus::{
    IntGaugeVec, Opts, Registry
};
use std::sync::mpsc::{channel, TryRecvError, Receiver};
use threadpool::ThreadPool;
use log::{debug, info};
use crate::{ThreeCXConfiguration, threecx};

lazy_static! {
    pub static ref BLACKLIST_SIZE: IntGaugeVec = IntGaugeVec::new(
        Opts::new("blacklist_size", "Number of blacklisted IP addresses"),
        &["host", "ipaddress", "leoid"]
    ).expect("metric can be created");
    pub static ref CALLS_ACTIVE: IntGaugeVec = IntGaugeVec::new(
        Opts::new("calls_active", "Number of current active calls"),
        &["host", "ipaddress", "leoid"]
    ).expect("metric can be created");
    pub static ref CALLS_LIMIT: IntGaugeVec = IntGaugeVec::new(
        Opts::new("calls_limit", "Maximum number of supported simultaneous calls"),
        &["host", "ipaddress", "leoid"]
    ).expect("metric can be created");
    pub static ref EXTENSIONS_TOTAL: IntGaugeVec = IntGaugeVec::new(
        Opts::new("extensions_total", "Number of total extensions"),
        &["host", "ipaddress", "leoid"]
    ).expect("metric can be created");
    pub static ref EXTENSIONS_REGISTERED: IntGaugeVec = IntGaugeVec::new(
        Opts::new("extensions_registered", "Number of registered extensions"),
        &["host", "ipaddress", "leoid"]
    ).expect("metric can be created");
    pub static ref SERVICE_STATUS: IntGaugeVec = IntGaugeVec::new(
        Opts::new("service_status", "Status of service"),
        &["host", "ipaddress", "leoid", "service_name"]
    ).expect("metric can be created");
    pub static ref SERVICE_CPU: IntGaugeVec = IntGaugeVec::new(
        Opts::new("service_cpu", "CPU usage of service"),
        &["host", "ipaddress", "leoid", "service_name"]
    ).expect("metric can be created");
    pub static ref SERVICE_MEMORY: IntGaugeVec = IntGaugeVec::new(
        Opts::new("service_memory", "Memory usage of service"),
        &["host", "ipaddress", "leoid", "service_name"]
    ).expect("metric can be created");
    pub static ref TRUNK_REGISTERED: IntGaugeVec = IntGaugeVec::new(
        Opts::new("trunk_registered", "Status of trunk"),
        &["host", "ipaddress", "leoid"]
    ).expect("metric can be created");

    pub static ref REGISTRY: Registry = Registry::new();
}

pub(crate) fn register_custom_metrics() {
    REGISTRY
        .register(Box::new(BLACKLIST_SIZE.clone()))
        .expect("collector can be registered");

    REGISTRY
        .register(Box::new(CALLS_ACTIVE.clone()))
        .expect("collector can be registered");

    REGISTRY
        .register(Box::new(CALLS_LIMIT.clone()))
        .expect("collector can be registered");

    REGISTRY
        .register(Box::new(EXTENSIONS_TOTAL.clone()))
        .expect("collector can be registered");

    REGISTRY
        .register(Box::new(EXTENSIONS_REGISTERED.clone()))
        .expect("collector can be registered");

    REGISTRY
        .register(Box::new(SERVICE_STATUS.clone()))
        .expect("collector can be registered");

    REGISTRY
        .register(Box::new(SERVICE_CPU.clone()))
        .expect("collector can be registered");

    REGISTRY
        .register(Box::new(SERVICE_MEMORY.clone()))
        .expect("collector can be registered");

    REGISTRY
        .register(Box::new(TRUNK_REGISTERED.clone()))
        .expect("collector can be registered");
}

async fn threecx_collector(server: Server)-> Result<u8, u8> {
    let host = server.host.clone();
    println!("Collect 3CX {}", host);
    let client = threecx::ThreeCXClient::new(ThreeCXConfiguration {
        host: host.clone(),
        username: "admin".parse().unwrap(),
        password: server.secret.clone()
    });
    let resp_login = client.login().await;
    if resp_login.is_err() {
        println!("{:?}", resp_login.err());
        return Err(1);
    }

    let resp_server_status = client.get_server_status().await;

    if resp_server_status.is_err() {
        println!("{:?}", resp_server_status.err());
        return Err(1);
    }

    let server_status = resp_server_status.ok().unwrap();
    let labels = [
        host.as_str(),
        server_status.IpV4.as_str(),
        server.leoid.as_str()
    ];

    TRUNK_REGISTERED.with_label_values(&labels).set(server_status.TrunksRegistered as i64);

    CALLS_ACTIVE.with_label_values(&labels).set(server_status.CallsActive as i64);
    CALLS_LIMIT.with_label_values(&labels).set(server_status.MaxSimCalls as i64);

    EXTENSIONS_TOTAL.with_label_values(&labels).set(server_status.ExtensionsTotal as i64);
    EXTENSIONS_REGISTERED.with_label_values(&labels).set(server_status.ExtensionsRegistered as i64);

    let resp_service_list = client.get_service_list().await;

    if resp_service_list.is_err() {
        println!("{:?}", resp_service_list.err());
        return Err(1);
    }

    let service_list = resp_service_list.ok().unwrap();

    for service in service_list {
        let labels = [
            host.as_str(),
            server_status.IpV4.as_str(),
            server.leoid.as_str(),
            service.Name.as_str()
        ];

        SERVICE_CPU.with_label_values(&labels).set(service.CpuUsage as i64);
        SERVICE_MEMORY.with_label_values(&labels).set(service.MemoryUsed as i64);
        SERVICE_STATUS.with_label_values(&labels).set(service.Status as i64);
    }

    Ok(0)
}

pub(crate) async fn main_collector<T>(rx: Receiver<T>) {
    let mut config = CONFIG.load();
    let mut collect_interval = tokio::time::interval(Duration::from_secs(config.interval));
    loop {
        collect_interval.tick().await;

        config = CONFIG.load();
        debug!("Configure Thread pool to {} parallel thread", config.threads);
        let pool = ThreadPool::new(config.threads);
        info!("Start collector for {} 3CX Servers", config.servers.len());
        for server in config.servers.clone() {
            pool.execute( || {
                let mut rt = tokio::runtime::Runtime::new().unwrap();
                let block = async {
                    info!("Worker Thread");
                    threecx_collector(server).await;
                };
                rt.block_on(block);
            });
        }

        match rx.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
    }
}
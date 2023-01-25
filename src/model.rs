use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct ThreeCXSystemStatusResponse {
    pub FQDN: String,
    pub Version: String,
    pub IpV4: String,
    pub Activated: bool,
    pub MaxSimCalls: u8,
    pub CallHistoryCount: u16,
    pub ExtensionsRegistered: u16,
    pub ExtensionsTotal: u16,
    pub TrunksRegistered: u16,
    pub TrunksTotal: u16,
    pub CallsActive: u16,
    pub BlacklistedIpCount: u16,
    pub MemoryUsage: u32,
    pub PhysicalMemoryUsage: u32,
    pub DiskUsage: u32,
    pub CpuUsage: u32,
    pub Support: bool,
    pub LicenseActive: bool,
    pub OutboundRules: u32
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct ThreeCXServiceListResponse {
    pub Name: String,
    pub DisplayName: String,
    pub Status: u8,
    pub MemoryUsed: u32,
    pub CpuUsage: u32,
    pub ThreadCount: u32,
    pub HandleCount: u32,
    pub startStopEnabled: bool,
    pub restartEnabled: bool
}
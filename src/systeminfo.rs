use std::{ffi::OsStr, fs::{self, File}, io::Write, thread};

use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use systemstat::{
    BatteryLife, BlockDeviceStats, CPULoad, Duration, Filesystem, LoadAverage, Memory, Network,
    OffsetDateTime, Platform, SocketStats, Swap,
};

use crate::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SystemInfoContainer {
    data: Vec<SystemInfo>,
}

impl SystemInfoContainer {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, system_info: &SystemInfo) {
        self.data.push(system_info.clone());
    }

    pub fn load(path: &OsStr) -> Result<Self, Error> {
        let contents = fs::read_to_string(path)?;
        let container = serde_json::from_str::<SystemInfoContainer>(&contents)?;

        Ok(container)
    }

    pub fn save(&self, path: &OsStr) -> Result<(), Error> {
        let content = serde_json::to_string(&self)?;
        let mut file = File::create(path)?;

        file.write_all(content.as_bytes()).map_err(Error::IO)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SystemInfo {
    record_date_time: NaiveDate,
    battery_life: Option<BatteryLife>,
    block_device_stats: Option<systemstat::BTreeMap<String, BlockDeviceStats>>,
    boot_time: Option<OffsetDateTime>,
    cpu_temp: Option<f32>,
    load_avg: Option<LoadAverage>,
    memory: Option<Memory>,
    mounts: Option<Vec<Filesystem>>,
    networks: Option<systemstat::BTreeMap<String, Network>>,
    on_ac_power: Option<bool>,
    socket_stats: Option<SocketStats>,
    swap: Option<Swap>,
    uptime: Option<Duration>,
    cpu_load: Option<Vec<CPULoad>>,
    cpu_load_aggregate: Option<CPULoad>,
}

impl SystemInfo {
    pub fn collect(system: &systemstat::System) -> SystemInfo {
        let battery_life: Option<BatteryLife> = system.battery_life().ok();
        let block_device_stats: Option<systemstat::BTreeMap<String, BlockDeviceStats>> =
            system.block_device_statistics().ok();
        let boot_time: Option<OffsetDateTime> = system.boot_time().ok();
        let cpu_load = system.cpu_load().ok();
        let cpu_load_aggregate = system.cpu_load_aggregate().ok();
        let cpu_temp: Option<f32> = system.cpu_temp().ok();
        let load_avg: Option<LoadAverage> = system.load_average().ok();
        let memory: Option<Memory> = system.memory().ok();
        let mounts: Option<Vec<Filesystem>> = system.mounts().ok();
        let networks: Option<systemstat::BTreeMap<String, Network>> = system.networks().ok();
        let on_ac_power: Option<bool> = system.on_ac_power().ok();
        let socket_stats: Option<SocketStats> = system.socket_stats().ok();
        let swap: Option<Swap> = system.swap().ok();
        let uptime: Option<Duration> = system.uptime().ok();

        thread::sleep(std::time::Duration::from_secs(1));

        let cpu_load: Option<Vec<CPULoad>> = cpu_load.map(|x| x.done().ok()).flatten();
        let cpu_load_aggregate: Option<CPULoad> =
            cpu_load_aggregate.map(|x| x.done().ok()).flatten();

        SystemInfo {
            record_date_time: Utc::now().date_naive(),
            battery_life,
            block_device_stats,
            boot_time,
            cpu_load,
            cpu_load_aggregate,
            cpu_temp,
            load_avg,
            memory,
            mounts,
            networks,
            on_ac_power,
            socket_stats,
            swap,
            uptime,
        }
    }

    pub fn save(&self, path: &OsStr) -> Result<(), Error> {
        let content = serde_json::to_string(&self)?;
        let mut file = File::create(path)?;

        file.write_all(content.as_bytes()).map_err(Error::IO)
    }
}

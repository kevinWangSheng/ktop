use std::time::Duration;

use async_trait::async_trait;
use sysinfo::{Disks, System};

use super::{DataSnapshot, DataSource};
use crate::errors::Result;

#[derive(Debug, Clone, Default)]
pub struct SystemSnapshot {
    pub cpu_usages: Vec<f64>,
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_disk: u64,
    pub used_disk: u64,
}

pub struct SystemSource {
    sys: System,
    disks: Disks,
}

impl SystemSource {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        let disks = Disks::new_with_refreshed_list();
        Self { sys, disks }
    }
}

#[async_trait]
impl DataSource for SystemSource {
    async fn collect(&mut self) -> Result<DataSnapshot> {
        self.sys.refresh_cpu_usage();
        self.sys.refresh_memory();
        self.disks.refresh(true);

        let cpu_usages: Vec<f64> = self.sys.cpus().iter().map(|c| c.cpu_usage() as f64).collect();

        let total_memory = self.sys.total_memory();
        let used_memory = self.sys.used_memory();

        let total_disk: u64 = self.disks.iter().map(|d| d.total_space()).sum();
        let used_disk: u64 = self
            .disks
            .iter()
            .map(|d| d.total_space() - d.available_space())
            .sum();

        Ok(DataSnapshot::System(SystemSnapshot {
            cpu_usages,
            total_memory,
            used_memory,
            total_disk,
            used_disk,
        }))
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(1)
    }
}

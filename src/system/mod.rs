/*
 **********************************************************************
 * -------------------------------------------------------------------
 * Project Name : BlackWin htop
 * File Name    : system/mod.rs
 * Author       : Ebrahim Shafiei (EbraSha)
 * Email        : Prof.Shafiei@Gmail.com
 * Created On   : 2024-03-17 12:00:00
 * Description  : System information module for BlackWin htop
 * -------------------------------------------------------------------
 *
 * "Coding is an engaging and beloved hobby for me. I passionately and insatiably pursue knowledge in cybersecurity and programming."
 * – Ebrahim Shafiei
 *
 **********************************************************************
 */

use sysinfo::{System, CpuRefreshKind, MemoryRefreshKind, RefreshKind};

pub struct SystemInfo {
    system: System,
    physical_core_count: usize,
}

impl SystemInfo {
    pub fn new() -> Self {
        let mut system = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything())
        );
        system.refresh_cpu();
        
        // Get physical core count
        let physical_core_count = system.physical_core_count().unwrap_or(1);

        Self { 
            system,
            physical_core_count,
        }
    }

    pub fn update(&mut self) {
        self.system.refresh_cpu();
        self.system.refresh_memory();
    }

    pub fn cpu_cores_usage(&self) -> Vec<(String, f32)> {
        // Get all CPU cores
        let cpus = self.system.cpus();
        let mut cores = Vec::with_capacity(self.physical_core_count);

        // Add each physical core with its proper index
        for i in 0..self.physical_core_count {
            if let Some(cpu) = cpus.get(i) {
                cores.push((format!("CPU{}", i), cpu.cpu_usage()));
            }
        }

        cores
    }

    pub fn cpu_usage(&self) -> f32 {
        let total: f32 = self.system.cpus()
            .iter()
            .take(self.physical_core_count)
            .map(|cpu| cpu.cpu_usage())
            .sum();
        total / self.physical_core_count as f32
    }

    pub fn memory_usage(&self) -> (u64, u64) {
        (self.system.used_memory(), self.system.total_memory())
    }

    pub fn load_average(&self) -> (f64, f64, f64) {
        (0.0, 0.0, 0.0) // Windows doesn't support load average
    }
}

#[derive(Clone, Debug)]
pub struct ProcessInfo {
    pub pid: sysinfo::Pid,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub status: String,
}

impl ProcessInfo {
    pub fn kill(&self) -> bool {
        if let Some(process) = System::new_all().process(self.pid) {
            process.kill()
        } else {
            false
        }
    }
} 
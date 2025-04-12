/*
 **********************************************************************
 * -------------------------------------------------------------------
 * Project Name : BlackWin htop
 * File Name    : process/mod.rs
 * Author       : Ebrahim Shafiei (EbraSha)
 * Email        : Prof.Shafiei@Gmail.com
 * Created On   : 2024-03-17 12:00:00
 * Description  : Process management module for BlackWin htop
 * -------------------------------------------------------------------
 *
 * "Coding is an engaging and beloved hobby for me. I passionately and insatiably pursue knowledge in cybersecurity and programming."
 * – Ebrahim Shafiei
 *
 **********************************************************************
 */

use std::cmp::Ordering;
use sysinfo::{Pid, System, Process as SysProcess, ProcessRefreshKind, RefreshKind};

#[derive(Debug, Clone)]
pub struct Process {
    pub pid: Pid,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortField {
    Pid,
    Name,
    Cpu,
    Memory,
}

pub struct ProcessList {
    processes: Vec<Process>,
    selected_index: usize,
    sort_field: SortField,
    filter: Option<String>,
    system: System,
}

impl ProcessList {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            selected_index: 0,
            sort_field: SortField::Cpu,
            filter: None,
            system: System::new_with_specifics(
                RefreshKind::new()
                    .with_processes(ProcessRefreshKind::everything())
            ),
        }
    }

    pub fn update(&mut self) {
        self.system.refresh_processes();
        
        self.processes = self.system.processes()
            .iter()
            .map(|(pid, process)| Process {
                pid: *pid,
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory_usage: process.memory(),
            })
            .collect();

        self.sort_processes();
        self.apply_filter();
        self.clamp_selection();
    }

    fn sort_processes(&mut self) {
        self.processes.sort_by(|a, b| {
            match self.sort_field {
                SortField::Pid => a.pid.cmp(&b.pid),
                SortField::Name => a.name.cmp(&b.name),
                SortField::Cpu => b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(Ordering::Equal),
                SortField::Memory => b.memory_usage.cmp(&a.memory_usage),
            }
        });
    }

    fn apply_filter(&mut self) {
        if let Some(filter) = &self.filter {
            self.processes.retain(|process| {
                process.name.to_lowercase().contains(&filter.to_lowercase())
            });
        }
    }

    pub fn filter(&mut self, query: &str) {
        self.filter = Some(query.to_string());
        self.apply_filter();
        self.clamp_selection();
    }

    pub fn set_sort_field(&mut self, field: SortField) {
        self.sort_field = field;
        self.sort_processes();
    }

    pub fn move_selection(&mut self, delta: i32) {
        let new_index = self.selected_index as i32 + delta;
        self.selected_index = new_index.clamp(0, self.processes.len().saturating_sub(1) as i32) as usize;
    }

    pub fn move_to_start(&mut self) {
        self.selected_index = 0;
    }

    pub fn move_to_end(&mut self) {
        self.selected_index = self.processes.len().saturating_sub(1);
    }

    fn clamp_selection(&mut self) {
        if !self.processes.is_empty() {
            self.selected_index = self.selected_index.min(self.processes.len() - 1);
        } else {
            self.selected_index = 0;
        }
    }

    pub fn selected_pid(&self) -> Option<Pid> {
        self.processes.get(self.selected_index).map(|p| p.pid)
    }

    pub fn kill_process(&mut self, pid: Pid) {
        if let Some(process) = self.system.process(pid) {
            process.kill();
        }
    }

    pub fn processes(&self) -> &[Process] {
        &self.processes
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }
} 
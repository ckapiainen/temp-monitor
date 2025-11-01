use sysinfo::System;
use crate::collectors::cpu_frequency_collector::FrequencyMonitor;

#[derive(Clone, Debug)]
pub struct CoreData {
    pub name: String,
    pub usage: f32,
}
pub struct CpuData {
    cpu_name: String,
    cpu_count: u32,
    base_cpu_frequency: f64,
    cpu_usage: f32,
    cores: Vec<CoreData>,
    frequency_monitor: Option<FrequencyMonitor>,
    current_frequency: f64,
}

impl CpuData {
    pub fn new(sys: &System) -> Self {
        let base_freq = sys.cpus()[0].frequency() as f64 / 1000.0;
        let frequency_monitor = FrequencyMonitor::new(base_freq)
            .ok(); // If it fails just use base frequency

        let cores: Vec<CoreData> = sys.cpus()
            .iter()
            .map(|cpu| CoreData {
                name: cpu.name().to_string(),
                usage: cpu.cpu_usage(),
            })
            .collect();

        Self {
            cpu_name: sys.cpus()[0].brand().trim().replace("Processor","").to_string(),
            cpu_count: sys.cpus().len() as u32,
            base_cpu_frequency: base_freq,
            cpu_usage: sys.global_cpu_usage(),
            cores,
            frequency_monitor,
            current_frequency: base_freq,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.cpu_name
    }

    pub fn get_count(&self) -> &u32 {
        &self.cpu_count
    }

    pub fn get_base_frequency(&self) -> &f64 {
        &self.base_cpu_frequency
    }

    pub fn get_current_frequency(&self) -> f64 {
        self.current_frequency
    }

    pub fn get_cpu_usage(&self) -> &f32 {
        &self.cpu_usage
    }

    pub fn get_cores(&self) -> &[CoreData] {
        &self.cores
    }

    // Method to update dynamic data
    pub fn update(&mut self, sys: &mut System) {
        sys.refresh_cpu_all();
        self.cpu_usage = sys.global_cpu_usage();
        for (i, cpu) in sys.cpus().iter().enumerate() {
            if let Some(core_data) = self.cores.get_mut(i) {
                core_data.usage = cpu.cpu_usage();
            }
        }
        if let Some(ref monitor) = self.frequency_monitor {
            if let Ok(freq) = monitor.get_current_frequency() {
                self.current_frequency = freq;
            }
        }
    }
}
use sysinfo::System;

#[derive(Default, Clone)]
pub struct CpuData {
    cpu_name: String,
    cpu_count: usize,
    // cpu_frequency: u64,
    // cpu_usage: f32,
}

impl CpuData {
    pub fn new(sys: &System) -> Self {
        Self {
            cpu_name: sys.cpus()[0].brand().trim().to_string(),
            cpu_count: sys.cpus().len(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.cpu_name
    }

    pub fn get_count(&self) -> usize {
        self.cpu_count
    }

    // Method to update dynamic data (like CPU usage)
    pub fn update(&mut self, sys: &System) {
        // Update things that change over time
        // self.cpu_usage = sys.global_cpu_usage();
    }
}
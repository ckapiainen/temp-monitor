use sysinfo::System;

#[derive(Default, Clone)]
pub struct CpuData {
    cpu_name: String,
    cpu_count: u32,
    base_cpu_frequency: f64,
    cpu_usage: f32,
}

impl CpuData {
    pub fn new(sys: &System) -> Self {
        Self {
            cpu_name: sys.cpus()[0].brand().trim().replace("Processor","").to_string(),
            cpu_count: sys.cpus().len() as u32,
            base_cpu_frequency: sys.cpus()[0].frequency() as f64 / 1000.0,
            cpu_usage:  sys.global_cpu_usage(),

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
    pub fn get_cpu_usage(&self) -> &f32 {
        &self.cpu_usage
    }

    // Method to update dynamic data (like CPU usage)
    pub fn update(&mut self, sys: &mut System) {
        sys.refresh_cpu_all();
        self.cpu_usage = sys.global_cpu_usage();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    }
}
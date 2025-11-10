use super::CoreStats;
use crate::collectors::cpu_frequency_collector::FrequencyMonitor;
use sysinfo::System;

pub struct CpuData {
    initial_run: bool,
    pub name: String,
    pub core_count: u32,
    pub base_cpu_frequency: f64,
    pub temp: f32,
    pub temp_low: f32,
    pub temp_high: f32,
    pub temp_avg: f32,
    pub usage: f32,
    pub usage_low: f32,
    pub usage_high: f32,
    pub usage_avg: f32,
    pub core_utilization: Vec<CoreStats>,
    pub total_power_draw: f32,
    pub core_power_draw: Vec<CoreStats>,
    frequency_monitor: Option<FrequencyMonitor>,
    pub current_frequency: f64,
}

impl CpuData {
    pub fn new(sys: &System) -> Self {
        let base_freq = sys.cpus()[0].frequency() as f64 / 1000.0;
        let frequency_monitor = FrequencyMonitor::new(base_freq).ok(); // If it fails just use base frequency

        let cores: Vec<CoreStats> = sys
            .cpus()
            .iter()
            .map(|cpu| CoreStats {
                name: cpu.name().to_string(),
                value: cpu.cpu_usage(),
            })
            .collect();

        Self {
            initial_run: true,
            name: sys.cpus()[0]
                .brand()
                .trim()
                .replace("Processor", "")
                .to_string(),
            core_count: sys.cpus().len() as u32,
            base_cpu_frequency: base_freq,
            temp: 0.0,
            temp_low: 0.0,
            temp_high: 0.0,
            total_power_draw: 0.0,
            core_power_draw: Vec::new(),
            usage: sys.global_cpu_usage(),
            usage_low: sys.global_cpu_usage(),
            usage_high: sys.global_cpu_usage(),
            usage_avg: 0.0,
            core_utilization: cores,
            frequency_monitor,
            current_frequency: base_freq,
            temp_avg: 0.0,
        }
    }

    // lhm service updates
    pub fn update_lhm_data(&mut self, temps: (f32, f32, Vec<CoreStats>)) {
        if self.initial_run {
            self.initial_run = false;
            self.temp_low = temps.0;
        }
        self.temp = temps.0;
        self.total_power_draw = temps.1;
        self.core_power_draw = temps.2;
        if self.temp < self.temp_low {
            self.temp_low = self.temp;
        }
        if self.temp > self.temp_high {
            self.temp_high = self.temp;
        }
    }

    // Method to update sysinfo and win32 api data
    pub fn update(&mut self, sys: &mut System) {
        sys.refresh_cpu_all();
        let usage_update = sys.global_cpu_usage();
        self.usage = usage_update;
        if usage_update < self.usage_low {
            self.usage_low = usage_update;
        }
        if usage_update > self.usage_high {
            self.usage_high = usage_update;
        }

        for (i, cpu) in sys.cpus().iter().enumerate() {
            if let Some(core_data) = self.core_utilization.get_mut(i) {
                core_data.value = cpu.cpu_usage();
            }
        }
        if let Some(ref monitor) = self.frequency_monitor {
            if let Ok(freq) = monitor.get_current_frequency() {
                self.current_frequency = freq;
            }
        }
    }
}

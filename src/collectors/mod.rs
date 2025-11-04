pub mod cpu_collector;
pub mod cpu_frequency_collector;
pub mod lhm_collector;

/// Shared data structure for CPU core statistics (usage, power, etc.)
#[derive(Debug, Clone)]
pub struct CoreStats {
    pub name: String,
    pub value: f32,
}
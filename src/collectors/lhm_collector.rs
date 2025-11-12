use super::CoreStats;
use lhm_client::{HardwareType, SensorType};

pub async fn lhm_cpu_queries(client: &lhm_client::LHMClientHandle) -> (f32, f32, Vec<CoreStats>) {
    // Request all CPU hardware
    let mut temp = 0.0;
    let mut total_package_power = 0.0;
    let mut core_power: Vec<CoreStats> = Vec::new();

    let cpu_list = client
        .query_hardware(None, Some(HardwareType::Cpu))
        .await
        .unwrap();

    for cpu in cpu_list {
        // Request all CPU temperature sensors
        let total_temp_query = client
            .query_sensors(Some(cpu.identifier.clone()), Some(SensorType::Temperature))
            .await
            .unwrap();

        let power_query = client
            .query_sensors(Some(cpu.identifier.clone()), Some(SensorType::Power))
            .await
            .unwrap();

        // Find the CPU temperature sensor
        // "CPU Package" (Intel), "Core (Tctl/Tdie)" (AMD), "CPU Core" (generic)
        let temp_sensor = total_temp_query
            .iter()
            .find(|sensor| {
                sensor.name.eq("CPU Package")
                    || sensor.name.eq("Core (Tctl/Tdie)")
                    || sensor.name.eq("CPU Core")
                    || sensor.name.contains("Package")
                    || sensor.name.contains("Tctl")
            })
            .expect("Missing cpu temp sensor");

        let total = power_query
            .iter()
            .find(|sensor| sensor.name.contains("Package"))
            .unwrap();
        total_package_power = total.value;

        core_power = power_query
            .iter()
            .filter(|sensor| sensor.name.contains("Core"))
            .map(|sensor| CoreStats {
                name: sensor.name.clone(),
                value: sensor.value,
            })
            .collect();

        // Get the current sensor value
        temp = client
            .get_sensor_value_by_idx(temp_sensor.index, true)
            .await
            .unwrap()
            .expect("cpu temp sensor is now unavailable");
    }
    (temp, total_package_power, core_power)
}

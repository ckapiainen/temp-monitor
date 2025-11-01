use lhm_client::{HardwareType, SensorType};

pub async fn cpu_temps_query(client: &lhm_client::LHMClientHandle) ->f32{
    // Request all CPU hardware
    let mut temp = 0.0;
    println!("Requesting CPU hardware");
    let cpu_list = client
        .query_hardware(None, Some(HardwareType::Cpu))
        .await
        .unwrap();

    for cpu in cpu_list {
        // Request all CPU temperature sensors
        let cpu_temps = client
            .query_sensors(Some(cpu.identifier.clone()), Some(SensorType::Temperature))
            .await
            .unwrap();

        dbg!(&cpu_temps);

        // Find the CPU temperature sensor
        // "CPU Package" (Intel), "Core (Tctl/Tdie)" (AMD), "CPU Core" (generic)
        let temp_sensor = cpu_temps
            .iter()
            .find(|sensor| {
                sensor.name.eq("CPU Package")
                    || sensor.name.eq("Core (Tctl/Tdie)")
                    || sensor.name.eq("CPU Core")
                    || sensor.name.contains("Package")
                    || sensor.name.contains("Tctl")
            })
            .expect("Missing cpu temp sensor");

        // Get the current sensor value
        temp = client
            .get_sensor_value_by_idx(temp_sensor.index, true)
            .await
            .unwrap()
            .expect("cpu temp sensor is now unavailable");
    }
    temp
}

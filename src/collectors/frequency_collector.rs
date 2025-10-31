use windows::core::PCWSTR;
use windows::Win32::System::Performance::{
    PdhAddCounterW, PdhCloseQuery, PdhCollectQueryData, PdhGetFormattedCounterValue, PdhOpenQueryW,
    PDH_FMT_COUNTERVALUE, PDH_FMT_DOUBLE,
};

pub struct FrequencyMonitor {
    query: isize,
    counter: isize,
    base_frequency: f64, // GHz
}

impl FrequencyMonitor {
    pub fn new(base_frequency_ghz: f64) -> Result<Self, Box<dyn std::error::Error>> {
        unsafe {
            let mut query: isize = 0;
            let result = PdhOpenQueryW(PCWSTR::null(), 0, &mut query);
            if result != 0 {
                return Err(format!("PdhOpenQueryW failed with error code: {}", result).into());
            }

            let counter_path = "\\Processor Information(_Total)\\% Processor Performance\0"
                .encode_utf16()
                .collect::<Vec<u16>>();

            let mut counter: isize = 0;
            let result = PdhAddCounterW(query, PCWSTR(counter_path.as_ptr()), 0, &mut counter);
            if result != 0 {
                return Err(format!("PdhAddCounterW failed with error code: {}", result).into());
            }

            // Initial collect
            let _ = PdhCollectQueryData(query);

            Ok(Self {
                query,
                counter,
                base_frequency: base_frequency_ghz,
            })
        }
    }

    pub fn get_current_frequency(&self) -> Result<f64, Box<dyn std::error::Error>> {
        unsafe {
            // Collect data
            let result = PdhCollectQueryData(self.query);
            if result != 0 {
                return Err(
                    format!("PdhCollectQueryData failed with error code: {}", result).into(),
                );
            }

            // Get formatted value
            let mut value: PDH_FMT_COUNTERVALUE = std::mem::zeroed();
            let result =
                PdhGetFormattedCounterValue(self.counter, PDH_FMT_DOUBLE, None, &mut value);
            if result != 0 {
                return Err(format!(
                    "PdhGetFormattedCounterValue failed with error code: {}",
                    result
                )
                .into());
            }

            // Calculate: (% Performance / 100) * Base Frequency
            let percent_performance = value.Anonymous.doubleValue;
            let current_freq = (percent_performance / 100.0) * self.base_frequency;

            Ok(current_freq)
        }
    }
}

impl Drop for FrequencyMonitor {
    fn drop(&mut self) {
        unsafe {
            // Clean up PDH query handle when dropped
            let _ = PdhCloseQuery(self.query);
        }
    }
}

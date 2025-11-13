use anyhow::{Context, Result};
use iced::Theme;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fmt, fs};

// Saved to disk
#[derive(Serialize, Deserialize)]
struct Config {
    theme: String,
    start_with_windows: bool,
    start_minimized: bool,
    selected_temp_units: TempUnits,
    data_update_interval: f32,
    temp_low_threshold: f32,
    temp_high_threshold: f32,
}

// Runtime settings
#[derive(Clone)]
pub struct Settings {
    pub theme: Theme,
    pub start_with_windows: bool,
    pub start_minimized: bool,
    pub selected_temp_units: Option<TempUnits>,
    pub data_update_interval: f32,
    pub temp_low_threshold: f32,
    pub temp_high_threshold: f32,
    pub temp_low_input: String,
    pub temp_high_input: String,
    pub update_interval_input: String,
}
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TempUnits {
    Celsius,
    Fahrenheit,
}
impl fmt::Display for TempUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TempUnits::Celsius => write!(f, "Celsius"),
            TempUnits::Fahrenheit => write!(f, "Fahrenheit"),
        }
    }
}

#[derive(Clone)]
pub enum Message {
    ToggleStartWithWindows,
    ToggleStartMinimized,
    TempUnitSelected(TempUnits),
    TempLowThresholdChanged(String),
    TempHighThresholdChanged(String),
}

impl Settings {
    const CONFIG_PATH: &'static str = "config/cfg.toml";

    pub fn load() -> Result<Self> {
        let path = Path::new(Self::CONFIG_PATH);

        // Create config directory if needed
        if !path.exists() {
            let default = Self::default();
            default.save()?;
            return Ok(default);
        }
        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config from {}", Self::CONFIG_PATH))?;
        let config: Config = toml::from_str(&contents).with_context(|| "Failed to parse config")?;

        let theme = match config.theme.as_str() {
            "Dark" => Theme::Dark,
            "Dracula" => Theme::Dracula,
            "Nord" => Theme::Nord,
            "Ferra" => Theme::Ferra,
            _ => Theme::Dracula,
        };

        dbg!("Loaded config from disk");
        Ok(Self {
            theme,
            start_minimized: config.start_minimized,
            start_with_windows: config.start_with_windows,
            selected_temp_units: Option::from(config.selected_temp_units),
            data_update_interval: config.data_update_interval,
            temp_low_threshold: config.temp_low_threshold,
            temp_high_threshold: config.temp_high_threshold,
            temp_low_input: config.temp_low_threshold.to_string(),
            temp_high_input: config.temp_high_threshold.to_string(),
            update_interval_input: config.data_update_interval.to_string(),
        })
    }

    pub fn save(&self) -> Result<()> {
        let path = Path::new(Self::CONFIG_PATH);

        // Create directory if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).context("Failed to create config directory")?;
        }

        let theme_name = self.theme.to_string();
        let config = Config {
            theme: theme_name,
            start_minimized: self.start_minimized,
            start_with_windows: self.start_with_windows,
            selected_temp_units: self.selected_temp_units.expect("Temp unit must be selected"),
            data_update_interval: self.data_update_interval,
            temp_low_threshold: self.temp_low_threshold,
            temp_high_threshold: self.temp_high_threshold,
        };

        let toml = toml::to_string_pretty(&config).context("Failed to serialize config")?;
        fs::write(Self::CONFIG_PATH, toml)
            .with_context(|| format!("Failed to write config to {}", Self::CONFIG_PATH))?;
        dbg!("Saved config to disk");
        Ok(())
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::Dracula,
            start_with_windows: true,
            start_minimized: false,
            selected_temp_units: Option::from(TempUnits::Celsius),
            data_update_interval: 2.0,
            temp_low_threshold: 40.0,
            temp_high_threshold: 70.0,
            temp_low_input: "40".to_string(),
            temp_high_input: "70".to_string(),
            update_interval_input: "2.0".to_string(),
        }
    }
}

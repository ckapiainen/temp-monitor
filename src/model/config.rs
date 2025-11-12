use anyhow::{Context, Result};
use iced::Theme;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// Saved to disk
#[derive(Serialize, Deserialize)]
struct Config {
    theme: String,
    temp_low_threshold: f32,
    temp_high_threshold: f32,
}

// Runtime settings
#[derive(Clone)]
pub struct Settings {
    pub theme: Theme,
    pub temp_low_threshold: f32,
    pub temp_high_threshold: f32,
    pub temp_low_input: String,
    pub temp_high_input: String,
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
            temp_low_threshold: config.temp_low_threshold,
            temp_high_threshold: config.temp_high_threshold,
            temp_low_input: config.temp_low_threshold.to_string(),
            temp_high_input: config.temp_high_threshold.to_string(),
        })
    }

    pub fn save(&self) -> Result<()> {
        let path = Path::new(Self::CONFIG_PATH);

        // Create directory if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create config directory")?;
        }

        let theme_name = self.theme.to_string();
        let config = Config {
            theme: theme_name,
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
            temp_low_threshold: 40.0,
            temp_high_threshold: 70.0,
            temp_low_input: "40".to_string(),
            temp_high_input: "70".to_string(),
        }
    }
}

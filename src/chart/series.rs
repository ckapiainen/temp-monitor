use anyhow::Result;
use csv::ReaderBuilder;
use iced::Color;
use serde::Deserialize;
use std::path::Path;

/// A single data point in the chart
#[derive(Debug, Clone, Copy)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
}

impl DataPoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// A series of data points with styling
#[derive(Debug, Clone)]
pub struct ChartSeries {
    pub label: String,
    pub points: Vec<DataPoint>,
    pub color: Color,
    pub show_points: bool,
    pub show_line: bool,
    pub line_width: f32,
    pub point_radius: f32,
}

impl ChartSeries {
    /// Create a new chart series with default styling
    pub fn new(label: impl Into<String>, color: Color) -> Self {
        Self {
            label: label.into(),
            points: Vec::new(),
            color,
            show_points: true,
            show_line: true,
            line_width: 2.0,
            point_radius: 3.0,
        }
    }

    /// Add a single data point to the series
    pub fn add_point(&mut self, x: f64, y: f64) {
        self.points.push(DataPoint::new(x, y));
    }

    /// Add multiple data points
    pub fn add_points(&mut self, points: Vec<DataPoint>) {
        self.points.extend(points);
    }

    /// Set whether to show individual data points
    pub fn with_points(mut self, show: bool) -> Self {
        self.show_points = show;
        self
    }

    /// Set whether to show connecting lines
    pub fn with_line(mut self, show: bool) -> Self {
        self.show_line = show;
        self
    }

    /// Set line width
    pub fn with_line_width(mut self, width: f32) -> Self {
        self.line_width = width;
        self
    }

    /// Set point radius
    pub fn with_point_radius(mut self, radius: f32) -> Self {
        self.point_radius = radius;
        self
    }

    /// Clear all data points
    pub fn clear(&mut self) {
        self.points.clear();
    }

    /// Get the min and max values for this series
    pub fn get_bounds(&self) -> Option<(DataPoint, DataPoint)> {
        if self.points.is_empty() {
            return None;
        }

        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for point in &self.points {
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
        }

        Some((DataPoint::new(min_x, min_y), DataPoint::new(max_x, max_y)))
    }
}

/// Helper struct for deserializing CSV data
#[derive(Debug, Deserialize)]
struct CsvRecord {
    timestamp: String,
    temperature: Option<f32>,
    cpu_usage: Option<f32>,
    power_draw: Option<f32>,
}

/// Load chart series from a CSV file
/// This function assumes your CSV format with semicolon delimiter
pub fn load_series_from_csv(
    csv_path: impl AsRef<Path>,
    x_column: CsvColumn,
    y_column: CsvColumn,
    label: impl Into<String>,
    color: Color,
) -> Result<ChartSeries> {
    let mut reader = ReaderBuilder::new().delimiter(b';').from_path(csv_path)?;

    let mut series = ChartSeries::new(label, color);

    for (idx, result) in reader.deserialize().enumerate() {
        let record: CsvRecord = result?;

        // Parse timestamp as x value (using row index for now, but can be improved)
        let x_value = match x_column {
            CsvColumn::RowIndex => idx as f64,
            CsvColumn::Timestamp => {
                // Simple approach: use index, but you can parse timestamp to Unix time
                idx as f64
            }
            CsvColumn::Temperature => record.temperature.unwrap_or(0.0) as f64,
            CsvColumn::CpuUsage => record.cpu_usage.unwrap_or(0.0) as f64,
            CsvColumn::PowerDraw => record.power_draw.unwrap_or(0.0) as f64,
        };

        let y_value = match y_column {
            CsvColumn::RowIndex => idx as f64,
            CsvColumn::Timestamp => idx as f64,
            CsvColumn::Temperature => record.temperature.unwrap_or(0.0) as f64,
            CsvColumn::CpuUsage => record.cpu_usage.unwrap_or(0.0) as f64,
            CsvColumn::PowerDraw => record.power_draw.unwrap_or(0.0) as f64,
        };

        series.add_point(x_value, y_value);
    }

    Ok(series)
}

/// Column selector for CSV data
#[derive(Debug, Clone, Copy)]
pub enum CsvColumn {
    RowIndex,
    Timestamp,
    Temperature,
    CpuUsage,
    PowerDraw,
}

use crate::chart::series::{ChartSeries, DataPoint};
use iced::widget::canvas::{self, Cache, Canvas, Frame, Geometry, Path, Stroke, Text};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Theme};

/// This chart can be customized also during runtime
///
/// Configuration for the chart
#[derive(Debug, Clone)]
pub struct ChartConfig {
    pub x_label: String,
    pub y_label: String,
    pub x_unit: String,
    pub y_unit: String,
    pub show_grid: bool,
    pub show_legend: bool,
    pub grid_color: Color,
    pub axis_color: Color,
    pub text_color: Color,
    pub background_color: Color,
    pub margin_top: f32,
    pub margin_bottom: f32,
    pub margin_left: f32,
    pub margin_right: f32,
    pub x_min: Option<f64>,
    pub x_max: Option<f64>,
    pub y_min: Option<f64>,
    pub y_max: Option<f64>,
}

impl Default for ChartConfig {
    fn default() -> Self {
        Self {
            x_label: "X Axis".to_string(),
            y_label: "Y Axis".to_string(),
            x_unit: "".to_string(),
            y_unit: "".to_string(),
            show_grid: true,
            show_legend: true,
            grid_color: Color::from_rgba(0.5, 0.5, 0.5, 0.2),
            axis_color: Color::from_rgb(0.3, 0.3, 0.3),
            text_color: Color::from_rgb(0.7, 0.7, 0.7),
            background_color: Color::from_rgba(0.0, 0.0, 0.0, 0.0),
            margin_top: 5.0,
            margin_bottom: 85.0,
            margin_left: 65.0,
            margin_right: 30.0,
            x_min: None,
            x_max: None,
            y_min: None,
            y_max: None,
        }
    }
}

/// The main chart widget
pub struct Chart {
    pub series: Vec<ChartSeries>,
    config: ChartConfig,
    cache: Cache,
}

impl Chart {
    /// Create a new empty chart
    pub fn new() -> Self {
        Self {
            series: Vec::new(),
            config: ChartConfig::default(),
            cache: Cache::new(),
        }
    }

    /// Create a new chart with custom configuration
    pub fn with_config(config: ChartConfig) -> Self {
        Self {
            series: Vec::new(),
            config,
            cache: Cache::new(),
        }
    }

    /// Add a data series to the chart
    pub fn add_series(&mut self, series: ChartSeries) {
        self.series.push(series);
        self.cache.clear();
    }

    /// Remove all series
    pub fn clear_series(&mut self) {
        self.series.clear();
        self.cache.clear();
    }

    pub fn set_x_label(&mut self, label: impl Into<String>) {
        self.config.x_label = label.into();
        self.cache.clear();
    }

    pub fn set_y_label(&mut self, label: impl Into<String>) {
        self.config.y_label = label.into();
        self.cache.clear();
    }

    pub fn set_x_unit(&mut self, unit: impl Into<String>) {
        self.config.x_unit = unit.into();
        self.cache.clear();
    }

    pub fn set_y_unit(&mut self, unit: impl Into<String>) {
        self.config.y_unit = unit.into();
        self.cache.clear();
    }

    /// Set manual axis bounds
    pub fn set_bounds(&mut self, x_min: f64, x_max: f64, y_min: f64, y_max: f64) {
        self.config.x_min = Some(x_min);
        self.config.x_max = Some(x_max);
        self.config.y_min = Some(y_min);
        self.config.y_max = Some(y_max);
        self.cache.clear();
    }

    /// Clear the cache to force a redraw (call this after modifying series data)
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Calculate the data bounds from all series
    fn calculate_bounds(&self) -> (DataPoint, DataPoint) {
        let mut min_x = self.config.x_min.unwrap_or(f64::INFINITY);
        let mut max_x = self.config.x_max.unwrap_or(f64::NEG_INFINITY);
        let mut min_y = self.config.y_min.unwrap_or(f64::INFINITY);
        let mut max_y = self.config.y_max.unwrap_or(f64::NEG_INFINITY);

        for series in &self.series {
            if let Some((series_min, series_max)) = series.get_bounds() {
                if self.config.x_min.is_none() {
                    min_x = min_x.min(series_min.x);
                    max_x = max_x.max(series_max.x);
                }
                if self.config.y_min.is_none() {
                    min_y = min_y.min(series_min.y);
                    max_y = max_y.max(series_max.y);
                }
            }
        }

        // Only add padding if bounds are auto-calculated (not manually set)
        // This prevents axes from being pushed outside the chart area
        let x_padding = if self.config.x_min.is_none() && self.config.x_max.is_none() {
            (max_x - min_x) * 0.05
        } else {
            0.0
        };

        let y_padding = if self.config.y_min.is_none() && self.config.y_max.is_none() {
            (max_y - min_y) * 0.05
        } else {
            0.0
        };

        (
            DataPoint::new(min_x - x_padding, min_y - y_padding),
            DataPoint::new(max_x + x_padding, max_y + y_padding),
        )
    }

    /// Convert data coordinates to screen coordinates
    fn data_to_screen(
        &self,
        data_point: DataPoint,
        bounds: Rectangle,
        data_min: DataPoint,
        data_max: DataPoint,
    ) -> Point {
        let chart_width = bounds.width - self.config.margin_left - self.config.margin_right;
        let chart_height = bounds.height - self.config.margin_top - self.config.margin_bottom;

        let x_range = data_max.x - data_min.x;
        let y_range = data_max.y - data_min.y;

        let x = bounds.x
            + self.config.margin_left
            + ((data_point.x - data_min.x) / x_range * chart_width as f64) as f32;

        // Invert Y axis (screen coordinates go down, chart coordinates go up)
        let y = bounds.y + self.config.margin_top + chart_height
            - ((data_point.y - data_min.y) / y_range * chart_height as f64) as f32;

        Point::new(x, y)
    }

    /// Convert to iced Element
    pub fn view<'a, Message: 'a + Clone>(&'a self) -> Element<'a, Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl<Message> canvas::Program<Message> for Chart {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            // Clear background
            frame.fill_rectangle(Point::ORIGIN, bounds.size(), self.config.background_color);

            if self.series.is_empty() {
                // Draw "No Data" message
                frame.fill_text(Text {
                    content: "No data available".to_string(),
                    position: Point::new(bounds.width / 2.0, bounds.height / 2.0),
                    color: self.config.text_color,
                    size: 16.0.into(),
                    ..Default::default()
                });
                return;
            }

            let (data_min, data_max) = self.calculate_bounds();

            // Draw grid if enabled
            if self.config.show_grid {
                self.draw_grid(frame, bounds, data_min, data_max);
            }

            // Draw axes
            self.draw_axes(frame, bounds, data_min, data_max);

            // Draw each series
            for series in &self.series {
                self.draw_series(frame, bounds, series, data_min, data_max);
            }

            // Draw legend if enabled
            if self.config.show_legend && !self.series.is_empty() {
                self.draw_legend(frame, bounds);
            }
        });

        vec![geometry]
    }
}

impl Chart {
    /// Draw the grid lines
    fn draw_grid(
        &self,
        frame: &mut Frame,
        bounds: Rectangle,
        data_min: DataPoint,
        data_max: DataPoint,
    ) {
        let num_lines = 5;

        // Vertical grid lines
        for i in 0..=num_lines {
            let x_data = data_min.x + (data_max.x - data_min.x) * (i as f64 / num_lines as f64);
            let top = self.data_to_screen(
                DataPoint::new(x_data, data_max.y),
                bounds,
                data_min,
                data_max,
            );
            let bottom = self.data_to_screen(
                DataPoint::new(x_data, data_min.y),
                bounds,
                data_min,
                data_max,
            );

            let path = Path::line(top, bottom);
            frame.stroke(&path, Stroke::default().with_color(self.config.grid_color));
        }

        // Horizontal grid lines
        for i in 0..=num_lines {
            let y_data = data_min.y + (data_max.y - data_min.y) * (i as f64 / num_lines as f64);
            let left = self.data_to_screen(
                DataPoint::new(data_min.x, y_data),
                bounds,
                data_min,
                data_max,
            );
            let right = self.data_to_screen(
                DataPoint::new(data_max.x, y_data),
                bounds,
                data_min,
                data_max,
            );

            let path = Path::line(left, right);
            frame.stroke(&path, Stroke::default().with_color(self.config.grid_color));
        }
    }

    /// Draw the axes and labels
    fn draw_axes(
        &self,
        frame: &mut Frame,
        bounds: Rectangle,
        data_min: DataPoint,
        data_max: DataPoint,
    ) {
        // Draw axis lines
        let origin = self.data_to_screen(data_min, bounds, data_min, data_max);
        let x_end = self.data_to_screen(
            DataPoint::new(data_max.x, data_min.y),
            bounds,
            data_min,
            data_max,
        );
        let y_end = self.data_to_screen(
            DataPoint::new(data_min.x, data_max.y),
            bounds,
            data_min,
            data_max,
        );

        // X-axis
        let x_axis = Path::line(origin, x_end);
        frame.stroke(
            &x_axis,
            Stroke::default()
                .with_color(self.config.axis_color)
                .with_width(2.0),
        );

        // Y-axis
        let y_axis = Path::line(origin, y_end);
        frame.stroke(
            &y_axis,
            Stroke::default()
                .with_color(self.config.axis_color)
                .with_width(2.0),
        );

        // Draw axis labels and ticks
        self.draw_axis_labels(frame, bounds, data_min, data_max);
    }

    /// Draw axis labels and tick marks
    fn draw_axis_labels(
        &self,
        frame: &mut Frame,
        bounds: Rectangle,
        data_min: DataPoint,
        data_max: DataPoint,
    ) {
        let num_ticks = 5;

        // X-axis ticks and labels
        for i in 0..=num_ticks {
            let x_data = data_min.x + (data_max.x - data_min.x) * (i as f64 / num_ticks as f64);
            let tick_pos = self.data_to_screen(
                DataPoint::new(x_data, data_min.y),
                bounds,
                data_min,
                data_max,
            );

            // Tick mark
            let tick_end = Point::new(tick_pos.x, tick_pos.y + 5.0);
            frame.stroke(
                &Path::line(tick_pos, tick_end),
                Stroke::default().with_color(self.config.axis_color),
            );

            // Label - format as time if unit is seconds
            let label = if self.config.x_unit == " s" {
                // Format as time: convert to seconds
                let seconds = x_data as i32;
                format!("{}s", seconds)
            } else {
                format!("{:.1}{}", x_data, self.config.x_unit)
            };

            // Position labels below the X-axis line
            frame.fill_text(Text {
                content: label,
                position: Point::new(tick_pos.x, tick_pos.y + 20.0),
                color: self.config.text_color,
                size: 12.0.into(),
                ..Default::default()
            });
        }

        // Y-axis ticks and labels
        for i in 0..=num_ticks {
            let y_data = data_min.y + (data_max.y - data_min.y) * (i as f64 / num_ticks as f64);
            let tick_pos = self.data_to_screen(
                DataPoint::new(data_min.x, y_data),
                bounds,
                data_min,
                data_max,
            );

            // Tick mark
            let tick_end = Point::new(tick_pos.x - 5.0, tick_pos.y);
            frame.stroke(
                &Path::line(tick_pos, tick_end),
                Stroke::default().with_color(self.config.axis_color),
            );

            // Label - position to the left of the Y-axis
            let label = format!("{:.1}{}", y_data, self.config.y_unit);
            frame.fill_text(Text {
                content: label,
                position: Point::new(tick_pos.x - 50.0, tick_pos.y),
                color: self.config.text_color,
                size: 12.0.into(),
                ..Default::default()
            });
        }

        // X-axis title - centered horizontally, in bottom margin
        let x_title_y = bounds.height - self.config.margin_bottom / 4.0;
        frame.fill_text(Text {
            content: self.config.x_label.clone(),
            position: Point::new(bounds.width / 2.0, x_title_y),
            color: self.config.text_color,
            size: 14.0.into(),
            ..Default::default()
        });

        // Y-axis title - positioned above the chart area (not centered)
        let y_title_y = self.config.margin_top / 2.0;
        frame.fill_text(Text {
            content: self.config.y_label.clone(),
            position: Point::new(15.0, y_title_y),
            color: self.config.text_color,
            size: 14.0.into(),
            ..Default::default()
        });
    }

    /// Draw a single data series
    fn draw_series(
        &self,
        frame: &mut Frame,
        bounds: Rectangle,
        series: &ChartSeries,
        data_min: DataPoint,
        data_max: DataPoint,
    ) {
        if series.points.is_empty() {
            return;
        }

        // Convert all points to screen coordinates
        let screen_points: Vec<Point> = series
            .points
            .iter()
            .map(|&point| self.data_to_screen(point, bounds, data_min, data_max))
            .collect();

        // Draw lines connecting points
        // Skip connections where points have large time gaps (rolling buffer artifacts)
        if series.show_line && screen_points.len() > 1 {
            for i in 0..series.points.len() - 1 {
                let x_gap = (series.points[i + 1].x - series.points[i].x).abs();
                let expected_gap = if i > 0 {
                    (series.points[i].x - series.points[i - 1].x).abs()
                } else if series.points.len() > 2 {
                    (series.points[2].x - series.points[1].x).abs()
                } else {
                    1.0 // Default to 1.0 if can't determine
                };

                // Only draw line if gap is reasonable (within 3x expected interval)
                if x_gap < expected_gap * 3.0 {
                    let path = Path::line(screen_points[i], screen_points[i + 1]);
                    frame.stroke(
                        &path,
                        Stroke::default()
                            .with_color(series.color)
                            .with_width(series.line_width),
                    );
                }
            }
        }

        // Draw individual points
        if series.show_points {
            for point in screen_points {
                let circle = Path::circle(point, series.point_radius);
                frame.fill(&circle, series.color);
            }
        }
    }

    /// Draw the legend
    fn draw_legend(&self, frame: &mut Frame, bounds: Rectangle) {
        let legend_x = bounds.width - 150.0;
        let legend_y = 40.0;
        let line_height = 25.0;

        for (i, series) in self.series.iter().enumerate() {
            let y = legend_y + i as f32 * line_height;

            // Color indicator (small line)
            let line_start = Point::new(legend_x, y);
            let line_end = Point::new(legend_x + 30.0, y);
            frame.stroke(
                &Path::line(line_start, line_end),
                Stroke::default()
                    .with_color(series.color)
                    .with_width(series.line_width),
            );

            // Series label
            frame.fill_text(Text {
                content: series.label.clone(),
                position: Point::new(legend_x + 40.0, y),
                color: self.config.text_color,
                size: 12.0.into(),
                ..Default::default()
            });
        }
    }
}

impl Default for Chart {
    fn default() -> Self {
        Self::new()
    }
}

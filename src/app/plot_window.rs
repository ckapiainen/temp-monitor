use crate::chart::{Chart, ChartConfig, ChartSeries};
use crate::utils::csv_logger::CsvLogger;
use iced::{Color, Element};
use iced::widget::{column, container, Space};

pub struct PlotWindow {
    line_chart: Chart,
}

impl PlotWindow {
    pub fn new() -> Self {
        let config = ChartConfig {
            x_label: "Time".to_string(),
            y_label: "Temperature".to_string(),
            x_unit: " s".to_string(),
            y_unit: " °C".to_string(),
            show_grid: true,
            show_legend: true,
            grid_color: Color::from_rgba(0.5, 0.5, 0.5, 0.2),
            axis_color: Color::from_rgb(0.5, 0.5, 0.5),
            text_color: Color::from_rgb(0.9, 0.9, 0.9),
            background_color: Color::from_rgba(0.1, 0.1, 0.1, 0.5),
            margin_top: 5.0,
            margin_bottom: 85.0,
            margin_left: 65.0,
            margin_right: 120.0,
            x_min: None,
            x_max: None,
            y_min: Some(20.0),
            y_max: Some(90.0),
        };

        let mut cpu_series =
            ChartSeries::new("CPU", Color::from_rgb(1.0, 0.5, 0.0)).with_line_width(2.5);

        //  points for testing
        for i in 0..10 {
            let t = i as f64 * 0.5;
            let temp = 45.0 + (t * 0.3).sin() * 10.0;
            cpu_series.add_point(t, temp);
        }

        let mut chart = Chart::with_config(config);
        chart.add_series(cpu_series);
        Self { line_chart: chart }
    }
    pub fn view<'a, Message: 'a + Clone>(
        &'a self,
        _csv_logger: &CsvLogger,
    ) -> Element<'a, Message> {
        let content = column![self.line_chart.view(), Space::new()];

        container(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

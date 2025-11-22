use crate::utils::csv_logger::CsvLogger;
use iced::{Color, Element};
use iced_plot::{LineStyle, MarkerStyle, PlotWidget, PlotWidgetBuilder, Series};

pub struct PlotWindow {
    plot: PlotWidget,
}

#[derive(Debug, Clone)]
pub enum PlotWindowMessage {
    PlotUiMessage(iced_plot::PlotUiMessage),
    Tick,
}

impl PlotWindow {
    pub fn new() -> Self {
        // Initial dummy series to prevent empty buffer panics
        let dummy_series = Series::circles(vec![[0.0, 0.0]], 3.0).with_label("waiting for data");

        Self {
            plot: PlotWidgetBuilder::new()
                .with_autoscale_on_updates(true)
                .with_y_label("Temperature (°C)")
                .with_x_label("Time (s)")
                .with_tooltips(true)
                .with_x_lim(0.0, 60.0)
                .with_y_lim(0.0, 100.0)
                .add_series(dummy_series)
                .build()
                .unwrap(),
        }
    }

    pub fn update(&mut self, csv_logger: &CsvLogger, message: PlotWindowMessage) {
        match message {
            PlotWindowMessage::PlotUiMessage(msg) => {
                self.plot.update(msg);
            }
            PlotWindowMessage::Tick => {
                let mut cpu_temp_series: Vec<[f64; 2]> = csv_logger
                    .graph_data_buffer
                    .iter()
                    .enumerate()
                    .map(|(i, entry)| {
                        [
                            i as f64, // X axis
                            entry.temperature as f64,
                        ]
                    })
                    .collect();

                if !cpu_temp_series.is_empty() {
                    // If we have fewer than 33 points, duplicate the last point until we do.
                    // Workaround: Pad to 33 points to force wgpu buffer update.
                    // Necessary to display points between 0 and 33
                    if cpu_temp_series.len() < 33 {
                        let last_point = *cpu_temp_series.last().unwrap();
                        while cpu_temp_series.len() < 33 {
                            cpu_temp_series.push(last_point);
                        }
                    }

                    // Remove dummy/old series
                    self.plot.remove_series("waiting for data");
                    self.plot.remove_series("CPU Temperature");

                    let temp_series =
                        Series::new(cpu_temp_series, MarkerStyle::circle(2.0), LineStyle::Solid)
                            .with_label("CPU Temperature")
                            .with_color(Color::from_rgb(1.0, 0.5, 0.2));

                    self.plot.add_series(temp_series).unwrap();
                }
            }
        }
    }

    pub fn subscription(&self) -> iced::Subscription<PlotWindowMessage> {
        iced::window::frames().map(|_| PlotWindowMessage::Tick)
    }

    pub fn view(&self) -> Element<'_, PlotWindowMessage> {
        self.plot.view().map(PlotWindowMessage::PlotUiMessage)
    }
}

use crate::app::main_window::Message;
use crate::utils::csv_logger::CsvLogger;
use iced::Element;
use iced_plot::{PlotWidget, PlotWidgetBuilder};

struct PlotWindow {
    plot: PlotWidget,
}

impl PlotWindow {
    pub fn new() -> Self {
        Self::default()
    }
    // pub fn view(&self, csv_logger: CsvLogger) -> Element<'_, Message> {
    //     // self.plot.add_series(&csv_logger.)
    // }
}

impl Default for PlotWindow {
    fn default() -> Self {
        let plot = PlotWidgetBuilder::new()
            .with_x_label("Time (seconds)")
            .with_y_label("Temperature (°C)")
            .with_y_lim(0.0, 100.0)
            .with_autoscale_on_updates(false)
            .with_tooltips(true)
            .build()
            .unwrap();
        Self { plot }
    }
}

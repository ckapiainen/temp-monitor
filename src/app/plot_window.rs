use iced::Element;
use iced_plot::{PlotUiMessage, PlotWidget, PlotWidgetBuilder, Series};

pub struct PlotWindow {
    plot: PlotWidget,
}

impl PlotWindow {
    pub fn new() -> Self {
        // Create a minimal initial series to prevent empty buffer panics
        // Remove this series later when you add real data
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

    pub fn update(&mut self, message: PlotUiMessage) {
        self.plot.update(message);
    }

    pub fn view(&self) -> Element<'_, PlotUiMessage> {
        self.plot.view()
    }
}

pub mod plotter;
pub mod series;

pub use plotter::{Chart, ChartConfig};
pub use series::{load_series_from_csv, ChartSeries, CsvColumn, DataPoint};

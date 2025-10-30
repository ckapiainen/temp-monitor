use crate::app::styles;
use crate::collectors::cpu_collector::CpuData;
use crate::Message;
use iced::widget::{column, container, row, text, Grid, Row};
use iced::{Center, Element, Fill};

pub fn view(cpu_data: &CpuData) -> Element<Message> {
    let brand_and_base_freq = text(format!(
        "{}@{:.1}hz",
        cpu_data.get_name(),
        cpu_data.get_base_frequency()
    ))
    .size(17);
    let cores = cpu_data.get_cores();

    let cpu_count = text(format!(" {} logical cores", cpu_data.get_count())).size(17);
    let heading = row![brand_and_base_freq, cpu_count].padding(15).width(Fill);
    let total_load = column![ text("Total load").size(15), text(format!("{:.2}%", cpu_data.get_cpu_usage())).size(50)].align_x(Center);

    let core_row = Row::with_children(cores.iter().map(|core| {
        let core_name = text(&core.name);
        let core_usage = text(format!("{:.2}%", core.usage)).size(15);
        column![core_name, core_usage].align_x(Center).into()
    }))
    .spacing(10);

    let content = column![heading, total_load, core_row]
        .align_x(Center)
        .spacing(20);
    let card = container(content)
        .width(Fill)
        .height(250)
        .align_x(Center)
        .style(styles::card_container_style);

    container(card).padding(20).width(Fill).into()
}

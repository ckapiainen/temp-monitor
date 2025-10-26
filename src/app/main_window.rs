use crate::app::styles;
use crate::collectors::cpu_collector::CpuData;
use crate::Message;
use iced::widget::{container, column, row, text};
use iced::{Center, Element, Fill};

pub fn view(cpu_data: &CpuData) -> Element<Message> {
    let brand_and_base_freq = text(format!(
        "{}@{:.1}hz",
        cpu_data.get_name(),
        cpu_data.get_base_frequency()
    ))
    .size(25);
    let cpu_count = text(format!(" {} logical cores", cpu_data.get_count())).size(25);
    let heading = row![brand_and_base_freq, cpu_count].padding(10);
    let values = row![text(format!(
        "CPU Usage: {:.2}%",
        cpu_data.get_cpu_usage()
    ))
    .size(30)];
    let content = column![heading, values].align_x(Center).spacing(20);
    let card = container(content)
        .width(Fill)
        .height(250)
        .align_x(Center)
        .style(styles::card_container_style);

    container(card).padding(20).width(Fill).into()
}

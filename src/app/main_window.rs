use crate::app::styles;
use crate::collectors::cpu_collector::CpuData;
use crate::Message;
use iced::widget::{column, container, rich_text, row, rule, space, span, text, Grid, Row};
use iced::{font, never, Bottom, Center, Element, Fill, Font, Padding, Top};

pub fn view(cpu_data: &CpuData) -> Element<Message> {
    let cores = cpu_data.get_cores();
    let heading = rich_text([
        span("CPU:").font(Font {
            weight: font::Weight::Bold,
            ..Font::default()
        }),
        span("  "),
        span(cpu_data.get_name()).font(Font {
            weight: font::Weight::Bold,
            ..Font::default()
        }),
    ]).on_link_click(never).size(17);

    let heading = row![heading].padding(Padding {
        top: 10.0,
        right: 0.0,
        bottom: 0.0,
        left: 10.0,
    }).width(Fill);

    // Cpu stats
    let total_load = column![
        text("LOAD").size(20),
        text(format!("{:.2}%", cpu_data.get_cpu_usage())).size(55)
    ].align_x(Center);

    let temp = column![
        text("TEMP").size(20),
        text(format!("40 °C")).size(55) // todo: Placeholder value
    ].align_x(Center);

    let clock_speed = column![
        text("CLOCK SPEED").size(20),
        text(format!("3800 MHz")).size(55) // todo: Placeholder value
    ].align_x(Center);

    let stats_row = row![total_load, rule::vertical(1), temp, rule::vertical(1),clock_speed].spacing(25);

    // Build core row with vertical rules between cores
    let mut core_elements: Vec<Element<Message>> = Vec::new();
    for (i, core) in cores.iter().enumerate() {
        let core_name = text(&core.name);
        let core_usage = text(format!("{:.2}%", core.usage)).size(15);
        let core_col = column![core_usage, core_name].align_x(Center);

        core_elements.push(core_col.into());

        // Add vertical rule between cores but not after the last one
        if i < cores.len() - 1 {
            core_elements.push(rule::vertical(1).into());
        }
    }

    let core_row = Row::with_children(core_elements).spacing(10);

    let content = column![heading, rule::horizontal(1), stats_row]
        .align_x(Center)
        .spacing(15);
    let card = container(content)
        .width(Fill)
        .height(250)
        .align_x(Center)
        .style(styles::card_container_style);

    // Core usage card
    let cores_card_content = column![
        text("CORE USAGE").size(15).font(Font {
            weight: font::Weight::Bold,
            ..Font::default()
        }),
        rule::horizontal(1),
        core_row
    ]
    .align_x(Center)
    .spacing(10)
    .padding(10);

    let cores_card = container(cores_card_content)
        .width(Fill)
        .height(200)
        .align_x(Center)
        .align_y(Bottom)
        .style(styles::card_container_style);

    let all_cards = column![card, cores_card].spacing(20);

    container(all_cards).padding(20).width(Fill).into()
}

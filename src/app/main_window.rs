use crate::app::styles;
use crate::collectors::cpu_collector::CpuData;
use iced::widget::{button, center, column, container, progress_bar, rich_text, row, rule, span, text, Row};
use iced::{font, never, Center, Element, Fill, Font, Padding};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BarChartState {
    Usage,
    Power,
}

#[derive(Debug, Clone)]
pub enum Message {
    UsageButtonPressed,
    PowerButtonPressed,
}

pub struct MainWindow {
    bar_chart_state: BarChartState,
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            bar_chart_state: BarChartState::Usage,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::UsageButtonPressed => {
                self.bar_chart_state = BarChartState::Usage;
            }
            Message::PowerButtonPressed => {
                self.bar_chart_state = BarChartState::Power;
            }
        }
    }

    pub fn view<'a>(&self, cpu_data: &'a CpuData) -> Element<'a, Message> {
    let core_usage_vector = &cpu_data.core_utilization;
    let core_power_draw_vector = &cpu_data.core_power_draw;
    let heading = rich_text([
        span("CPU:").font(Font {
            weight: font::Weight::Bold,
            ..Font::default()
        }),
        span("  "),
        span(&cpu_data.cpu_name).font(Font {
            weight: font::Weight::Bold,
            ..Font::default()
        }),
    ])
    .on_link_click(never)
    .size(17);

    let heading = row![heading]
        .padding(Padding {
            top: 10.0,
            right: 0.0,
            bottom: 0.0,
            left: 10.0,
        })
        .width(Fill);

    /*
    General CPU info card
    */
    let total_load = column![
        text("LOAD").size(20),
        text(format!("{:.2}%", cpu_data.cpu_usage)).size(55)
    ]
    .align_x(Center)
    .width(150);

    let temp = rich_text![
        span("TEMP\n").size(20),
        span(format!("{:.1}", cpu_data.cpu_temp)).size(55),
        span(" \u{00B0}").size(38).font(Font {
            weight: font::Weight::Light,
            ..Font::default()
        }),
        span("C")
            .font(Font {
                weight: font::Weight::Light,
                ..Font::default()
            })
            .size(35),
    ]
    .align_x(Center)
    .on_link_click(never)
    .width(170);

    let clock_speed = column![
        text("CLOCK SPEED").size(20),
        text(format!("{:.0} MHz", cpu_data.current_frequency * 1000.0)).size(55)
    ]
    .align_x(Center);

    let stats_row = row![
        total_load,
        rule::vertical(1),
        temp,
        rule::vertical(1),
        clock_speed
    ]
    .spacing(25)
    .padding(Padding {
        top: 0.0,
        right: 0.0,
        bottom: 10.0,
        left: 0.0,
    });
    let content = column![heading, rule::horizontal(1), stats_row]
        .align_x(Center)
        .spacing(15);
    let general_cpu_info_card = center(content)
        .height(210)
        .style(styles::card_container_style)
        .clip(true);

    /*
      CORE USAGE COLUMNS
    */

    // Build core row with vertical rules between cores
    let mut usage_bar_chart: Vec<Element<Message>> = Vec::new();
    for (i, core) in core_usage_vector.iter().enumerate() {
        let utilization = progress_bar(0.0..=100.0, core.value)
            .vertical()
            .length(150)
            .girth(35);

        let name_util_val = rich_text![
            span(format!("{:.2}%\n", core.value))
                .font(Font {
                    weight: font::Weight::Thin,
                    ..Font::default()
                })
                .size(15),
            span(format!("{}", core.name))
                .font(Font {
                    weight: font::Weight::Thin,
                    ..Font::default()
                })
                .size(15),
        ]
        .on_link_click(never)
        .align_x(Center)
        .width(70);
        let core_col = column![utilization, name_util_val].align_x(Center);
        usage_bar_chart.push(core_col.into());

        // Add vertical rule between cores but not after the last one
        if i < core_usage_vector.len() - 1 {
            usage_bar_chart.push(rule::vertical(1).into());
        }
    }

    /*
      CORE POWER DRAW COLUMNS
    */
    let mut power_bar_chart: Vec<Element<Message>> = Vec::new();
    for (i, core) in core_power_draw_vector.iter().enumerate() {
        let wattage_bar = progress_bar(0.0..=20.0, core.value)
            .vertical()
            .length(150)
            .girth(35);

        let name_util_val = rich_text![
            span(format!("{:.2}W\n", core.value))
                .font(Font {
                    weight: font::Weight::Thin,
                    ..Font::default()
                })
                .size(15),
            span(format!("{}", core.name.replace("#", "")))
                .font(Font {
                    weight: font::Weight::Thin,
                    ..Font::default()
                })
                .size(15),
        ]
            .on_link_click(never)
            .align_x(Center)
            .width(70);
        let core_col = column![wattage_bar, name_util_val].align_x(Center);
        power_bar_chart.push(core_col.into());

        // Add vertical rule between cores but not after the last one
        if i < core_usage_vector.len() - 1 {
            power_bar_chart.push(rule::vertical(1).into());
        }
    }
    let core_usage_row = Row::with_children(usage_bar_chart).spacing(1);
    let core_power_row = Row::with_children(power_bar_chart).spacing(1);


    // Core card
    let usage_button = button(
        container(text("U").size(11))
            .align_x(Center)
            .align_y(Center)
            .width(Fill)
            .height(Fill)
    )
    .on_press(Message::UsageButtonPressed)
    .style(styles::compact_icon_button_style)
    .width(50)
    .height(23);

    let power_button = button(
        container(text("P").size(11))
            .align_x(Center)
            .align_y(Center)
            .width(Fill)
            .height(Fill)
    )
    .on_press(Message::PowerButtonPressed)
    .style(styles::compact_icon_button_style)
    .width(50)
    .height(23);

    let button_group = row![usage_button, power_button].spacing(5);

    let header_text = container(text("CORE USAGE").size(15).font(Font {
        weight: font::Weight::Bold,
        ..Font::default()
    }))
    .width(Fill)
    .align_x(Center);

    let header_row = row![header_text, button_group]
        .align_y(Center)
        .spacing(10)
        .width(Fill);

    let cores_card_content = column![
        header_row,
        rule::horizontal(1),
        match self.bar_chart_state {
            BarChartState::Usage => core_usage_row,
            BarChartState::Power => core_power_row,
        }
    ]
    .align_x(Center)
    .spacing(10)
    .padding(10);

    let cores_card = container(cores_card_content)
        .width(Fill)
        .height(280)
        .align_x(Center)
        .align_y(Center)
        .style(styles::card_container_style);

    let all_cards = column![general_cpu_info_card, cores_card].spacing(20);
    container(all_cards).padding(20).width(Fill).into()
    }
}

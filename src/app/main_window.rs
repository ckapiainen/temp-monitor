use crate::app::styles;
use crate::collectors::cpu_collector::CpuData;
use crate::Message;
use iced::widget::{button, container, pick_list, row, text};
use iced::{window, Element, Fill, Theme};
use std::sync::mpsc::Receiver;
use std::time::Duration;
use sysinfo::System;

pub fn view<'a>() -> Element<'a, Message> {
    let content = row!["Test", "This is the main container"].spacing(10).padding(20);

    let card = container(content)
        .width(Fill)
        .height(250)
        .style(styles::card_container_style);

    container(card)
        .padding(20)
        .width(Fill)
        .into()
}

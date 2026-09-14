use iced::widget::{column, container, row, text, text_input};
use iced::{Color, Element, Fill, Length};

use crate::message::Message;
use crate::styles::{bar_fill_style, bar_track_style, card_style, chip_style, panel_style};

pub(crate) fn card<'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(content)
        .padding(18)
        .width(Fill)
        .style(card_style)
        .into()
}

pub(crate) fn panel<'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(content)
        .padding(14)
        .width(Fill)
        .style(panel_style)
        .into()
}

pub(crate) fn section_title(label: &str) -> Element<'static, Message> {
    text(label.to_string()).size(20).into()
}

pub(crate) fn status_chip(label: String) -> Element<'static, Message> {
    container(text(label).size(12))
        .padding([6, 10])
        .style(chip_style)
        .into()
}

pub(crate) fn checklist_line(label: &str, valid: bool) -> String {
    let marker = if valid { "✓" } else { "!" };
    format!("{marker} {label}")
}

pub(crate) fn labeled_input<'a>(
    label: impl Into<String>,
    value: impl Into<String>,
    on_input: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    let label = label.into();
    let value = value.into();

    column![
        text(label.clone()).size(12),
        text_input(&label, &value).on_input(on_input).padding(12),
    ]
    .spacing(5)
    .width(Fill)
    .into()
}

const STAT_BAR_TRACK_WIDTH: f32 = 220.0;
const STAT_BAR_HEIGHT: f32 = 10.0;

pub(crate) fn stat_bar<'a>(
    label: String,
    value: usize,
    max_value: usize,
    accent: Color,
) -> Element<'a, Message> {
    let max_value = max_value.max(1);
    let ratio = (value as f32 / max_value as f32).clamp(0.0, 1.0);
    let filled_width = if value > 0 {
        (STAT_BAR_TRACK_WIDTH * ratio).max(4.0)
    } else {
        0.0
    };

    let fill = container(text(""))
        .width(Length::Fixed(filled_width))
        .height(Length::Fixed(STAT_BAR_HEIGHT))
        .style(bar_fill_style(accent));

    let track = container(fill)
        .width(Length::Fixed(STAT_BAR_TRACK_WIDTH))
        .height(Length::Fixed(STAT_BAR_HEIGHT))
        .style(bar_track_style);

    column![
        row![text(label).size(13), text(value.to_string()).size(13)].spacing(8),
        track,
    ]
    .spacing(4)
    .into()
}

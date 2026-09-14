use iced::{Background, Border, Color, Shadow, Theme, Vector};

pub(crate) fn shell_style(_theme: &Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: Some(color(226, 232, 240)),
        background: Some(Background::Color(color(15, 23, 42))),
        ..Default::default()
    }
}

pub(crate) fn card_style(_theme: &Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: Some(color(226, 232, 240)),
        background: Some(Background::Color(color_alpha(30, 41, 59, 0.74))),
        border: Border::default()
            .rounded(24)
            .width(1)
            .color(color_alpha(148, 163, 184, 0.22)),
        shadow: Shadow {
            color: color_alpha(2, 6, 23, 0.24),
            offset: Vector::new(0.0, 18.0),
            blur_radius: 42.0,
        },
        ..Default::default()
    }
}

pub(crate) fn panel_style(_theme: &Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: Some(color(226, 232, 240)),
        background: Some(Background::Color(color_alpha(15, 23, 42, 0.64))),
        border: Border::default()
            .rounded(18)
            .width(1)
            .color(color_alpha(148, 163, 184, 0.16)),
        ..Default::default()
    }
}

pub(crate) fn accent_panel_style(_theme: &Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: Some(color(254, 243, 199)),
        background: Some(Background::Color(color_alpha(120, 53, 15, 0.24))),
        border: Border::default()
            .rounded(18)
            .width(1)
            .color(color_alpha(251, 191, 36, 0.32)),
        ..Default::default()
    }
}

pub(crate) fn chip_style(_theme: &Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: Some(color(203, 213, 225)),
        background: Some(Background::Color(color_alpha(2, 6, 23, 0.42))),
        border: Border::default()
            .rounded(999)
            .width(1)
            .color(color_alpha(148, 163, 184, 0.2)),
        ..Default::default()
    }
}

fn color(red: u8, green: u8, blue: u8) -> Color {
    Color::from_rgb8(red, green, blue)
}

fn color_alpha(red: u8, green: u8, blue: u8, alpha: f32) -> Color {
    Color {
        a: alpha,
        ..Color::from_rgb8(red, green, blue)
    }
}

pub(crate) fn success_color() -> Color {
    color(74, 222, 128)
}

pub(crate) fn warning_color() -> Color {
    color(251, 191, 36)
}

pub(crate) fn danger_color() -> Color {
    color(248, 113, 113)
}

pub(crate) fn info_color() -> Color {
    color(96, 165, 250)
}

pub(crate) fn accent_color() -> Color {
    color(192, 132, 252)
}

pub(crate) fn bar_track_style(_theme: &Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(Background::Color(color_alpha(148, 163, 184, 0.16))),
        border: Border::default().rounded(6),
        ..Default::default()
    }
}

pub(crate) fn bar_fill_style(accent: Color) -> impl Fn(&Theme) -> iced::widget::container::Style {
    move |_theme: &Theme| iced::widget::container::Style {
        background: Some(Background::Color(accent)),
        border: Border::default().rounded(6),
        ..Default::default()
    }
}

use iced::border;
use iced::widget::button::{Status, Style};
use iced::{Background, Color, Shadow, Theme};

pub fn custom_primary(_theme: &Theme, status: Status) -> Style {
    match status {
        Status::Active => Style {
            background: Some(Background::Color(Color::from_rgb(0.2, 0.5, 0.8))),
            text_color: Color::WHITE,
            border: border::rounded(8),
            shadow: Shadow::default(),
        },
        Status::Hovered => Style {
            background: Some(Background::Color(Color::from_rgb(0.3, 0.6, 0.9))),
            text_color: Color::WHITE,
            border: border::rounded(8),
            shadow: Shadow::default(),
        },
        Status::Pressed => Style {
            background: Some(Background::Color(Color::from_rgb(0.1, 0.4, 0.7))),
            text_color: Color::WHITE,
            border: border::rounded(8),
            shadow: Shadow::default(),
        },
        Status::Disabled => Style {
            background: Some(Background::Color(Color::from_rgb(0.5, 0.5, 0.5))),
            text_color: Color::from_rgb(0.7, 0.7, 0.7),
            border: border::rounded(8),
            shadow: Shadow::default(),
        },
    }
}

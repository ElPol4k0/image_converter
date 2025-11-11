use iced::widget::{container, Container};
use iced::{Border, Color, Element, Length, Shadow, Theme};

/// Erstellt eine Card (Box mit Hintergrund und Border Radius)
pub fn card(
    content: impl Into<Element<'static, crate::app::message::Message, Theme>>,
) -> Container<'static, crate::app::message::Message, Theme> {
    container(content)
        .padding(20)
        .width(Length::Fill)
        .style(card_style)
}

pub fn card_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(Color::from_rgb(0.15, 0.15, 0.15))),
        border: Border {
            color: Color::from_rgb(0.25, 0.25, 0.25),
            width: 1.0,
            radius: 12.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        text_color: None,
    }
}

pub fn card_success(
    content: impl Into<Element<'static, crate::app::message::Message, Theme>>,
) -> Container<'static, crate::app::message::Message, Theme> {
    container(content)
        .padding(20)
        .width(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.0, 0.2, 0.1))), // Grünlicher BG
            border: Border {
                color: Color::from_rgb(0.0, 0.8, 0.4), // Grün
                width: 2.0,
                radius: 12.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.8, 0.4, 0.3),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 12.0,
            },
            text_color: None,
        })
}

pub fn card_error(
    content: impl Into<Element<'static, crate::app::message::Message, Theme>>,
) -> Container<'static, crate::app::message::Message, Theme> {
    container(content)
        .padding(20)
        .width(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.2, 0.0, 0.0))), // Rötlicher BG
            border: Border {
                color: Color::from_rgb(0.8, 0.0, 0.0), // Rot
                width: 2.0,
                radius: 12.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.8, 0.0, 0.0, 0.3),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 12.0,
            },
            text_color: None,
        })
}

fn card_primary(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(Color::from_rgb(0.15, 0.15, 0.15))),
        border: Border {
            color: Color::from_rgb(0.74, 0.16, 0.92), // ← Lila!
            width: 2.0,                               // ← Dicker
            radius: 12.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.74, 0.16, 0.92, 0.3), // ← Lila Glow
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 12.0,
        },
        text_color: None,
    }
}

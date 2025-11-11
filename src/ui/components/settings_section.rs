use crate::app::message::Message;
use crate::ui::components::card;
use iced::font::Weight;
use iced::widget::{column, pick_list, row, slider, text};
use iced::{color, Alignment, Element, Font, Length};

pub fn view(output_format: &str, quality: u8) -> Element<'_, Message> {
    card::card(
        column![
            text("Settings")
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .size(20)
                .font(Font {
                    weight: Weight::ExtraBold,
                    ..Font::default()
                })
                .color(color!(0xffffff)),
            row![
                text("Format:"),
                pick_list(
                    vec![
                        "PNG".to_string(),
                        "JPEG".to_string(),
                        "WEBP".to_string(),
                        "GIF".to_string()
                    ],
                    Some(output_format.to_string()),
                    |format| Message::FormatChanged(format)
                )
            ]
            .spacing(10),
            row![
                text("Quality:"),
                slider(0..=100, quality, Message::QualityChanged),
                text(format!("{}%", quality)),
            ]
            .spacing(10),
        ]
        .spacing(15),
    )
    .width(Length::Fill)
    .padding(20)
    .into()
}

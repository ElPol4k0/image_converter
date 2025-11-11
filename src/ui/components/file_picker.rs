use crate::app::message::Message;
use crate::ui::components::card;
use crate::ui::styles::button_style;
use iced::font::Weight;
use iced::widget::{button, column, text};
use iced::Alignment;
use iced::{color, Font};
use iced::{Element, Length};
use std::path::PathBuf;

pub fn view(selected: &Option<PathBuf>) -> Element<'static, Message> {
    let (file_text, text_color) = match selected {
        Some(path) => (path.to_string_lossy().to_string(), color!(0x00FF00)),
        None => ("No file selected".to_string(), color!(0xFF0000)),
    };

    card::card(
        column![
            text("Select Image")
                .size(18)
                .font(Font {
                    weight: Weight::ExtraBold,
                    ..Font::default()
                })
                .color(color!(0xffffff)),
            text(file_text).color(text_color).font(Font {
                weight: Weight::Semibold,
                ..Font::default()
            }),
            button("Browse...")
                .on_press(Message::SelectFilePressed)
                .style(button_style::custom_primary)
                .padding(12),
        ]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .spacing(15),
    )
    .into()
}

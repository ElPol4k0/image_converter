use crate::app::message::Message;
use iced::color;
use iced::font::Weight;
use iced::widget::{column, container, text};
use iced::{Alignment, Element, Font, Length};

pub fn view() -> Element<'static, Message> {
    container(
        column![
            text("Image Converter")
                .size(32)
                .color(color!(0xffffff))
                .font(Font {
                    weight: Weight::ExtraBold,
                    ..Font::default()
                }),
            text("Convert your images easily")
                .size(14)
                .color(color!(0xffffff))
                .font(Font {
                    weight: Weight::Bold,
                    ..Font::default()
                }),
        ]
        .spacing(5)
        .align_x(Alignment::Center)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding(20)
    .into()
}

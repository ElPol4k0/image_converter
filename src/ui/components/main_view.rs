use crate::app::{message::Message, state::MyApp};
use crate::ui::components::{file_picker, footer, header, settings_section};
use iced::widget::{column, container};
use iced::{Element, Length};

pub fn view(app: &MyApp) -> Element<'_, Message> {
    let content = column![
        header::view(),
        file_picker::view(&app.selected_file),
        settings_section::view(&app.output_format, app.quality),
        footer::view(&app.conversion_status),
    ]
    .spacing(20);

    container(content)
        .padding(30)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

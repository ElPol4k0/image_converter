use crate::app::message::Message;
use crate::ui::config::UiConfig;
use iced::daemon::Appearance;
use iced::window::{Position, Settings as WindowSettings};
use iced::{Color, Element, Size, Task};
use std::path::PathBuf;

pub struct MyApp {
    // Dein App-State
    pub selected_file: Option<PathBuf>,
    pub output_format: String,
    pub quality: u8,
    pub conversion_status: String,
}

impl MyApp {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                selected_file: None,
                output_format: String::from("PNG"),
                quality: 90,
                conversion_status: String::new(),
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectFilePressed => {
                self.conversion_status = String::from("Opening File dialog");

                Task::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .add_filter("Images", &["png", "jpg", "jpeg", "ico"])
                            .pick_file()
                            .await
                            .map(|handle| handle.path().to_path_buf())
                    },
                    Message::FileSelected,
                )
            }
            Message::FileSelected(path) => {
                self.selected_file = path;
                Task::none()
            }
            Message::FormatChanged(format) => {
                self.output_format = format;
                Task::none()
            }
            Message::QualityChanged(quality) => {
                self.quality = quality;
                Task::none()
            }
            Message::ConvertPressed => {
                // Start >Convert File
                Task::none()
            }
            Message::ConversionComplete(_result) => {
                // Handle result
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        crate::ui::components::main_view::view(self)
    }

    pub fn window_settings() -> WindowSettings {
        let config = UiConfig::default();
        WindowSettings {
            size: Size::new(config.window_width, config.window_height),
            position: Position::Centered,
            ..Default::default()
        }
    }

    pub fn style(_state: &Self, _theme: &iced::Theme) -> Appearance {
        let config = UiConfig::default();
        Appearance {
            background_color: config.background_color,
            text_color: Color::from_rgb(0.051, 0.067, 0.090),
        }
    }
}

use iced::Color;

pub struct UiConfig {
    pub font: &'static str,
    pub font_size: u16,
    pub font_weight: u16,

    pub window_width: f32,
    pub window_height: f32,
    pub background_color: Color,
}

impl Default for UiConfig {
    fn default() -> Self {
        UiConfig {
            font: "Arial",
            font_size: 16,
            font_weight: 400,
            window_width: 800.0,
            window_height: 600.0,
            background_color: Color::from_rgb(0.10, 0.10, 0.10),
        }
    }
}

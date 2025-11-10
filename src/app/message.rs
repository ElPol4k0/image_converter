#[derive(Debug, Clone)]
pub enum Message {
    // File Picker Events
    SelectFilePressed,
    FileSelected(Option<std::path::PathBuf>),

    // Conversion Events
    FormatChanged(String),
    QualityChanged(u8),
    ConvertPressed,
    ConversionComplete(Result<String, String>),
}

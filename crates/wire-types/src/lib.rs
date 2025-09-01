/// Supported pixel type.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum PixelType {
    Rgb8,
    Rgba8,
}

/// Wire type for an RGB8 image.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u8>,
    pub pixel: PixelType,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Error {
    pub description: String,
}

impl From<String> for Error {
    fn from(description: String) -> Self {
        Error { description }
    }
}

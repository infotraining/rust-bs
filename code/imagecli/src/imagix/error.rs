use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImagixError {
    #[error("User input error: {0}")]
    UserInputError(String),
    #[error("File I/O error: {0}")]
    FileIOError(#[from] std::io::Error),
    #[error("Image resizing error: {0}")]
    ImageResizingError(#[from] image::ImageError)
}

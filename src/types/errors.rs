use thiserror::Error;

#[derive(Error, Debug)]
pub enum BatchError {
    #[error("Batch is already drawing")]
    BatchIsDrawing,
    #[error("Batch is not drawing")]
    BatchNotDrawing,
    #[error("Frame is None")]
    FrameIsNone,
}

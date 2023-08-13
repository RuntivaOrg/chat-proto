#[derive(Debug, thiserror::Error)]
pub enum ChatProtoError {
    #[error("Proto Date OutOfRange: {0}/{1}")]
    DateOutOfRange(i64, i32),
}

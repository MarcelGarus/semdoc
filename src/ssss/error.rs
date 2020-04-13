use crate::utils::SourceRange;

#[derive(Debug)]
pub struct SsssError {
    pub id: String,
    pub message: String,
    pub position: SourceRange,
}

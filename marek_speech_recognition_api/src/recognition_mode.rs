#[non_exhaustive]
pub enum RecognitionMode {
    Speech,

    Commands(Vec<String>),
}

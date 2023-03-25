#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum RecognitionMode {
    Speech,

    Commands(Vec<String>),
}

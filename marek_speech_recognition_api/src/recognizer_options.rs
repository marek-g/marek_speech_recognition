use crate::recognition_mode::RecognitionMode;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct RecognizerOptions {
    pub language: String,
    pub sample_rate: i32,
    pub mode: RecognitionMode,
}

impl Default for RecognizerOptions {
    fn default() -> Self {
        Self {
            language: "en-US".to_string(),
            sample_rate: 16000,
            mode: RecognitionMode::Speech,
        }
    }
}

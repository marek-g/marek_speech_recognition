use crate::recognition_mode::RecognitionMode;

#[non_exhaustive]
pub struct RecognizerOptions {
    pub language: String,
    pub channel_count: i32,
    pub sample_rate: i32,
    pub mode: RecognitionMode,
}

impl Default for RecognizerOptions {
    fn default() -> Self {
        Self {
            language: "en-US".to_string(),
            channel_count: 1,
            sample_rate: 16000,
            mode: RecognitionMode::Speech,
        }
    }
}

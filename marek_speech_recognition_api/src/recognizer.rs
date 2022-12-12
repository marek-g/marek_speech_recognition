use crate::error::SpeechResult;

pub trait Recognizer {
    fn start(&mut self) -> SpeechResult;

    fn write(&mut self, buffer: &[u8]) -> SpeechResult;

    fn stop(&mut self) -> SpeechResult;
}

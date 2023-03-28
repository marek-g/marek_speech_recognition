use crate::{RecognizerInfo, SpeechResult};

pub trait Recognizer {
    fn info(&self) -> &RecognizerInfo;

    fn start(&mut self) -> SpeechResult;

    fn write(&mut self, buffer: &[i16]) -> SpeechResult;

    fn stop(&mut self) -> SpeechResult;
}

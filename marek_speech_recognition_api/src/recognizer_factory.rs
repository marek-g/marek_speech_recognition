use crate::{RecognitionEvent, Recognizer, RecognizerOptions, SpeechResult};
use futures::channel::mpsc::UnboundedReceiver;

pub trait RecognizerFactory {
    fn create_recognizer(
        &mut self,
        options: RecognizerOptions,
    ) -> SpeechResult<(
        Box<dyn Recognizer + Send>,
        UnboundedReceiver<RecognitionEvent>,
    )>;
}

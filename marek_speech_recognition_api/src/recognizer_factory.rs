use crate::{RecognitionEvent, Recognizer, RecognizerOptions, SpeechResult};
use futures::channel::mpsc::UnboundedReceiver;

pub trait RecognizerFactory<T: Recognizer> {
    fn create_recognizer(
        &mut self,
        options: RecognizerOptions,
    ) -> SpeechResult<(T, UnboundedReceiver<RecognitionEvent>)>;
}

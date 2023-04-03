use std::path::PathBuf;

use futures::channel::mpsc::UnboundedReceiver;
use marek_speech_recognition_api::{Recognizer, RecognizerFactory, SpeechError, SpeechResult};

use crate::VoskRecognizer;

pub struct VoskRecognizerFactory {
    models: Vec<VoskModelInfo>,
}

pub struct VoskModelInfo {
    pub language: String,
    pub folder: PathBuf,
}

impl VoskRecognizerFactory {
    pub fn new(models: Vec<VoskModelInfo>) -> SpeechResult<Self> {
        Ok(Self { models })
    }
}

impl RecognizerFactory for VoskRecognizerFactory {
    fn create_recognizer(
        &mut self,
        options: marek_speech_recognition_api::RecognizerOptions,
    ) -> marek_speech_recognition_api::SpeechResult<(
        Box<dyn Recognizer>,
        UnboundedReceiver<marek_speech_recognition_api::RecognitionEvent>,
    )> {
        let model_path = self
            .models
            .iter()
            .filter(|el| el.language == options.language)
            .map(|el| el.folder.clone())
            .next()
            .ok_or(SpeechError::NoLanguageFound(options.language))?;

        let (recognizer, receiver) =
            VoskRecognizer::new(&model_path, options.sample_rate, options.mode)?;

        Ok((Box::new(recognizer), receiver))
    }
}

use crate::GoogleRecognizer;
use futures::channel::mpsc::UnboundedReceiver;
use libsoda_sys::LibSoda;
use marek_speech_recognition_api::{
    RecognitionEvent, RecognizerFactory, RecognizerOptions, SpeechError, SpeechResult,
};
use std::path::PathBuf;
use std::sync::Arc;

pub struct GoogleRecognizerFactory {
    lib_soda: Arc<LibSoda>,
    language_packs_folder: PathBuf,
}

impl GoogleRecognizerFactory {
    pub fn new<T: Into<PathBuf>>(
        library_folder: T,
        language_packs_folder: T,
    ) -> SpeechResult<Self> {
        let lib_soda = LibSoda::load(library_folder)
            .map_err(|err| SpeechError::LoadLibraryError(format!("{:?}", err)))?;
        Ok(Self {
            lib_soda: Arc::new(lib_soda),
            language_packs_folder: language_packs_folder.into(),
        })
    }
}

impl RecognizerFactory<GoogleRecognizer> for GoogleRecognizerFactory {
    fn create_recognizer(
        &mut self,
        recognizer_options: RecognizerOptions,
    ) -> SpeechResult<(GoogleRecognizer, UnboundedReceiver<RecognitionEvent>)> {
        GoogleRecognizer::new(
            self.lib_soda.clone(),
            &self.language_packs_folder,
            recognizer_options,
        )
    }
}

use futures::channel::mpsc;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use libsoda_sys::extended_soda_config_msg::RecognitionMode;
use libsoda_sys::soda_endpoint_event::EndpointType;
use libsoda_sys::soda_recognition_result::ResultType;
use libsoda_sys::soda_response::SodaMessageType;
use libsoda_sys::{ExtendedSodaConfigMsg, LibSoda, SodaConfig, SodaHandle, SodaResponse};
use marek_speech_recognition_api::{
    RecognitionEvent, Recognizer, RecognizerOptions, SpeechError, SpeechResult,
};
use prost::Message;
use std::ffi::{c_char, c_int, c_void};
use std::fs;
use std::path::Path;
use std::sync::Arc;

pub struct GoogleRecognizer {
    lib_soda: Arc<LibSoda>,
    sender: *mut UnboundedSender<RecognitionEvent>,
    handle: SodaHandle,
}

unsafe impl Send for GoogleRecognizer {}

#[no_mangle]
pub extern "C" fn callback(
    response: *const c_char,
    res_length: c_int,
    callback_handle: *const c_void,
) {
    let slice = unsafe { std::slice::from_raw_parts(response as *const u8, res_length as usize) };

    if let Ok(result) = SodaResponse::decode(slice) {
        let event = if result.soda_type() == SodaMessageType::Start {
            Some(RecognitionEvent::Start)
        } else if result.soda_type() == SodaMessageType::Stop {
            Some(RecognitionEvent::Stop)
        } else if let Some(endpoint_event) = &result.endpoint_event {
            let audio_time_usec = if let Some(timing_metrics) = &endpoint_event.timing_metrics {
                timing_metrics.event_end_time_usec.map(|time| time as u64)
            } else {
                None
            };

            match endpoint_event.endpoint_type() {
                EndpointType::StartOfSpeech => {
                    Some(RecognitionEvent::StartOfSpeech { audio_time_usec })
                }
                EndpointType::EndOfSpeech => {
                    Some(RecognitionEvent::EndOfSpeech { audio_time_usec })
                }
                _ => None,
            }
        } else if let Some(mut recognition_event) = result.recognition_result {
            let audio_time_usec = if let Some(timing_metrics) = &recognition_event.timing_metrics {
                timing_metrics.event_end_time_usec.map(|time| time as u64)
            } else {
                None
            };

            Some(RecognitionEvent::Recognition {
                text: if !recognition_event.hypothesis.is_empty() {
                    recognition_event.hypothesis.swap_remove(0)
                } else {
                    "".to_string()
                },
                is_final: recognition_event.result_type() == ResultType::Final,
                audio_end_time_usec: audio_time_usec,
            })
        } else if let Some(langid_event) = result.langid_event {
            langid_event
                .language
                .map(|language| RecognitionEvent::Language { id: language })
        } else {
            None
        };

        if let Some(event) = event {
            let sender = callback_handle as *mut UnboundedSender<RecognitionEvent>;
            unsafe {
                sender.as_mut().unwrap().unbounded_send(event).unwrap();
            }
        }
    }
}

impl GoogleRecognizer {
    pub(crate) fn new(
        lib_soda: Arc<LibSoda>,
        language_packs_folder: &Path,
        recognizer_options: RecognizerOptions,
    ) -> SpeechResult<(Self, UnboundedReceiver<RecognitionEvent>)> {
        let folder =
            Self::get_language_pack_folder(language_packs_folder, &recognizer_options.language)?;

        unsafe {
            let config = ExtendedSodaConfigMsg {
                channel_count: Some(recognizer_options.channel_count),
                sample_rate: Some(recognizer_options.sample_rate),
                api_key: Some("dummy_api_key".to_string()),
                language_pack_directory: Some(folder),
                recognition_mode: Some(match recognizer_options.mode {
                    marek_speech_recognition_api::RecognitionMode::Ime => RecognitionMode::Ime,
                    marek_speech_recognition_api::RecognitionMode::Caption => {
                        RecognitionMode::Caption
                    }
                    _ => RecognitionMode::Ime,
                } as i32),
                include_timing_metrics: Some(true),
                ..ExtendedSodaConfigMsg::default()
            };
            let config_buf = config.encode_to_vec();

            let (sender, receiver) = mpsc::unbounded();
            let sender = Box::new(sender);
            let sender = Box::into_raw(sender);

            let soda_config = SodaConfig {
                soda_config: config_buf.as_ptr() as *const c_char,
                soda_config_size: config_buf.len() as c_int,
                callback: Some(callback),
                callback_handle: sender as *const c_void,
            };

            let handle = (lib_soda.create_soda_async)(soda_config);

            Ok((
                Self {
                    lib_soda,
                    sender,
                    handle,
                },
                receiver,
            ))
        }
    }

    fn get_language_pack_folder(
        language_packs_folder: &Path,
        language_name: &str,
    ) -> SpeechResult<String> {
        let folder = language_packs_folder.join(language_name);
        let versions =
            fs::read_dir(&folder).map_err(|_| SpeechError::LanguageFolderError(folder.clone()))?;
        let mut versions = versions
            .into_iter()
            .filter_map(|e| e.ok())
            .collect::<Vec<_>>();
        versions.sort_by_key(|b| std::cmp::Reverse(b.file_name()));
        let version = versions
            .first()
            .map(|v| v.file_name())
            .ok_or_else(|| SpeechError::LanguageFolderError(folder.clone()))?;
        let folder = folder.join(version).join("SODAModels");
        let folder = folder
            .as_path()
            .to_str()
            .ok_or_else(|| SpeechError::LanguageFolderError(folder.clone()))?
            .to_string();
        Ok(folder)
    }
}

impl Drop for GoogleRecognizer {
    fn drop(&mut self) {
        unsafe {
            (self.lib_soda.delete_soda_async)(self.handle);
            let _ = Box::from_raw(self.sender);
        }
    }
}

impl Recognizer for GoogleRecognizer {
    fn start(&mut self) -> SpeechResult<()> {
        unsafe {
            (self.lib_soda.soda_start)(self.handle);
        }

        Ok(())
    }

    fn write(&mut self, buffer: &[u8]) -> SpeechResult {
        unsafe {
            (self.lib_soda.add_audio)(
                self.handle,
                buffer.as_ptr() as *const c_char,
                buffer.len() as c_int,
            );
        }

        Ok(())
    }

    fn stop(&mut self) -> SpeechResult {
        unsafe {
            (self.lib_soda.soda_stop)(self.handle);
        }

        Ok(())
    }
}

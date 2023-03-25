use std::{
    path::{Path, PathBuf},
    thread::{self, JoinHandle},
};

use futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use marek_speech_recognition_api::{
    RecognitionEvent, RecognitionMode, Recognizer, SpeechError, SpeechResult, Word,
};

pub struct VoskRecognizer {
    model: vosk::Model,
    sample_rate: i32,
    recognition_mode: RecognitionMode,
    sender: UnboundedSender<RecognitionEvent>,
    vosk_thread_sender: Option<std::sync::mpsc::Sender<VoskRecognizerEvent>>,
    vosk_thread_handle: Option<JoinHandle<()>>,
}

impl VoskRecognizer {
    pub(crate) fn new(
        model_path: &Path,
        sample_rate: i32,
        recognition_mode: RecognitionMode,
    ) -> SpeechResult<(Self, UnboundedReceiver<RecognitionEvent>)> {
        let (sender, receiver) = mpsc::unbounded();
        let model = vosk::Model::new(
            model_path
                .as_os_str()
                .to_str()
                .ok_or_else(|| SpeechError::LanguageFolderError(PathBuf::from(model_path)))?,
        )
        .ok_or_else(|| SpeechError::LanguageFolderError(PathBuf::from(model_path)))?;

        Ok((
            VoskRecognizer {
                model,
                sample_rate,
                recognition_mode,
                sender,
                vosk_thread_sender: None,
                vosk_thread_handle: None,
            },
            receiver,
        ))
    }
}

enum VoskRecognizerEvent {
    Write(Vec<i16>),
    Stop,
}

impl Recognizer for VoskRecognizer {
    fn start(&mut self) -> SpeechResult {
        let mut recognizer = match &self.recognition_mode {
            RecognitionMode::Speech => vosk::Recognizer::new(&self.model, self.sample_rate as f32)
                .ok_or_else(|| {
                    SpeechError::LoadLibraryError("Cannot create Vosk recognizer".to_string())
                })?,
            RecognitionMode::Commands(commands) => {
                vosk::Recognizer::new_with_grammar(&self.model, self.sample_rate as f32, commands)
                    .ok_or_else(|| {
                    SpeechError::LoadLibraryError("Cannot create Vosk recognizer".to_string())
                })?
            }
            _ => panic!(
                "Recognition mode {:#?} not implemented",
                self.recognition_mode
            ),
        };

        recognizer.set_max_alternatives(0);
        recognizer.set_words(true); // include metadata about words in final result
        recognizer.set_partial_words(true); // include metadata about words in partial result

        let (sender, receiver) = std::sync::mpsc::channel();
        self.vosk_thread_sender = Some(sender);

        let result_sender = self.sender.clone();

        self.vosk_thread_handle = Some(thread::spawn(move || {
            let mut last_recognition_event = None;

            result_sender
                .unbounded_send(RecognitionEvent::Start)
                .unwrap();

            loop {
                let event = receiver.recv().unwrap();
                match event {
                    VoskRecognizerEvent::Write(buffer) => match recognizer.accept_waveform(&buffer)
                    {
                        vosk::DecodingState::Running => partial_result(
                            &mut recognizer,
                            &mut last_recognition_event,
                            &result_sender,
                        ),
                        vosk::DecodingState::Finalized => finalized_result(
                            &mut recognizer,
                            &mut last_recognition_event,
                            &result_sender,
                        ),
                        vosk::DecodingState::Failed => panic!("VoskRecognizer error!"),
                    },
                    VoskRecognizerEvent::Stop => break,
                }
            }

            finalized_result(&mut recognizer, &mut last_recognition_event, &result_sender);

            result_sender
                .unbounded_send(RecognitionEvent::Stop)
                .unwrap();
        }));

        Ok(())
    }

    fn write(&mut self, buffer: &[i16]) -> SpeechResult {
        if let Some(sender) = &self.vosk_thread_sender {
            sender
                .send(VoskRecognizerEvent::Write(Vec::from(buffer)))
                .unwrap();
        }
        Ok(())
    }

    fn stop(&mut self) -> SpeechResult {
        if let Some(sender) = &self.vosk_thread_sender {
            sender.send(VoskRecognizerEvent::Stop).unwrap();
        }

        if let Some(handle) = self.vosk_thread_handle.take() {
            handle.join().unwrap();
        }

        Ok(())
    }
}

fn partial_result(
    recognizer: &mut vosk::Recognizer,
    last_recognition_event: &mut Option<RecognitionEvent>,
    result_sender: &UnboundedSender<RecognitionEvent>,
) {
    let result = recognizer.partial_result();
    let text = result.partial;
    let words = result.partial_result;

    send_recognition_event(result_sender, last_recognition_event, text, words, false);
}

fn finalized_result(
    recognizer: &mut vosk::Recognizer,
    last_recognition_event: &mut Option<RecognitionEvent>,
    result_sender: &UnboundedSender<RecognitionEvent>,
) {
    let result = recognizer.result().single().unwrap();
    let text = result.text;
    let words = result.result;

    send_recognition_event(result_sender, last_recognition_event, text, words, true);
}

fn send_recognition_event(
    result_sender: &UnboundedSender<RecognitionEvent>,
    last_recognition_event: &mut Option<RecognitionEvent>,
    text: &str,
    words: Vec<vosk::Word>,
    is_final: bool,
) {
    if words.len() > 0 && text.len() > 0 {
        let recognition_event = Some(to_recognition_event(text, words, is_final));

        if recognition_event != *last_recognition_event {
            if let Some(recognition_event) = recognition_event {
                result_sender
                    .unbounded_send(recognition_event.clone())
                    .unwrap();
                last_recognition_event.replace(recognition_event);
            }
        }
    }
}

fn to_recognition_event(text: &str, words: Vec<vosk::Word>, is_final: bool) -> RecognitionEvent {
    RecognitionEvent::Recognition {
        text: text.to_string(),
        is_final,
        audio_start_time_usec: Some((words[0].start * 1000000f32) as u64),
        audio_end_time_usec: Some((words[words.len() - 1].end * 1000000f32) as u64),
        words: Some(
            words
                .into_iter()
                .map(|word| Word {
                    conf: word.conf,
                    start_time_usec: (word.start * 1000000f32) as u64,
                    end_time_usec: (word.end * 1000000f32) as u64,
                    word: word.word.to_string(),
                })
                .collect(),
        ),
    }
}

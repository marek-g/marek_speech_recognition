use async_trait::async_trait;

use crate::{RecognizerInfo, SpeechResult};

#[async_trait]
pub trait Recognizer {
    /// Returns information about the Recognizer.
    fn info(&self) -> &RecognizerInfo;

    /// Starts the recognition.
    async fn start(&mut self) -> SpeechResult;

    /// Process new chunk of data.
    ///
    /// It waits the time needed to process the data,
    /// so the stop method can always be fast.
    ///
    /// For 'is_realtime_only' mode the different behaviour is better.
    /// The first call is quick, but the subsequent calls should wait
    /// for the right (real) time.
    async fn write(&mut self, buffer: &[i16]) -> SpeechResult;

    /// Stops the recognition.
    /// Finish processing all sent buffers.
    async fn stop(&mut self) -> SpeechResult;
}

#[derive(Debug)]
#[non_exhaustive]
pub enum RecognitionEvent {
    /// Started listening.
    Start,

    /// Stopped listening.
    Stop,

    /// A start-of-speech moment has been detected at this time. Audio currently
    /// contains speech.
    StartOfSpeech { audio_time_usec: Option<u64> },

    /// End of speech has been detected by the end pointer, audio does not contain
    /// speech right now.
    EndOfSpeech { audio_time_usec: Option<u64> },

    /// Speech was recognized.
    Recognition {
        text: String,
        is_final: bool,
        audio_end_time_usec: Option<u64>,
    },

    /// Locale detected, e.g. "en-us" or "af-za"
    Language { id: String },
}

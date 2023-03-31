pub struct RecognizerInfo {
    /// Name of the recognizer
    pub name: String,

    /// Some recognizers like Google's libsoda do not work
    /// in a different speed than in realtime. In such case it is set to 'true'.
    pub is_realtime_only: bool,

    /// Does output has punctuation.
    pub has_punctuation: bool,
}

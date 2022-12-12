# Marek Speech Recognition

Rust project to allow easy usage of Speech Recognition libraries with a common API.

## Supported backends:

- `marek_google_speech_recognition` - Google Chrome's `libsoda` wrapper. Fast, offline and accurate, but unfortunately `libsoda` is secured with an API key and stack verification.

## Examples

- `speech_recognition_test` - uses `marek_google_speech_recognizer` to recognize speech. To run it you need to have `data/`, `SODALanguagePacks/` and `soda` library in the current folder

# Marek Speech Recognition

Rust project to allow easy usage of Speech Recognition libraries with a common API.

## Supported backends:

- `marek_google_speech_recognition` - Google Chrome's `libsoda` wrapper. Fast, offline and accurate, but unfortunately `libsoda` is secured with an API key and stack verification. Tested on Linux and Windows (mingw).

- `marek_vosk_speech_recognition` - [Vosk](https://alphacephei.com/vosk/) wrapper. Fast, offline, accurate, mmulti-language, open-source. Does not support punctation yet.

## Examples

- `speech_recognition_test` - recognize speech from wave example file using choosen backend.

# Marek Speech Recognition

Rust project to allow easy usage of Speech Recognition libraries with a common API.

## Supported backends:

- `marek_google_speech_recognition` - Google Chrome's `libsoda` wrapper. Fast, offline and accurate, but unfortunately `libsoda` is secured with an API key and stack verification. Tested on Linux and Windows (mingw).

- `marek_vosk_speech_recognition` - [Vosk](https://alphacephei.com/vosk/) wrapper. Fast, offline, accurate, mmulti-language, open-source. Does not support punctation yet.

## Examples

- `speech_recognition_test` - recognize speech from wave example file using choosen backend.

## Research

## Whisper.cpp

- https://github.com/ggml-org/whisper.cpp - multilingual, C++

Allows to run the best available model (as of 2025) - whisper-large-v3 in realtime on GPU (a few times faster). It supports low-end GPUs through Vulkan and CUDA (tested on GTX 1070 Ti).

The good enough model is ggml-large-v3-q5_0.bin - which takes about 2-2.5 GB VRAM.

Probably the best choice to run on GPU.

### NVIDIA NeMo - Parakeet

- https://huggingface.co/nvidia/parakeet-tdt-0.6b-v3 - multilingual, Python

It is currently the fastest large model. Mulilingual, punctation. It can run without problems on the CPU (a few times faster than realtime on a single core). But I was not able to run in on the GTX 1070 (it has cuda compute capability 6.1, required 7.0).

Probably the best choice to run on CPU.

### Sherpa ONNX

- https://github.com/k2-fsa/sherpa-onnx multilingual, C++
- Rust bindings: https://github.com/thewh1teagle/sherpa-rs

Has a great potential. Can run multiple models (including Whisper and NeMo). Didn't try it on the GPU yet (it may be poss. The problem is the available models are quantized to int8, which gives only medium quality.

It may be worth to look again at it when it starts supporting different quantizations.

Usage of parakeet tdt 0.6 v3 with Sherpa: https://k2-fsa.github.io/sherpa/onnx/pretrained_models/offline-transducer/nemo-transducer-models.html#sherpa-onnx-nemo-parakeet-tdt-0-6b-v3-int8-25-european-languages
Uses about 3GB of RAM.


Build sherpa-onnx (Linux instructions: https://k2-fsa.github.io/sherpa/onnx/install/linux.html)

``` shell
git clone https://github.com/k2-fsa/sherpa-onnx
cd sherpa-onnx
mkdir build
cd build
cmake -DCMAKE_BUILD_TYPE=Release ..
make -j6
```


Download precompiled sherpa onnx and build sherpa-rs:

``` shell
wget https://github.com/k2-fsa/sherpa-onnx/releases/download/v1.12.9/sherpa-onnx-v1.12.9-linux-x64-static.tar.bz2
tar xf sherpa-onnx-v1.12.9-linux-x64-static.tar.bz2
rm sherpa-onnx-v1.12.9-linux-x64-static.tar.bz2
export SHERPA_LIB_PATH="$(pwd)/sherpa-onnx-v1.12.9-linux-x64-static"
export RUSTFLAGS="-C relocation-model=dynamic-no-pic"
cargo build
```

Run example:

``` shell
wget https://github.com/k2-fsa/sherpa-onnx/releases/download/asr-models/sherpa-onnx-nemo-parakeet-tdt-0.6b-v3-int8.tar.bz2
tar xvf sherpa-onnx-nemo-parakeet-tdt-0.6b-v3-int8.tar.bz2
rm sherpa-onnx-nemo-parakeet-tdt-0.6b-v3-int8.tar.bz2
wget https://github.com/thewh1teagle/sherpa-rs/releases/download/v0.1.0/motivation.wav -O motivation.wav
cargo run --example parakeet motivation.wav
```

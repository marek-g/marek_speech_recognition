# libsoda_sys

Google Chrome's `libsoda` wrapper.  `libsoda` is fast and accurate and works offline.

Unfortunately, `libsoda` is secured with an API key and stack verification, so it cannot be used with other software without modifications. How to make such modifications is out of scope of this project. 

Tested with `libsoda` v1.1.1.2 (as of December 2022).

Based on:
- https://github.com/biemster/gasr
- https://chromium.googlesource.com/chromium/src/+/refs/heads/main/chrome/services/speech/soda.

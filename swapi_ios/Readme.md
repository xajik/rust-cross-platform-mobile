# Enviroment setup:
https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-06-rust-on-ios.html

### Add target:
rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios

### install build tool:
* cargo install cargo-lipo
* cargo install --force cbindgen

### Build with:
scripts/release_build_ios

### Link frameworks & libraries
* `../swapi_ios/target/universal/release/libswapi.a`
* `libresolv.tbd`

### Crete bridging header:

```Objective-C
#ifndef Greetings_Bridging_Header_h
#define Greetings_Bridging_Header_h

#import "swapi_native.h"

#endif
```

### Add headers to IOS project (drag&drop):
* `.../swapi_ios/src/ffi/swapi_native.h`
* `.../swapi_core/src/swapi_core.h`

### Add library search path:

* `../swapi_ios/target/universal/release`

# Ready to build & run
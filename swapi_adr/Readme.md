## Enviroment setup:
[official example](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html)

## NDK
Install NDK >= 20

### Exports:
export ANDROID_HOME=/Users/$USER/Library/Android/sdk
export NDK_HOME=$ANDROID_HOME/ndk-bundle

### Create NDK toolchain (you can reuse it for multiple projects):
```
mkdir NDK
${NDK_HOME}/build/tools/make_standalone_toolchain.py --api 26 --arch arm64 --install-dir NDK/arm64
${NDK_HOME}/build/tools/make_standalone_toolchain.py --api 26 --arch arm --install-dir NDK/arm
${NDK_HOME}/build/tools/make_standalone_toolchain.py --api 26 --arch x86 --install-dir NDK/x86
```

### Create:
cargo-config.toml and populate with:

```
[target.aarch64-linux-android]
ar = "<ndk path>/NDK/arm64/bin/aarch64-linux-android-ar"
linker = "<ndk path>/NDK/arm64/bin/aarch64-linux-android-clang"

[target.armv7-linux-androideabi]
ar = "<ndk path>/NDK/arm/bin/arm-linux-androideabi-ar"
linker = "<ndk path>/NDK/arm/bin/arm-linux-androideabi-clang"

[target.i686-linux-android]
ar = "<ndk path>/NDK/x86/bin/i686-linux-android-ar"
linker = "<ndk path>/NDK/x86/bin/i686-linux-android-clang"
```

### Copy it cargo install dir:
`cp cargo-config.toml ~/.cargo/config`

## Add target:
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android

## Build project:
run> `scripts/release_build`

## Create link from build files to android project. (You need to run it only once)
run> `scripts/create_s_link`

## JNI Bridge

JNI brigge is generated with [rust_swig](https://github.com/Dushistov/rust_swig) library

### Example of Gradle integration with rust_swig:

[rust_swig/android-example](https://github.com/Dushistov/rust_swig/tree/master/android-example)

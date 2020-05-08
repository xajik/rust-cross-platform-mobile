# swapi-rust-mobile
Crossplatform mobile solution based on Rust for IOS &amp; Android.

***More details here:["Rust & cross-platform mobile development"](https://medium.com/@igor.stebliy/rust-cross-platform-mobile-development-9117a67ac9b7)***

In this project you'll find Rust network client, that laods and serialize data from 
[star wars API](https://swapi.dev/people) and pass it to Android and IOS.

Android                    |  IOS
:-------------------------:|:-------------------------:
![](/content/swapi_adr.png)  |  ![](/content/swapi_ios.png)

# Architecture

Project consist of sub-project:
* Android applicaition
* IOS application
* Rust core module
* Rust Android bridge
* Rust IOS bridge

Architecture               |
:-------------------------:|
<img src="/content/bridge_arch.png" width="500">|

## swapi_core
 module wtih shared logic between IOS& Android.
 
More details: [Readme](/swapi_core/Readme.md)

## swapi_adr
Android specific module, that depends on [swapi_core] for networking and provide custom build for JNI communicaiton.

More details: [Readme](/swapi_adr/Readme.md)

## swapi_ios
IOS specific module, that depends on [swapi_code] for networking and provide custom build for FFI communicaiton.

More details: [Readme](/swapi_ios/Readme.md)

# Build & Run

## Enviroment project
To setup enviroment follow instuctions in :
* swapi_adr/Readme.
* swapi_ios/Readme.
* swapi_core/Readme.

### Official guides

* [Android](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html)
* [IOS](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-06-rust-on-ios.html)

## Build:

Each project contain `scripts` folder to automate build.

### IOS:
* ./swapi_ios/scripts/build_release

### Andorid:
* ./swapi_adr/scripts/build_release
* ./swapi_adr/scripts/create_s_link - create symb link between generated files .so and Android project `jniLibs`

## Build:
* ./build_release (execute all 3 commands listed on top)
* ./clean - clean all 3 Rust project

# IDE

Based on my experience

## Rust has very good support with:
* InteliJ IDE CE 
* [Rust Plugin](https://intellij-rust.github.io/)

## Alternatvie:
* VSCode 
* [Rust plugin](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)

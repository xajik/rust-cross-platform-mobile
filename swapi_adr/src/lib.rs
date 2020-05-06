//Rust name convention is `_` not camel case
#![allow(non_snake_case)]

//Logs to Android
mod android_logger;
//rust_swig module
pub mod swig;

pub use crate::swig::java_glue::*;

//JNI imports
extern crate jni;

use self::jni::JNIEnv;
use self::jni::objects::JClass;

#[no_mangle]
pub unsafe extern fn Java_com_rust_app_swapiclient_swapi_Logger_initLogger(_env: JNIEnv, _: JClass) {
    android_logger::initLogger();
}
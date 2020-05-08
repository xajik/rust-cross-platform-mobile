#[allow(dead_code)]
mod internal_aliases {
    use super::*;
    pub type JStringOptStr = jstring;
    pub type JOptionalInt = jobject;
    pub type JInteger = jobject;
    pub type JByte = jobject;
    pub type JShort = jobject;
    pub type JFloat = jobject;
    pub type JDouble = jobject;
    pub type JOptionalDouble = jobject;
    pub type JLong = jobject;
    pub type JOptionalLong = jobject;
    #[repr(transparent)]
    pub struct JForeignObjectsArray<T: SwigForeignClass> {
        pub(crate) inner: jobjectArray,
        pub(crate) _marker: ::std::marker::PhantomData<T>,
    }
    pub type JStringPath = jstring;
}
#[doc = " Default JNI_VERSION"]
const SWIG_JNI_VERSION: jint = JNI_VERSION_1_6 as jint;
#[doc = " Marker for what to cache in JNI_OnLoad"]
#[allow(unused_macros)]
macro_rules! swig_jni_find_class {
    ( $ id : ident , $ path : expr ) => {
        unsafe { $id }
    };
    ( $ id : ident , $ path : expr , ) => {
        unsafe { $id }
    };
}
#[allow(unused_macros)]
macro_rules! swig_jni_get_method_id {
    ( $ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr ) => {
        unsafe { $global_id }
    };
    ( $ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr , ) => {
        unsafe { $global_id }
    };
}
#[allow(unused_macros)]
macro_rules! swig_jni_get_static_method_id {
    ( $ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr ) => {
        unsafe { $global_id }
    };
    ( $ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr , ) => {
        unsafe { $global_id }
    };
}
#[allow(unused_macros)]
macro_rules! swig_jni_get_field_id {
    ( $ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr ) => {
        unsafe { $global_id }
    };
    ( $ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr , ) => {
        unsafe { $global_id }
    };
}
#[allow(unused_macros)]
macro_rules! swig_jni_get_static_field_id {
    ( $ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr ) => {
        unsafe { $global_id }
    };
    ( $ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr , ) => {
        unsafe { $global_id }
    };
}
#[allow(dead_code)]
#[doc = ""]
trait SwigInto<T> {
    fn swig_into(self, env: *mut JNIEnv) -> T;
}
#[allow(dead_code)]
#[doc = ""]
trait SwigFrom<T> {
    fn swig_from(_: T, env: *mut JNIEnv) -> Self;
}
#[allow(dead_code)]
#[doc = ""]
trait SwigDeref {
    type Target: ?Sized;
    fn swig_deref(&self) -> &Self::Target;
}
#[allow(dead_code)]
#[doc = ""]
trait SwigDerefMut {
    type Target: ?Sized;
    fn swig_deref_mut(&mut self) -> &mut Self::Target;
}
#[allow(unused_macros)]
macro_rules! swig_c_str {
    ( $ lit : expr ) => {
        concat!($lit, "\0").as_ptr() as *const ::std::os::raw::c_char
    };
}
#[allow(unused_macros)]
macro_rules ! swig_assert_eq_size { ( $ x : ty , $ ( $ xs : ty ) ,+ $ ( , ) * ) => { $ ( let _ = :: std :: mem :: transmute ::<$ x , $ xs >; ) + } ; }
#[cfg(target_pointer_width = "32")]
pub unsafe fn jlong_to_pointer<T>(val: jlong) -> *mut T {
    (val as u32) as *mut T
}
#[cfg(target_pointer_width = "64")]
pub unsafe fn jlong_to_pointer<T>(val: jlong) -> *mut T {
    val as *mut T
}
#[allow(dead_code)]
pub trait SwigForeignClass {
    type PointedType;
    fn jni_class() -> jclass;
    fn jni_class_pointer_field() -> jfieldID;
    fn box_object(x: Self) -> jlong;
    fn unbox_object(x: jlong) -> Self;
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType>;
}
#[allow(dead_code)]
pub trait SwigForeignCLikeEnum {
    fn as_jint(&self) -> jint;
    #[doc = " # Panics"]
    #[doc = " Panics on error"]
    fn from_jint(_: jint) -> Self;
}
#[allow(dead_code)]
pub struct JavaString {
    string: jstring,
    chars: *const ::std::os::raw::c_char,
    env: *mut JNIEnv,
}
#[allow(dead_code)]
impl JavaString {
    pub fn new(env: *mut JNIEnv, js: jstring) -> JavaString {
        let chars = if !js.is_null() {
            unsafe { (**env).GetStringUTFChars.unwrap()(env, js, ::std::ptr::null_mut()) }
        } else {
            ::std::ptr::null_mut()
        };
        JavaString {
            string: js,
            chars: chars,
            env: env,
        }
    }
    pub fn to_str(&self) -> &str {
        if !self.chars.is_null() {
            let s = unsafe { ::std::ffi::CStr::from_ptr(self.chars) };
            s.to_str().unwrap()
        } else {
            ""
        }
    }
}
#[allow(dead_code)]
impl Drop for JavaString {
    fn drop(&mut self) {
        assert!(!self.env.is_null());
        if !self.string.is_null() {
            assert!(!self.chars.is_null());
            unsafe {
                (**self.env).ReleaseStringUTFChars.unwrap()(self.env, self.string, self.chars)
            };
            self.env = ::std::ptr::null_mut();
            self.chars = ::std::ptr::null_mut();
        }
    }
}
#[allow(dead_code)]
struct JavaCallback {
    java_vm: *mut JavaVM,
    this: jobject,
    methods: Vec<jmethodID>,
}
#[doc = " According to JNI spec it should be safe to"]
#[doc = " pass pointer to JavaVm and jobject (global) across threads"]
unsafe impl Send for JavaCallback {}
#[allow(dead_code)]
struct JniEnvHolder<'a> {
    env: Option<*mut JNIEnv>,
    callback: &'a JavaCallback,
    need_detach: bool,
}
#[allow(dead_code)]
impl<'a> Drop for JniEnvHolder<'a> {
    fn drop(&mut self) {
        if self.need_detach {
            let res = unsafe {
                (**self.callback.java_vm).DetachCurrentThread.unwrap()(self.callback.java_vm)
            };
            if res != 0 {
                log::error!("JniEnvHolder: DetachCurrentThread failed: {}", res);
            }
        }
    }
}
#[allow(dead_code)]
impl JavaCallback {
    fn new(obj: jobject, env: *mut JNIEnv) -> JavaCallback {
        let mut java_vm: *mut JavaVM = ::std::ptr::null_mut();
        let ret = unsafe { (**env).GetJavaVM.unwrap()(env, &mut java_vm) };
        assert_eq!(0, ret, "GetJavaVm failed");
        let global_obj = unsafe { (**env).NewGlobalRef.unwrap()(env, obj) };
        assert!(!global_obj.is_null());
        JavaCallback {
            java_vm,
            this: global_obj,
            methods: Vec::new(),
        }
    }
    fn get_jni_env(&self) -> JniEnvHolder {
        assert!(!self.java_vm.is_null());
        let mut env: *mut JNIEnv = ::std::ptr::null_mut();
        let res = unsafe {
            (**self.java_vm).GetEnv.unwrap()(
                self.java_vm,
                (&mut env) as *mut *mut JNIEnv as *mut *mut ::std::os::raw::c_void,
                SWIG_JNI_VERSION,
            )
        };
        if res == (JNI_OK as jint) {
            return JniEnvHolder {
                env: Some(env),
                callback: self,
                need_detach: false,
            };
        }
        if res != (JNI_EDETACHED as jint) {
            panic!("get_jni_env: GetEnv return error `{}`", res);
        }
        trait ConvertPtr<T> {
            fn convert_ptr(self) -> T;
        }
        impl ConvertPtr<*mut *mut ::std::os::raw::c_void> for *mut *mut JNIEnv {
            fn convert_ptr(self) -> *mut *mut ::std::os::raw::c_void {
                self as *mut *mut ::std::os::raw::c_void
            }
        }
        impl ConvertPtr<*mut *mut JNIEnv> for *mut *mut JNIEnv {
            fn convert_ptr(self) -> *mut *mut JNIEnv {
                self
            }
        }
        let res = unsafe {
            (**self.java_vm).AttachCurrentThread.unwrap()(
                self.java_vm,
                (&mut env as *mut *mut JNIEnv).convert_ptr(),
                ::std::ptr::null_mut(),
            )
        };
        if res != 0 {
            log::error!(
                "JavaCallback::get_jnienv: AttachCurrentThread failed: {}",
                res
            );
            JniEnvHolder {
                env: None,
                callback: self,
                need_detach: false,
            }
        } else {
            assert!(!env.is_null());
            JniEnvHolder {
                env: Some(env),
                callback: self,
                need_detach: true,
            }
        }
    }
}
#[allow(dead_code)]
impl Drop for JavaCallback {
    fn drop(&mut self) {
        let env = self.get_jni_env();
        if let Some(env) = env.env {
            assert!(!env.is_null());
            unsafe { (**env).DeleteGlobalRef.unwrap()(env, self.this) };
        } else {
            log::error!("JavaCallback::drop failed, can not get JNIEnv");
        }
    }
}
#[allow(dead_code)]
fn jni_throw(env: *mut JNIEnv, ex_class: jclass, message: &str) {
    let c_message = ::std::ffi::CString::new(message).unwrap();
    let res = unsafe { (**env).ThrowNew.unwrap()(env, ex_class, c_message.as_ptr()) };
    if res != 0 {
        log::error!(
            "JNI ThrowNew({}) failed for class {:?} failed",
            message,
            ex_class
        );
    }
}
#[allow(dead_code)]
fn jni_throw_exception(env: *mut JNIEnv, message: &str) {
    let exception_class = swig_jni_find_class!(JAVA_LANG_EXCEPTION, "java/lang/Exception");
    jni_throw(env, exception_class, message)
}
#[allow(dead_code)]
fn object_to_jobject<T: SwigForeignClass>(env: *mut JNIEnv, obj: T) -> jobject {
    let jcls = <T>::jni_class();
    assert!(!jcls.is_null());
    let field_id = <T>::jni_class_pointer_field();
    assert!(!field_id.is_null());
    let jobj: jobject = unsafe { (**env).AllocObject.unwrap()(env, jcls) };
    assert!(!jobj.is_null(), "object_to_jobject: AllocObject failed");
    let ret: jlong = <T>::box_object(obj);
    unsafe {
        (**env).SetLongField.unwrap()(env, jobj, field_id, ret);
        if (**env).ExceptionCheck.unwrap()(env) != 0 {
            panic!("object_to_jobject: Can not set mNativeObj field: catch exception");
        }
    }
    jobj
}
#[allow(dead_code)]
fn jobject_array_to_vec_of_objects<T: SwigForeignClass + Clone>(
    env: *mut JNIEnv,
    arr: internal_aliases::JForeignObjectsArray<T>,
) -> Vec<T> {
    let field_id = <T>::jni_class_pointer_field();
    assert!(!field_id.is_null());
    let length = unsafe { (**env).GetArrayLength.unwrap()(env, arr.inner) };
    let len = <usize as ::std::convert::TryFrom<jsize>>::try_from(length)
        .expect("invalid jsize, in jsize => usize conversation");
    let mut result = Vec::with_capacity(len);
    for i in 0..length {
        let native: &mut T = unsafe {
            let obj = (**env).GetObjectArrayElement.unwrap()(env, arr.inner, i);
            if (**env).ExceptionCheck.unwrap()(env) != 0 {
                panic!("Failed to retrieve element {} from this `jobjectArray'", i);
            }
            let ptr = (**env).GetLongField.unwrap()(env, obj, field_id);
            let native = (jlong_to_pointer(ptr) as *mut T).as_mut().unwrap();
            (**env).DeleteLocalRef.unwrap()(env, obj);
            native
        };
        result.push(native.clone());
    }
    result
}
#[allow(dead_code)]
fn vec_of_objects_to_jobject_array<T: SwigForeignClass>(
    env: *mut JNIEnv,
    mut arr: Vec<T>,
) -> internal_aliases::JForeignObjectsArray<T> {
    let jcls: jclass = <T>::jni_class();
    assert!(!jcls.is_null());
    let arr_len = <jsize as ::std::convert::TryFrom<usize>>::try_from(arr.len())
        .expect("invalid usize, in usize => to jsize conversation");
    let obj_arr: jobjectArray =
        unsafe { (**env).NewObjectArray.unwrap()(env, arr_len, jcls, ::std::ptr::null_mut()) };
    assert!(!obj_arr.is_null());
    let field_id = <T>::jni_class_pointer_field();
    assert!(!field_id.is_null());
    for (i, r_obj) in arr.drain(..).enumerate() {
        let jobj: jobject = unsafe { (**env).AllocObject.unwrap()(env, jcls) };
        assert!(!jobj.is_null());
        let r_obj: jlong = <T>::box_object(r_obj);
        unsafe {
            (**env).SetLongField.unwrap()(env, jobj, field_id, r_obj);
            if (**env).ExceptionCheck.unwrap()(env) != 0 {
                panic!("Can not mNativeObj field: catch exception");
            }
            (**env).SetObjectArrayElement.unwrap()(env, obj_arr, i as jsize, jobj);
            if (**env).ExceptionCheck.unwrap()(env) != 0 {
                panic!("SetObjectArrayElement({}) failed", i);
            }
            (**env).DeleteLocalRef.unwrap()(env, jobj);
        }
    }
    internal_aliases::JForeignObjectsArray {
        inner: obj_arr,
        _marker: ::std::marker::PhantomData,
    }
}
#[allow(dead_code)]
trait JniInvalidValue {
    fn jni_invalid_value() -> Self;
}
impl<T> JniInvalidValue for *const T {
    fn jni_invalid_value() -> Self {
        ::std::ptr::null()
    }
}
impl<T> JniInvalidValue for *mut T {
    fn jni_invalid_value() -> Self {
        ::std::ptr::null_mut()
    }
}
impl JniInvalidValue for () {
    fn jni_invalid_value() {}
}
impl<T: SwigForeignClass> JniInvalidValue for internal_aliases::JForeignObjectsArray<T> {
    fn jni_invalid_value() -> Self {
        Self {
            inner: ::std::ptr::null_mut(),
            _marker: ::std::marker::PhantomData,
        }
    }
}
macro_rules ! impl_jni_jni_invalid_value { ( $ ( $ type : ty ) * ) => ( $ ( impl JniInvalidValue for $ type { fn jni_invalid_value ( ) -> Self { <$ type >:: default ( ) } } ) * ) }
impl_jni_jni_invalid_value! { jbyte jshort jint jlong jfloat jdouble }
#[allow(dead_code)]
pub fn u64_to_jlong_checked(x: u64) -> jlong {
    <jlong as ::std::convert::TryFrom<u64>>::try_from(x)
        .expect("invalid u64, in u64 => jlong conversation")
}
#[allow(dead_code)]
fn from_std_string_jstring(x: String, env: *mut JNIEnv) -> jstring {
    let x = x.into_bytes();
    unsafe {
        let x = ::std::ffi::CString::from_vec_unchecked(x);
        (**env).NewStringUTF.unwrap()(env, x.as_ptr())
    }
}
macro_rules ! define_array_handling_code { ( $ ( [ jni_arr_type = $ jni_arr_type : ident , rust_arr_wrapper = $ rust_arr_wrapper : ident , jni_get_array_elements = $ jni_get_array_elements : ident , jni_elem_type = $ jni_elem_type : ident , rust_elem_type = $ rust_elem_type : ident , jni_release_array_elements = $ jni_release_array_elements : ident , jni_new_array = $ jni_new_array : ident , jni_set_array_region = $ jni_set_array_region : ident ] ) ,* ) => { $ ( # [ allow ( dead_code ) ] struct $ rust_arr_wrapper { array : $ jni_arr_type , data : * mut $ jni_elem_type , env : * mut JNIEnv , } # [ allow ( dead_code ) ] impl $ rust_arr_wrapper { fn new ( env : * mut JNIEnv , array : $ jni_arr_type ) -> $ rust_arr_wrapper { assert ! ( ! array . is_null ( ) ) ; let data = unsafe { ( ** env ) .$ jni_get_array_elements . unwrap ( ) ( env , array , :: std :: ptr :: null_mut ( ) ) } ; $ rust_arr_wrapper { array , data , env } } fn to_slice ( & self ) -> & [ $ rust_elem_type ] { unsafe { let len : jsize = ( ** self . env ) . GetArrayLength . unwrap ( ) ( self . env , self . array ) ; assert ! ( ( len as u64 ) <= ( usize :: max_value ( ) as u64 ) ) ; :: std :: slice :: from_raw_parts ( self . data , len as usize ) } } fn from_slice_to_raw ( arr : & [ $ rust_elem_type ] , env : * mut JNIEnv ) -> $ jni_arr_type { assert ! ( ( arr . len ( ) as u64 ) <= ( jsize :: max_value ( ) as u64 ) ) ; let jarr : $ jni_arr_type = unsafe { ( ** env ) .$ jni_new_array . unwrap ( ) ( env , arr . len ( ) as jsize ) } ; assert ! ( ! jarr . is_null ( ) ) ; unsafe { ( ** env ) .$ jni_set_array_region . unwrap ( ) ( env , jarr , 0 , arr . len ( ) as jsize , arr . as_ptr ( ) ) ; if ( ** env ) . ExceptionCheck . unwrap ( ) ( env ) != 0 { panic ! ( "{}:{} {} failed" , file ! ( ) , line ! ( ) , stringify ! ( $ jni_set_array_region ) ) ; } } jarr } } # [ allow ( dead_code ) ] impl Drop for $ rust_arr_wrapper { fn drop ( & mut self ) { assert ! ( ! self . env . is_null ( ) ) ; assert ! ( ! self . array . is_null ( ) ) ; unsafe { ( ** self . env ) .$ jni_release_array_elements . unwrap ( ) ( self . env , self . array , self . data , JNI_ABORT as jint , ) } ; } } ) * } }
define_array_handling_code!(
    [
        jni_arr_type = jbyteArray,
        rust_arr_wrapper = JavaByteArray,
        jni_get_array_elements = GetByteArrayElements,
        jni_elem_type = jbyte,
        rust_elem_type = i8,
        jni_release_array_elements = ReleaseByteArrayElements,
        jni_new_array = NewByteArray,
        jni_set_array_region = SetByteArrayRegion
    ],
    [
        jni_arr_type = jshortArray,
        rust_arr_wrapper = JavaShortArray,
        jni_get_array_elements = GetShortArrayElements,
        jni_elem_type = jshort,
        rust_elem_type = i16,
        jni_release_array_elements = ReleaseShortArrayElements,
        jni_new_array = NewShortArray,
        jni_set_array_region = SetShortArrayRegion
    ],
    [
        jni_arr_type = jintArray,
        rust_arr_wrapper = JavaIntArray,
        jni_get_array_elements = GetIntArrayElements,
        jni_elem_type = jint,
        rust_elem_type = i32,
        jni_release_array_elements = ReleaseIntArrayElements,
        jni_new_array = NewIntArray,
        jni_set_array_region = SetIntArrayRegion
    ],
    [
        jni_arr_type = jlongArray,
        rust_arr_wrapper = JavaLongArray,
        jni_get_array_elements = GetLongArrayElements,
        jni_elem_type = jlong,
        rust_elem_type = i64,
        jni_release_array_elements = ReleaseLongArrayElements,
        jni_new_array = NewLongArray,
        jni_set_array_region = SetLongArrayRegion
    ],
    [
        jni_arr_type = jfloatArray,
        rust_arr_wrapper = JavaFloatArray,
        jni_get_array_elements = GetFloatArrayElements,
        jni_elem_type = jfloat,
        rust_elem_type = f32,
        jni_release_array_elements = ReleaseFloatArrayElements,
        jni_new_array = NewFloatArray,
        jni_set_array_region = SetFloatArrayRegion
    ],
    [
        jni_arr_type = jdoubleArray,
        rust_arr_wrapper = JavaDoubleArray,
        jni_get_array_elements = GetDoubleArrayElements,
        jni_elem_type = jdouble,
        rust_elem_type = f64,
        jni_release_array_elements = ReleaseDoubleArrayElements,
        jni_new_array = NewDoubleArray,
        jni_set_array_region = SetDoubleArrayRegion
    ]
);
#[allow(dead_code)]
fn to_java_util_optional_double(
    env: *mut JNIEnv,
    x: Option<f64>,
) -> internal_aliases::JOptionalDouble {
    let class: jclass = swig_jni_find_class!(JAVA_UTIL_OPTIONAL_DOUBLE, "java/util/OptionalDouble");
    assert!(!class.is_null(),);
    match x {
        Some(val) => {
            let of_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_DOUBLE_OF,
                JAVA_UTIL_OPTIONAL_DOUBLE,
                "of",
                "(D)Ljava/util/OptionalDouble;"
            );
            assert!(!of_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, of_m, val);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalDouble.of failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
        None => {
            let empty_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_DOUBLE_EMPTY,
                JAVA_UTIL_OPTIONAL_DOUBLE,
                "empty",
                "()Ljava/util/OptionalDouble;"
            );
            assert!(!empty_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, empty_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalDouble.empty failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_double_to_rust(env: *mut JNIEnv, x: internal_aliases::JDouble) -> Option<f64> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_DOUBLE, "java/lang/Double");
            assert!(!class.is_null());
            let double_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_DOUBLE_DOUBLE_VALUE_METHOD,
                JAVA_LANG_DOUBLE,
                "doubleValue",
                "()D",
            );
            assert!(!double_value_m.is_null(),);
            let ret: f64 = unsafe {
                let ret = (**env).CallDoubleMethod.unwrap()(env, x, double_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Double.doubleValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_float_to_rust(env: *mut JNIEnv, x: internal_aliases::JFloat) -> Option<f32> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_FLOAT, "java/lang/Float");
            assert!(!class.is_null());
            let float_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_FLOAT_FLOAT_VALUE,
                JAVA_LANG_FLOAT,
                "floatValue",
                "()F"
            );
            assert!(!float_value_m.is_null());
            let ret: f32 = unsafe {
                let ret = (**env).CallFloatMethod.unwrap()(env, x, float_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Float.floatValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn to_java_util_optional_long(env: *mut JNIEnv, x: Option<i64>) -> internal_aliases::JOptionalLong {
    let class: jclass = swig_jni_find_class!(JAVA_UTIL_OPTIONAL_LONG, "java/util/OptionalLong");
    assert!(!class.is_null(),);
    match x {
        Some(val) => {
            let of_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_LONG_OF,
                JAVA_UTIL_OPTIONAL_LONG,
                "of",
                "(J)Ljava/util/OptionalLong;"
            );
            assert!(!of_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, of_m, val);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalLong.of failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
        None => {
            let empty_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_LONG_EMPTY,
                JAVA_UTIL_OPTIONAL_LONG,
                "empty",
                "()Ljava/util/OptionalLong;",
            );
            assert!(!empty_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, empty_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalLong.empty failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_long_to_rust(env: *mut JNIEnv, x: internal_aliases::JLong) -> Option<i64> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_LONG, "java/lang/Long");
            assert!(!class.is_null());
            let long_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_LONG_LONG_VALUE,
                JAVA_LANG_LONG,
                "longValue",
                "()J"
            );
            assert!(!long_value_m.is_null());
            let ret: i64 = unsafe {
                let ret = (**env).CallLongMethod.unwrap()(env, x, long_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Long.longValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_int_to_rust(env: *mut JNIEnv, x: internal_aliases::JInteger) -> Option<i32> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_INTEGER, "java/lang/Integer");
            assert!(!class.is_null());
            let int_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_INTEGER_INT_VALUE,
                JAVA_LANG_INTEGER,
                "intValue",
                "()I"
            );
            assert!(!int_value_m.is_null(),);
            let ret: i32 = unsafe {
                let ret = (**env).CallIntMethod.unwrap()(env, x, int_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Integer.intValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_byte_to_rust(env: *mut JNIEnv, x: internal_aliases::JByte) -> Option<i8> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_BYTE, "java/lang/Byte");
            assert!(!class.is_null());
            let byte_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_BYTE_BYTE_VALUE,
                JAVA_LANG_BYTE,
                "byteValue",
                "()B"
            );
            assert!(!byte_value_m.is_null(),);
            let ret: i8 = unsafe {
                let ret = (**env).CallByteMethod.unwrap()(env, x, byte_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Byte.byteValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_short_to_rust(env: *mut JNIEnv, x: internal_aliases::JByte) -> Option<i16> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_SHORT, "java/lang/Short");
            assert!(!class.is_null());
            let short_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_SHORT_SHORT_VALUE,
                JAVA_LANG_SHORT,
                "shortValue",
                "()S"
            );
            assert!(!short_value_m.is_null());
            let ret: i16 = unsafe {
                let ret = (**env).CallShortMethod.unwrap()(env, x, short_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Short.shortValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn to_java_util_optional_int(env: *mut JNIEnv, x: Option<i32>) -> jobject {
    let class: jclass = swig_jni_find_class!(JAVA_UTIL_OPTIONAL_INT, "java/util/OptionalInt");
    assert!(!class.is_null(),);
    match x {
        Some(val) => {
            let of_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_INT_OF,
                JAVA_UTIL_OPTIONAL_INT,
                "of",
                "(I)Ljava/util/OptionalInt;"
            );
            assert!(!of_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, of_m, val);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalInt.of failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
        None => {
            let empty_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_INT_EMPTY,
                JAVA_UTIL_OPTIONAL_INT,
                "empty",
                "()Ljava/util/OptionalInt;"
            );
            assert!(!empty_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, empty_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalInt.empty failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
    }
}
use jni_sys::*;
use swapi_core::swapi::{People, SwapiCallback, SwapiClient};
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_rust_app_swapiclient_swapi_JNIReachabilityFence_reachabilityFence1(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_rust_app_swapiclient_swapi_JNIReachabilityFence_reachabilityFence2(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_rust_app_swapiclient_swapi_JNIReachabilityFence_reachabilityFence3(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_rust_app_swapiclient_swapi_JNIReachabilityFence_reachabilityFence4(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_rust_app_swapiclient_swapi_JNIReachabilityFence_reachabilityFence5(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_rust_app_swapiclient_swapi_JNIReachabilityFence_reachabilityFence6(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_rust_app_swapiclient_swapi_JNIReachabilityFence_reachabilityFence7(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_rust_app_swapiclient_swapi_JNIReachabilityFence_reachabilityFence8(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
impl SwigForeignClass for People {
    type PointedType = People;
    fn jni_class() -> jclass {
        swig_jni_find_class!(
            FOREIGN_CLASS_PEOPLE,
            "com/rust/app/swapiclient/swapi/People"
        )
    }
    fn jni_class_pointer_field() -> jfieldID {
        swig_jni_get_field_id!(
            FOREIGN_CLASS_PEOPLE_MNATIVEOBJ_FIELD,
            FOREIGN_CLASS_PEOPLE,
            "mNativeObj",
            "J"
        )
    }
    fn box_object(this: Self) -> jlong {
        let this: Box<People> = Box::new(this);
        let this: *mut People = Box::into_raw(this);
        this as jlong
    }
    fn unbox_object(x: jlong) -> Self {
        let x: *mut People = unsafe { jlong_to_pointer::<People>(x).as_mut().unwrap() };
        let x: Box<People> = unsafe { Box::from_raw(x) };
        let x: People = *x;
        x
    }
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
        let x: *mut People = unsafe { jlong_to_pointer::<People>(x).as_mut().unwrap() };
        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
    }
}
impl<'a> SwigFrom<&'a str> for jstring {
    fn swig_from(x: &'a str, env: *mut JNIEnv) -> Self {
        let x = ::std::ffi::CString::new(x).unwrap();
        unsafe { (**env).NewStringUTF.unwrap()(env, x.as_ptr()) }
    }
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_rust_app_swapiclient_swapi_People_do_1getName(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> jstring {
    let this: &People = unsafe { jlong_to_pointer::<People>(this).as_mut().unwrap() };
    let mut ret: &str = { &this.name };
    let mut ret: jstring = <jstring>::swig_from(ret, env);
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_rust_app_swapiclient_swapi_People_do_1getGender(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> jstring {
    let this: &People = unsafe { jlong_to_pointer::<People>(this).as_mut().unwrap() };
    let mut ret: &str = { &this.gender };
    let mut ret: jstring = <jstring>::swig_from(ret, env);
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_rust_app_swapiclient_swapi_People_do_1getMass(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> jstring {
    let this: &People = unsafe { jlong_to_pointer::<People>(this).as_mut().unwrap() };
    let mut ret: &str = { &this.mass };
    let mut ret: jstring = <jstring>::swig_from(ret, env);
    ret
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_rust_app_swapiclient_swapi_People_do_1delete(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) {
    let this: *mut People = unsafe { jlong_to_pointer::<People>(this).as_mut().unwrap() };
    let this: Box<People> = unsafe { Box::from_raw(this) };
    drop(this);
}
impl SwapiCallback for JavaCallback {
    #[allow(unused_mut)]
    fn onLoad(&mut self, a0: Vec<People>) {
        swig_assert_eq_size!(::std::os::raw::c_uint, u32);
        swig_assert_eq_size!(::std::os::raw::c_int, i32);
        let env = self.get_jni_env();
        if let Some(env) = env.env {
            let mut a0: internal_aliases::JForeignObjectsArray<People> =
                vec_of_objects_to_jobject_array(env, a0);
            unsafe {
                (**env).CallVoidMethod.unwrap()(env, self.this, self.methods[0usize], a0);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    log::error!(concat!(stringify!(onLoad), ": java throw exception"));
                    (**env).ExceptionDescribe.unwrap()(env);
                    (**env).ExceptionClear.unwrap()(env);
                }
            };
        }
    }
    #[allow(unused_mut)]
    fn onError(&mut self, a0: &str) {
        swig_assert_eq_size!(::std::os::raw::c_uint, u32);
        swig_assert_eq_size!(::std::os::raw::c_int, i32);
        let env = self.get_jni_env();
        if let Some(env) = env.env {
            let mut a0: jstring = <jstring>::swig_from(a0, env);
            unsafe {
                (**env).CallVoidMethod.unwrap()(env, self.this, self.methods[1usize], a0);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    log::error!(concat!(stringify!(onError), ": java throw exception"));
                    (**env).ExceptionDescribe.unwrap()(env);
                    (**env).ExceptionClear.unwrap()(env);
                }
            };
        }
    }
}
impl SwigForeignClass for SwapiClient {
    type PointedType = SwapiClient;
    fn jni_class() -> jclass {
        swig_jni_find_class!(
            FOREIGN_CLASS_SWAPICLIENT,
            "com/rust/app/swapiclient/swapi/SwapiClient"
        )
    }
    fn jni_class_pointer_field() -> jfieldID {
        swig_jni_get_field_id!(
            FOREIGN_CLASS_SWAPICLIENT_MNATIVEOBJ_FIELD,
            FOREIGN_CLASS_SWAPICLIENT,
            "mNativeObj",
            "J"
        )
    }
    fn box_object(this: Self) -> jlong {
        let this: Box<SwapiClient> = Box::new(this);
        let this: *mut SwapiClient = Box::into_raw(this);
        this as jlong
    }
    fn unbox_object(x: jlong) -> Self {
        let x: *mut SwapiClient = unsafe { jlong_to_pointer::<SwapiClient>(x).as_mut().unwrap() };
        let x: Box<SwapiClient> = unsafe { Box::from_raw(x) };
        let x: SwapiClient = *x;
        x
    }
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
        let x: *mut SwapiClient = unsafe { jlong_to_pointer::<SwapiClient>(x).as_mut().unwrap() };
        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
    }
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_rust_app_swapiclient_swapi_SwapiClient_init(
    env: *mut JNIEnv,
    _: jclass,
) -> jlong {
    let this: SwapiClient = SwapiClient::new();
    let this: Box<SwapiClient> = Box::new(this);
    let this: *mut SwapiClient = Box::into_raw(this);
    this as jlong
}
#[doc = ""]
impl SwigFrom<jobject> for Box<dyn SwapiCallback + Send> {
    fn swig_from(this: jobject, env: *mut JNIEnv) -> Self {
        let mut cb = JavaCallback::new(this, env);
        cb.methods.reserve(2);
        let class = unsafe { (**env).GetObjectClass.unwrap()(env, cb.this) };
        assert!(
            !class.is_null(),
            "GetObjectClass return null class for SwapiPeopleLoadedListener"
        );
        let method_id: jmethodID = unsafe {
            (**env).GetMethodID.unwrap()(
                env,
                class,
                swig_c_str!("onLoaded"),
                swig_c_str!("([Lcom/rust/app/swapiclient/swapi/People;)V"),
            )
        };
        assert!(!method_id.is_null(), "Can not find onLoaded id");
        cb.methods.push(method_id);
        let method_id: jmethodID = unsafe {
            (**env).GetMethodID.unwrap()(
                env,
                class,
                swig_c_str!("onError"),
                swig_c_str!("(Ljava/lang/String;)V"),
            )
        };
        assert!(!method_id.is_null(), "Can not find onError id");
        cb.methods.push(method_id);
        Box::new(cb)
    }
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_rust_app_swapiclient_swapi_SwapiClient_do_1loadAllPeople(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
    callback: jobject,
) -> () {
    let mut callback: Box<dyn SwapiCallback + Send> =
        <Box<dyn SwapiCallback + Send>>::swig_from(callback, env);
    let this: &mut SwapiClient = unsafe { jlong_to_pointer::<SwapiClient>(this).as_mut().unwrap() };
    let mut ret: () = SwapiClient::loadAllPeople(this, callback);
    ret
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_rust_app_swapiclient_swapi_SwapiClient_do_1delete(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) {
    let this: *mut SwapiClient = unsafe { jlong_to_pointer::<SwapiClient>(this).as_mut().unwrap() };
    let this: Box<SwapiClient> = unsafe { Box::from_raw(this) };
    drop(this);
}
static mut JAVA_LANG_SHORT: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_SHORT_SHORT_VALUE: jmethodID = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_LONG: jclass = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_LONG_OF: jmethodID = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_LONG_EMPTY: jmethodID = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_SWAPICLIENT: jclass = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_SWAPICLIENT_MNATIVEOBJ_FIELD: jfieldID = ::std::ptr::null_mut();
static mut JAVA_LANG_EXCEPTION: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_FLOAT: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_FLOAT_FLOAT_VALUE: jmethodID = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_PEOPLE: jclass = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_PEOPLE_MNATIVEOBJ_FIELD: jfieldID = ::std::ptr::null_mut();
static mut JAVA_LANG_BYTE: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_BYTE_BYTE_VALUE: jmethodID = ::std::ptr::null_mut();
static mut JAVA_LANG_LONG: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_LONG_LONG_VALUE: jmethodID = ::std::ptr::null_mut();
static mut JAVA_LANG_INTEGER: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_INTEGER_INT_VALUE: jmethodID = ::std::ptr::null_mut();
static mut JAVA_LANG_DOUBLE: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_DOUBLE_DOUBLE_VALUE_METHOD: jmethodID = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_INT: jclass = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_INT_OF: jmethodID = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_INT_EMPTY: jmethodID = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_DOUBLE: jclass = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_DOUBLE_OF: jmethodID = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_DOUBLE_EMPTY: jmethodID = ::std::ptr::null_mut();
#[no_mangle]
pub extern "system" fn JNI_OnLoad(
    java_vm: *mut JavaVM,
    _reserved: *mut ::std::os::raw::c_void,
) -> jint {
    println!("JNI_OnLoad begin");
    assert!(!java_vm.is_null());
    let mut env: *mut JNIEnv = ::std::ptr::null_mut();
    let res = unsafe {
        (**java_vm).GetEnv.unwrap()(
            java_vm,
            (&mut env) as *mut *mut JNIEnv as *mut *mut ::std::os::raw::c_void,
            SWIG_JNI_VERSION,
        )
    };
    if res != (JNI_OK as jint) {
        panic!("JNI GetEnv in JNI_OnLoad failed, return code {}", res);
    }
    assert!(!env.is_null());
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Short"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Short")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Short")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_SHORT = class;
        let method_id: jmethodID =
            (**env).GetMethodID.unwrap()(env, class, swig_c_str!("shortValue"), swig_c_str!("()S"));
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Short",
                " method ",
                "shortValue",
                " sig ",
                "()S",
                " failed"
            )
        );
        JAVA_LANG_SHORT_SHORT_VALUE = method_id;
    }
    unsafe {
        let class_local_ref =
            (**env).FindClass.unwrap()(env, swig_c_str!("java/util/OptionalLong"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalLong")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalLong")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_UTIL_OPTIONAL_LONG = class;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("of"),
            swig_c_str!("(J)Ljava/util/OptionalLong;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalLong",
                " method ",
                "of",
                " sig ",
                "(J)Ljava/util/OptionalLong;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_LONG_OF = method_id;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("empty"),
            swig_c_str!("()Ljava/util/OptionalLong;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalLong",
                " method ",
                "empty",
                " sig ",
                "()Ljava/util/OptionalLong;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_LONG_EMPTY = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(
            env,
            swig_c_str!("com/rust/app/swapiclient/swapi/SwapiClient"),
        );
        assert!(
            !class_local_ref.is_null(),
            concat!(
                "FindClass failed for ",
                "com/rust/app/swapiclient/swapi/SwapiClient"
            )
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!(
                "FindClass failed for ",
                "com/rust/app/swapiclient/swapi/SwapiClient"
            )
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_CLASS_SWAPICLIENT = class;
        let field_id: jfieldID =
            (**env).GetFieldID.unwrap()(env, class, swig_c_str!("mNativeObj"), swig_c_str!("J"));
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/rust/app/swapiclient/swapi/SwapiClient",
                " method ",
                "mNativeObj",
                " sig ",
                "J",
                " failed"
            )
        );
        FOREIGN_CLASS_SWAPICLIENT_MNATIVEOBJ_FIELD = field_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Exception"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Exception")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Exception")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_EXCEPTION = class;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Float"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Float")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Float")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_FLOAT = class;
        let method_id: jmethodID =
            (**env).GetMethodID.unwrap()(env, class, swig_c_str!("floatValue"), swig_c_str!("()F"));
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Float",
                " method ",
                "floatValue",
                " sig ",
                "()F",
                " failed"
            )
        );
        JAVA_LANG_FLOAT_FLOAT_VALUE = method_id;
    }
    unsafe {
        let class_local_ref =
            (**env).FindClass.unwrap()(env, swig_c_str!("com/rust/app/swapiclient/swapi/People"));
        assert!(
            !class_local_ref.is_null(),
            concat!(
                "FindClass failed for ",
                "com/rust/app/swapiclient/swapi/People"
            )
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!(
                "FindClass failed for ",
                "com/rust/app/swapiclient/swapi/People"
            )
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_CLASS_PEOPLE = class;
        let field_id: jfieldID =
            (**env).GetFieldID.unwrap()(env, class, swig_c_str!("mNativeObj"), swig_c_str!("J"));
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/rust/app/swapiclient/swapi/People",
                " method ",
                "mNativeObj",
                " sig ",
                "J",
                " failed"
            )
        );
        FOREIGN_CLASS_PEOPLE_MNATIVEOBJ_FIELD = field_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Byte"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Byte")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Byte")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_BYTE = class;
        let method_id: jmethodID =
            (**env).GetMethodID.unwrap()(env, class, swig_c_str!("byteValue"), swig_c_str!("()B"));
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Byte",
                " method ",
                "byteValue",
                " sig ",
                "()B",
                " failed"
            )
        );
        JAVA_LANG_BYTE_BYTE_VALUE = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Long"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Long")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Long")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_LONG = class;
        let method_id: jmethodID =
            (**env).GetMethodID.unwrap()(env, class, swig_c_str!("longValue"), swig_c_str!("()J"));
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Long",
                " method ",
                "longValue",
                " sig ",
                "()J",
                " failed"
            )
        );
        JAVA_LANG_LONG_LONG_VALUE = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Integer"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Integer")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Integer")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_INTEGER = class;
        let method_id: jmethodID =
            (**env).GetMethodID.unwrap()(env, class, swig_c_str!("intValue"), swig_c_str!("()I"));
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Integer",
                " method ",
                "intValue",
                " sig ",
                "()I",
                " failed"
            )
        );
        JAVA_LANG_INTEGER_INT_VALUE = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Double"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Double")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Double")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_DOUBLE = class;
        let method_id: jmethodID = (**env).GetMethodID.unwrap()(
            env,
            class,
            swig_c_str!("doubleValue"),
            swig_c_str!("()D"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Double",
                " method ",
                "doubleValue",
                " sig ",
                "()D",
                " failed"
            )
        );
        JAVA_LANG_DOUBLE_DOUBLE_VALUE_METHOD = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/util/OptionalInt"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalInt")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalInt")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_UTIL_OPTIONAL_INT = class;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("of"),
            swig_c_str!("(I)Ljava/util/OptionalInt;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalInt",
                " method ",
                "of",
                " sig ",
                "(I)Ljava/util/OptionalInt;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_INT_OF = method_id;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("empty"),
            swig_c_str!("()Ljava/util/OptionalInt;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalInt",
                " method ",
                "empty",
                " sig ",
                "()Ljava/util/OptionalInt;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_INT_EMPTY = method_id;
    }
    unsafe {
        let class_local_ref =
            (**env).FindClass.unwrap()(env, swig_c_str!("java/util/OptionalDouble"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalDouble")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalDouble")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_UTIL_OPTIONAL_DOUBLE = class;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("of"),
            swig_c_str!("(D)Ljava/util/OptionalDouble;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalDouble",
                " method ",
                "of",
                " sig ",
                "(D)Ljava/util/OptionalDouble;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_DOUBLE_OF = method_id;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("empty"),
            swig_c_str!("()Ljava/util/OptionalDouble;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalDouble",
                " method ",
                "empty",
                " sig ",
                "()Ljava/util/OptionalDouble;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_DOUBLE_EMPTY = method_id;
    }
    SWIG_JNI_VERSION
}
#[no_mangle]
pub extern "system" fn JNI_OnUnload(java_vm: *mut JavaVM, _reserved: *mut ::std::os::raw::c_void) {
    println!("JNI_OnUnLoad begin");
    assert!(!java_vm.is_null());
    let mut env: *mut JNIEnv = ::std::ptr::null_mut();
    let res = unsafe {
        (**java_vm).GetEnv.unwrap()(
            java_vm,
            (&mut env) as *mut *mut JNIEnv as *mut *mut ::std::os::raw::c_void,
            SWIG_JNI_VERSION,
        )
    };
    if res != (JNI_OK as jint) {
        panic!("JNI GetEnv in JNI_OnLoad failed, return code {}", res);
    }
    assert!(!env.is_null());
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_SHORT);
        JAVA_LANG_SHORT = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_UTIL_OPTIONAL_LONG);
        JAVA_UTIL_OPTIONAL_LONG = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_CLASS_SWAPICLIENT);
        FOREIGN_CLASS_SWAPICLIENT = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_EXCEPTION);
        JAVA_LANG_EXCEPTION = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_FLOAT);
        JAVA_LANG_FLOAT = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_CLASS_PEOPLE);
        FOREIGN_CLASS_PEOPLE = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_BYTE);
        JAVA_LANG_BYTE = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_LONG);
        JAVA_LANG_LONG = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_INTEGER);
        JAVA_LANG_INTEGER = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_DOUBLE);
        JAVA_LANG_DOUBLE = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_UTIL_OPTIONAL_INT);
        JAVA_UTIL_OPTIONAL_INT = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_UTIL_OPTIONAL_DOUBLE);
        JAVA_UTIL_OPTIONAL_DOUBLE = ::std::ptr::null_mut()
    }
}

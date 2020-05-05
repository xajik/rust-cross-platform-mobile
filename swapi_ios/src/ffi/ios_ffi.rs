use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use super::native_model::{PeopleNative, PeopleNativeWrapper};
use swapi_core::swapi::{SwapiClient};
use std::ops::Deref;
use super::callback::{Callback};

//Create client
#[no_mangle]
pub extern "C" fn create_swapi_client() -> *mut SwapiClient {
    Box::into_raw(Box::new(SwapiClient::new()))
}

//Release memory
#[no_mangle]
pub unsafe extern "C" fn free_swapi_client(client: *mut SwapiClient) {
    assert!(!client.is_null());
    Box::from_raw(client);
}

//you need reference to owner context to return data
#[allow(non_snake_case)]
#[repr(C)]
pub struct PeopleCallback {
    owner: *mut c_void,
    onResult: extern fn(owner: *mut c_void, arg: *const PeopleNativeWrapper),
    onError: extern fn(owner: *mut c_void, arg: *const c_char),
}

impl Copy for PeopleCallback {}

impl Clone for PeopleCallback {

    fn clone(&self) -> Self {
        *self
    }

}

unsafe impl Send for PeopleCallback {}

impl Deref for PeopleCallback {
    type Target = PeopleCallback;

    fn deref(&self) -> &PeopleCallback {
        &self
    }
}

#[no_mangle]
pub unsafe extern "C" fn load_all_people(client: *mut SwapiClient, outer_listener: PeopleCallback) {
    assert!(!client.is_null());

    let local_client = client.as_ref().unwrap();
    let cb = Callback {
        result: Box::new(move |result| {
            let mut native_vec: Vec<PeopleNative> = Vec::new();
            for p in result {
                let native_people = PeopleNative {
                    name: CString::new(p.name).unwrap().into_raw(),
                    gender: CString::new(p.gender).unwrap().into_raw(),
                    mass: CString::new(p.mass).unwrap().into_raw(),
                };
                native_vec.push(native_people);
            }

            let ptr = PeopleNativeWrapper {
                array: native_vec.as_mut_ptr(),
                length: native_vec.len() as _,
            };

            (outer_listener.onResult)(outer_listener.owner, &ptr);
        }),
        error: Box::new(move |error| {
            let error_message = CString::new(error.to_owned()).unwrap().into_raw();
            (outer_listener.onError)(outer_listener.owner, error_message);
        }),
    };
    let callback = Box::new(cb);
    local_client.loadAllPeople(callback);
}

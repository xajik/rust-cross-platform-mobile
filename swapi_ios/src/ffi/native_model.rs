use std::os::raw::{c_char};

//DTO
#[repr(C)]
pub struct PeopleNativeWrapper {
    pub(crate) array: *mut PeopleNative,
    pub(crate) length: usize,
}

#[repr(C)]
pub struct PeopleNative {
    pub name: *const c_char,
    pub gender: *const c_char,
    pub mass: *const c_char,
}
use std::string::ToString;
use swapi_core::swapi::{People, SwapiCallback};

//Local callback for loading
pub struct Callback {
    pub result: Box<dyn FnMut(Vec<People>)>,
    pub error: Box<dyn FnMut(String)>,
}

#[allow(non_snake_case)]
impl SwapiCallback for Callback {
    fn onLoad(&mut self, s: Vec<People>) {
        (self.result)(s);
    }

    fn onError(&mut self, s: &str) {
        (self.error)(s.to_string());
    }
}

unsafe impl Send for Callback {}
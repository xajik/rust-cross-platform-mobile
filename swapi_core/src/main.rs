//swapi
pub mod swapi;

//used in swapi client
#[macro_use]
extern crate lazy_static;

//measure time
use std::time::Instant;
//DTO
use swapi::{SwapiCallback, People};
//for thread barrier
use std::sync::{Arc, Mutex, Condvar};

fn main() {
    println!("Main started");
    //barrier
    let con_var = Arc::new((Mutex::new(false), Condvar::new()));
    //Will be used in another thread
    let con_var_send = con_var.clone();

    //Callback that will nlock thread
    let unlock = move || {
        let (lock, cvar) = &*con_var_send;
        let mut started = lock.lock().unwrap();
        *started = true;
        // We notify the condvar that the value has changed.
        cvar.notify_one();
    };

    //call swapi client
    let client = swapi::SwapiClient::new();
    client.loadAllPeople(Box::new(Callback::new(Box::new(unlock))));

    //wait for thread to finish
    // Wait for the thread to start up.
    let (lock, cvar) = &*con_var;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    println!("Main finished")
}

//Create callback
struct Callback {
    start: Instant,
    unlock: Box<dyn FnMut()>,
}

impl Callback {
    fn new(unlock: Box<dyn FnMut()>) -> Callback {
        Callback {
            start: Instant::now(),
            unlock,
        }
    }
}

//Send - types that can be transferred across thread boundaries.
unsafe impl Send for Callback {}

//require to share it between threads
impl SwapiCallback for Callback {
    #[allow(non_snake_case)]
    fn onLoad(&mut self, s: Vec<People>) {
        let diff = self.start.elapsed().as_millis();
        println!("Request: count {}; duration: {}", s.len(), diff);
        //notify lock that thread finished work
        (self.unlock)();
    }

    #[allow(non_snake_case)]
    fn onError(&mut self, s: &str) {
        println!("Error: {:#?}", s);
        //notify lock that thread finished work
        (self.unlock)();
    }
}
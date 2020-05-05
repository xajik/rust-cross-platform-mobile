use log::info;

#[allow(non_snake_case)]
pub fn initLogger() {
        android_logger::init_once(
        android_logger::Config::default()
            .with_min_level(log::Level::Debug)
            .with_tag("RustLogger"),
    );
    log_panics::init(); // log panics rather than printing them
    info!("Logger initialized");
}
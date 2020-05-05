use std::env;

fn main() {
    print_target_os();
}

fn print_target_os() {
    let target_os = env::var("CARGO_CFG_TARGET_OS");
    match target_os.as_ref().map(|x| &**x) {
        Ok(tos) => println!("cargo:warning=target os {:?}", tos),
        Err(err)=> println!("cargo:warning=os not defined {:?}!", err)
    }
    let target_os = env::var("CARGO_CFG_TARGET_ARCH");
    match target_os.as_ref().map(|x| &**x) {
        Ok(tos) => println!("cargo:warning=target arch {:?}", tos),
        Err(err)=> println!("cargo:warning=arch not defined {:?}!", err)
    }
}
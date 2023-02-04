use ni_rs::detect::detect_package_manager;
use std::{env, error::Error, process::Command};
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let manager = detect_package_manager();
    let mut install = Command::new(manager);
    install.arg("run").args(&args[1..]).status()?;
    Ok(())
}

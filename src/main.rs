use ni_rs::detect::detect_package_manager;
use std::{error::Error, process::Command};
fn main() -> Result<(), Box<dyn Error>> {
    let manager = detect_package_manager();
    let mut install = Command::new(manager);
    install.arg("install").status()?;
    Ok(())
}

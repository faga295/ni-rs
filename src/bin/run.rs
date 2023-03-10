use ni_rs::utils::{detect::detect_package_manager, print::print_command};
use std::{env, error::Error, process::Command};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let manager = detect_package_manager().await;
    print_command(&manager, "run", &args);
    let mut install = Command::new(manager);
    install.arg("run").args(&args[1..]).status()?;
    Ok(())
}

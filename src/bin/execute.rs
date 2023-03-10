use std::{env, error::Error, process::Command};

use ni_rs::utils::{detect::detect_package_manager, print::print_command};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = detect_package_manager().await;
    let args: Vec<String> = env::args().collect();
    if manager.eq("npm") {
        print_command("npx", "", &args);
        let mut excute = Command::new("npx");
        excute.args(&args[1..]).status()?;
    } else {
        print_command(&manager, "dlx", &args);
        let mut excute = Command::new(manager);
        excute.arg("dlx").args(&args[1..]).status()?;
    }
    Ok(())
}

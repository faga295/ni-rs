use ni_rs::utils::args::resolve_args;
use ni_rs::utils::detect::detect_package_manager;
use ni_rs::utils::print::print_command;
use std::{env, error::Error, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect();
    let has_package = resolve_args(&args);

    let manager = detect_package_manager();
    let mut install_arg = "install";
    if has_package && (manager.eq("yarn") || manager.eq("pnpm")) {
        install_arg = "add"
    }
    print_command(&manager, install_arg, &args);
    let mut install = Command::new(manager);
    install.arg(install_arg).args(&args[1..]).status()?;
    Ok(())
}

use ni_rs::utils::args::resolve_args;
use ni_rs::utils::detect::detect_package_manager;
use std::{env, error::Error, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect();
    let has_package = resolve_args(&args);

    let manager = detect_package_manager();
    let mut install_arg = "install";
    if has_package && (manager.eq("yarn") || manager.eq("pnpm")) {
        install_arg = "add"
    }
    let mut args_str = String::new();
    for arg in &args[1..] {
        args_str += " ";
        args_str += arg;
    }
    println!("> {} {}{}", manager, install_arg, args_str);
    let mut install = Command::new(manager);
    install.arg(install_arg).args(&args[1..]).status()?;
    Ok(())
}

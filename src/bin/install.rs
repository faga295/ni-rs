use clap::Parser;
use ni_rs::utils::{
    config::{get_global_agent, resolve_config},
    detect::detect_package_manager,
};
use std::{error::Error, process::Command};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    dep: Vec<String>,

    // add dependency to devDependencies
    #[arg(short = 'D', long)]
    save_dev: bool,

    // add dependency globally
    #[arg(short = 'g', long)]
    global: bool,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut manager = detect_package_manager().await;
    if cli.global {
        manager = String::from("npm")
    }
    let mut install_arg = "install";
    if cli.dep.len() != 0 && (manager.eq("yarn") || manager.eq("pnpm")) {
        install_arg = "add"
    }
    if cli.global {
        manager = get_global_agent();
    }
    let mut install = Command::new(&manager);
    if manager.eq("yarn") {
        install.args(&cli.dep).status()?;
    } else {
        install.arg(install_arg).args(&cli.dep).status()?;
    }
    Ok(())
}

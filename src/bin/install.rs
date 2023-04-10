use clap::Parser;
use ni_rs::utils::{config::get_global_agent, detect::detect_package_manager};
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
    let manager;
    if cli.global {
        manager = get_global_agent();
    } else {
        manager = detect_package_manager().await;
    }
    let mut install_arg = "install";
    if cli.dep.len() != 0 && (manager.eq("yarn") || manager.eq("pnpm")) {
        install_arg = "add"
    }

    let mut install = Command::new(&manager);
    let mut runner: &mut Command = &mut install;
    if manager.eq("yarn") {
        if cli.global {
            runner = runner.arg("global");
        }
        if cli.dep.len() != 0 {
            runner = runner.arg(install_arg);
        }
        runner = runner.args(&cli.dep);
    } else {
        runner.arg(install_arg).args(&cli.dep);
        if cli.global {
            runner.arg("-g");
        }
    }
    runner.status()?;
    Ok(())
}

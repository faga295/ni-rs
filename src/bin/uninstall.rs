use ni_rs::utils::{detect::detect_package_manager, print::print_command};
use std::{env, error::Error, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().collect();
    let manager = detect_package_manager();
    let global = has_global(&mut args, &manager);
    println!("{:?}", args);

    let mut uninstall_arg = "remove";
    if manager.eq("npm") {
        uninstall_arg = "uninstall"
    }
    if global && manager.eq("yarn") {
        uninstall_arg = "global";
        args.insert(1, String::from("remove"));
    }
    print_command(&manager, uninstall_arg, &args);
    let mut uninstall = Command::new(manager);
    uninstall.arg(uninstall_arg).args(&args[1..]).status()?;
    Ok(())
}

fn has_global(args: &mut Vec<String>, manager: &String) -> bool {
    let mut global = false;
    for (index, arg) in args.clone().iter().enumerate() {
        if arg.eq("-g") {
            global = true;
            if manager.eq("yarn") {
                args.remove(index);
            }
        }
    }
    global
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_global_arg() {
        let mut args = vec!["nun".to_string(), "-g".to_string(), "webpack".to_string()];
        assert_eq!(has_global(&mut args, &String::from("yarn")), true);
    }
}

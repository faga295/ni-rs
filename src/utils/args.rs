pub fn resolve_args(args: &Vec<String>) -> bool {
    for arg in args {
        if !arg.starts_with('-') {
            return true;
        }
    }
    false
}
pub fn args_to_string(args: &[String]) -> String {
    let mut args_str = String::new();
    for arg in args {
        args_str += " ";
        args_str += arg;
    }
    args_str
}

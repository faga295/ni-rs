pub fn resolve_args(args: &Vec<String>) -> bool {
    for arg in args {
        if !arg.starts_with('-') {
            return true;
        }
    }
    false
}
